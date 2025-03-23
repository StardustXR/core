use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use split_iter::Splittable;
use stardust_xr_schemas::protocol::*;

#[proc_macro]
pub fn codegen_root_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ROOT_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_node_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(NODE_PROTOCOL, false, true)
}
#[proc_macro]
pub fn codegen_spatial_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(SPATIAL_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_field_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(FIELD_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_audio_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(AUDIO_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_drawable_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(DRAWABLE_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_input_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(INPUT_PROTOCOL, true, false)
}
#[proc_macro]
pub fn codegen_item_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ITEM_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_item_camera_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ITEM_CAMERA_PROTOCOL, true, true)
}
#[proc_macro]
pub fn codegen_item_panel_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ITEM_PANEL_PROTOCOL, true, true)
}

trait Tokenize {
	fn tokenize(&self, generate_node: bool, partial_eq: bool) -> TokenStream;
}

fn codegen_client_protocol(
	protocol: &'static str,
	generate_node: bool,
	partial_eq: bool,
) -> proc_macro::TokenStream {
	Protocol::parse(protocol)
		.unwrap()
		.tokenize(generate_node, partial_eq)
		.into()
}
impl Tokenize for Protocol {
	fn tokenize(&self, generate_node: bool, partial_eq: bool) -> TokenStream {
		let protocol_version = self.version;
		let protocol_version = quote!(pub(crate) const INTERFACE_VERSION: u32 = #protocol_version;);
		let interface_node_id = self
			.interface
			.as_ref()
			.map(|i| {
				let id = i.node_id;
				quote!(pub(crate) const INTERFACE_NODE_ID: u64 = #id;)
			})
			.unwrap_or_default();
		let custom_enums = self
			.custom_enums
			.iter()
			.map(|e| e.tokenize(generate_node, partial_eq));
		let custom_unions = self
			.custom_unions
			.iter()
			.map(|u| u.tokenize(generate_node, partial_eq));
		let custom_structs = self
			.custom_structs
			.iter()
			.map(|s| s.tokenize(generate_node, partial_eq));
		let aspects = self
			.aspects
			.iter()
			.map(|a| a.tokenize(generate_node, partial_eq));
		let interface = self
			.interface
			.as_ref()
			.map(|p| {
				p.members
					.iter()
					.map(|m| {
						let member_name = m.name.to_case(Case::ScreamingSnake);
						let name_type = if m.side == Side::Client {
							"CLIENT"
						} else {
							"SERVER"
						};
						let name = Ident::new(
							&format!("INTERFACE_{member_name}_{name_type}_OPCODE"),
							Span::call_site(),
						);
						let opcode = m.opcode;

						let member = generate_server_member(Some(p.node_id), 0, m);
						quote! {
							pub(crate) const #name: u64 = #opcode;
							#member
						}
					})
					.reduce(|a, b| quote!(#a #b))
					.unwrap_or_default()
			})
			.unwrap_or_default();
		quote!(
			#protocol_version
			#interface_node_id
			#(#custom_enums)*
			#(#custom_unions)*
			#(#custom_structs)*
			#(#aspects)*
			#interface
		)
	}
}
impl Tokenize for CustomEnum {
	fn tokenize(&self, _generate_node: bool, partial_eq: bool) -> TokenStream {
		let name = Ident::new(&self.name.to_case(Case::Pascal), Span::call_site());
		let description = &self.description;

		let argument_decls = self
			.variants
			.iter()
			.map(|a| Ident::new(&a.to_case(Case::Pascal), Span::call_site()).to_token_stream());

		let derive = if partial_eq {
			quote!( #[derive(Debug, Clone, Copy, Hash, PartialEq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)] )
		} else {
			quote!( #[derive(Debug, Clone, Copy, Hash, serde::Deserialize, serde::Serialize)] )
		};
		quote! {
			#[doc = #description]
			#derive
			#[repr(u32)]
			pub enum #name {
				#(#argument_decls),*
			}
		}
	}
}
impl Tokenize for CustomUnion {
	fn tokenize(&self, _generate_node: bool, partial_eq: bool) -> TokenStream {
		let name = Ident::new(&self.name.to_case(Case::Pascal), Span::call_site());
		let description = &self.description;

		let option_decls = self
			.options
			.iter()
			.map(|e| e.tokenize(_generate_node, partial_eq));

		let derive = if partial_eq {
			quote!( #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)] )
		} else {
			quote!( #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)] )
		};
		quote! {
			#[doc = #description]
			#derive
			#[serde(tag = "t", content = "c")]
			pub enum #name {
				#(#option_decls),*
			}
		}
	}
}
impl Tokenize for UnionOption {
	fn tokenize(&self, _generate_node: bool, _partial_eq: bool) -> TokenStream {
		let name = self
			.name
			.as_ref()
			.map(|n| n.to_case(Case::Pascal))
			.unwrap_or_else(|| argument_type_option_name(&self._type));
		let description = self
			.description
			.as_ref()
			.map(|d| quote!(#[doc = #d]))
			.unwrap_or_default();
		let identifier = Ident::new(&name, Span::call_site());
		let _type = generate_argument_type(&self._type, true);
		quote! (#description #identifier(#_type))
	}
}
fn argument_type_option_name(argument_type: &ArgumentType) -> String {
	match argument_type {
		ArgumentType::Empty => "Empty".to_string(),
		ArgumentType::Bool => "Bool".to_string(),
		ArgumentType::Int => "Int".to_string(),
		ArgumentType::UInt => "UInt".to_string(),
		ArgumentType::Float => "Float".to_string(),
		ArgumentType::Vec2(_) => "Vec2".to_string(),
		ArgumentType::Vec3(_) => "Vec3".to_string(),
		ArgumentType::Quat => "Quat".to_string(),
		ArgumentType::Mat4 => "Mat4".to_string(),
		ArgumentType::Color => "Color".to_string(),
		ArgumentType::String => "String".to_string(),
		ArgumentType::Bytes => "Bytes".to_string(),
		ArgumentType::Vec(v) => format!("{}Vector", argument_type_option_name(v)),
		ArgumentType::Map(m) => format!("{}Map", argument_type_option_name(m)),
		ArgumentType::NodeID => "Node ID".to_string(),
		ArgumentType::Datamap => "Datamap".to_string(),
		ArgumentType::ResourceID => "ResourceID".to_string(),
		ArgumentType::Enum(e) => e.clone(),
		ArgumentType::Union(u) => u.clone(),
		ArgumentType::Struct(s) => s.clone(),
		ArgumentType::Node { _type, .. } => _type.clone(),
		ArgumentType::Fd => "File Descriptor".to_string(),
	}
}

impl Tokenize for CustomStruct {
	fn tokenize(&self, _generate_node: bool, partial_eq: bool) -> TokenStream {
		let name = Ident::new(&self.name.to_case(Case::Pascal), Span::call_site());
		let description = &self.description;

		let argument_decls = self
			.fields
			.iter()
			.map(|a| generate_argument_decl(a, true))
			.map(|d| quote!(pub #d));

		let derive = if partial_eq {
			quote!( #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)] )
		} else {
			quote!( #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)] )
		};
		quote! {
			#[doc = #description]
			#derive
			pub struct #name {
				#(#argument_decls),*
			}
		}
	}
}

impl Tokenize for Aspect {
	fn tokenize(&self, generate_node: bool, _partial_eq: bool) -> TokenStream {
		let node_name = Ident::new(&self.name, Span::call_site());
		let description = &self.description;
		let opcode = self.id;
		let server_members = self.members.iter().filter(|m| m.side == Side::Server);

		let aspect_id = {
			let name = Ident::new(
				&format!("{}_ASPECT_ID", self.name.to_case(Case::ScreamingSnake)),
				Span::call_site(),
			);
			quote!(pub(crate) const #name: u64 = #opcode;)
		};

		let aspect_trait_name = Ident::new(
			&format!("{}Aspect", &self.name.to_case(Case::Pascal)),
			Span::call_site(),
		);

		let opcodes = self.members.iter().map(|m| {
			let aspect_name = self.name.to_case(Case::ScreamingSnake);
			let member_name = m.name.to_case(Case::ScreamingSnake);
			let name_type = if m.side == Side::Client {
				"CLIENT"
			} else {
				"SERVER"
			};
			let name = Ident::new(
				&format!("{aspect_name}_{member_name}_{name_type}_OPCODE"),
				Span::call_site(),
			);
			let opcode = m.opcode;

			quote!(pub(crate) const #name: u64 = #opcode;)
		});

		let aspect_name = Ident::new(&self.name.to_case(Case::Pascal), Span::call_site());
		let event_name = Ident::new(&format!("{}Event", aspect_name), Span::call_site());
		let aspect_events = {
			let variants = self
				.members
				.iter()
				.filter(|m| m.side == Side::Client)
				.map(|m| {
					let variant_name = Ident::new(&m.name.to_case(Case::Pascal), Span::call_site());
					let fields = m.arguments.iter().map(|a| generate_argument_decl(a, true));
					let response = if let Some(response_type) = &m.return_type {
						let response_type = generate_argument_type(response_type, false);
						quote!(response: crate::TypedMethodResponse<#response_type>)
					} else {
						quote!()
					};
					quote! {
						#variant_name { #(#fields),* #response }
					}
				});

			quote! {
				#[derive(Debug)]
				pub enum #event_name {
					#(#variants),*
				}
			}
		};
		let aspect_event_sender_impl = if self.members.iter().any(|m| m.side == Side::Client) {
			generate_event_sender_impl(self)
		} else {
			quote!()
		};
		let inherit_types = self
			.inherits
			.iter()
			.map(|m| Ident::new(&format!("{m}Aspect"), Span::call_site()))
			.fold(quote!(crate::node::NodeType), |a, b| quote!(#a + #b));
		let server_side_members = server_members.map(|m| generate_server_member(None, self.id, m));
		let do_add_aspect = self.members.iter().any(|m| m.side == Side::Client);
		let add_aspect_tokens = do_add_aspect
			.then_some(quote!(client.scenegraph.add_aspect::<#event_name>(&node);))
			.unwrap_or_default();

		let conversion_functions = self
			.inherits
			.iter()
			.filter(|i| i.to_case(Case::Snake) != "owned")
			.map(|i| {
				let inherited_aspect = Ident::new(&i.to_case(Case::UpperCamel), Span::call_site());
				let conversion_fn_name =
					Ident::new(&format!("as_{}", i.to_case(Case::Snake)), Span::call_site());

				quote! {
					pub fn #conversion_fn_name(self) -> #inherited_aspect {
						#inherited_aspect(self.0)
					}
				}
			});
		let node = generate_node.then_some(quote! {
			#[allow(clippy::all)]
			#[doc = #description]
			#[derive(Debug, Clone, Hash, PartialEq, Eq)]
			pub struct #node_name (pub(crate) std::sync::Arc<crate::node::Node>);
			#[allow(clippy::all)]
			impl #node_name {
				pub(crate) fn from_id(client: &std::sync::Arc<crate::client::ClientHandle>, id: u64, owned: bool) -> Self {
					let node = crate::node::Node::from_id(client, id, owned);
					#add_aspect_tokens
					#node_name(node)
				}
				#(#conversion_functions)*
			}
			#[allow(clippy::all)]
			impl crate::node::NodeType for #node_name {
				fn node(&self) -> &crate::node::Node {
					&self.0
				}
			}
			#[allow(clippy::all)]
			impl serde::Serialize for #node_name {
				fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
					serializer.serialize_u64(self.0.id())
				}
			}
			#[allow(clippy::all)]
			impl #aspect_trait_name for #node_name {}
		});

		let events_method = self
			.members
			.iter()
			.any(|m| m.side == Side::Client)
			.then(|| {
				let recv_event_method_name = Ident::new(
					&format!("recv_{}_event", self.name.to_case(Case::Snake)),
					Span::call_site(),
				);
				let id = self.id;
				quote! {
					fn #recv_event_method_name(&self) -> Option<#event_name> {
						self.node().recv_event(#id)
					}
				}
			})
			.unwrap_or_default();
		quote! {
			#node
			#aspect_id
			#(#opcodes)*

			#aspect_events
			#aspect_event_sender_impl

			#[allow(clippy::all)]
			#[doc = #description]
			pub trait #aspect_trait_name: #inherit_types {
				#events_method
				#(#server_side_members)*
			}
		}
	}
}

fn generate_event_sender_impl(aspect: &Aspect) -> TokenStream {
	let aspect_name = Ident::new(&aspect.name.to_case(Case::Pascal), Span::call_site());
	let event_name = Ident::new(&format!("{}Event", aspect_name), Span::call_site());
	let opcode = aspect.id;

	let (signal_matches, method_matches) = aspect
		.members
		.iter()
		.filter(|m| m.side == Side::Client)
		.map(|m| {
			let opcode = m.opcode;
			let variant_name = Ident::new(&m.name.to_case(Case::Pascal), Span::call_site());
			let field_names = m
				.arguments
				.iter()
				.map(|a| Ident::new(&a.name.to_case(Case::Snake), Span::call_site()));
			let deserialize_types = m
				.arguments
				.iter()
				.map(|a| generate_argument_type(&convert_deserializeable_argument_type(&a._type), true));
			let field_uses: Vec<_> = m
				.arguments
				.iter()
				.map(|a| {
					let name_ident = Ident::new(&a.name.to_case(Case::Snake), Span::call_site());
					let argument_deserialize =
						generate_argument_deserialize(&a.name, &a._type, a.optional);
					quote!(#name_ident: #argument_deserialize)
				})
				.collect();

			let response_sender = if m._type == MemberType::Method {
				quote! {
					response: crate::TypedMethodResponse(response.borrow_mut().take().unwrap(), std::marker::PhantomData),
				}
			} else {
				quote!()
			};

			(
				m._type,
				quote! {
					#opcode => {
						let (#(#field_names),*): (#(#deserialize_types),*) = stardust_xr::schemas::flex::deserialize(_data)?;
						Ok(#event_name::#variant_name { #(#field_uses,)* #response_sender })
					}
				},
			)
		})
		.split(|(_ty, _)| _ty == &MemberType::Method);

	let signal_matches = signal_matches.map(|t| t.1);
	let method_matches = method_matches.map(|t| t.1);

	quote! {
		#[allow(clippy::all)]
		impl crate::scenegraph::EventParser for #event_name {
			const ASPECT_ID: u64 = #opcode;
			fn serialize_signal(_client: &std::sync::Arc<crate::client::ClientHandle>, signal_id: u64, _data: &[u8], _fds: Vec<std::os::fd::OwnedFd>) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
				match signal_id {
					#(#signal_matches)*
					_ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
				}
			}
			fn serialize_method(_client: &std::sync::Arc<crate::client::ClientHandle>, method_id: u64, _data: &[u8], _fds: Vec<std::os::fd::OwnedFd>, response: stardust_xr::scenegraph::MethodResponse) -> Option<Self> {
				let response = std::rc::Rc::new(std::cell::RefCell::new(Some(response)));
				let response2 = response.clone();
				let result = || match method_id {
					#(#method_matches)*
					_ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
				};
				match (result)() {
					Ok(event) => Some(event),
					Err(e) => {
						let _ = response2.borrow_mut().take().unwrap().send(Err(e));
						None
					}
				}
			}
		}
	}
}

fn generate_server_member(
	interface_node_id: Option<u64>,
	aspect_id: u64,
	member: &Member,
) -> TokenStream {
	let opcode = member.opcode;
	let name = Ident::new(&member.name.to_case(Case::Snake), Span::call_site());
	let description = &member.description;

	if member.side == Side::Client {
		return quote!();
	}
	let _type = member._type;

	let first_arg = if interface_node_id.is_some() {
		quote!(_client: &std::sync::Arc<crate::client::ClientHandle>)
	} else {
		quote!(&self)
	};
	let argument_decls = member
		.arguments
		.iter()
		.map(|a| generate_argument_decl(a, member.side == Side::Client))
		.fold(first_arg, |a, b| quote!(#a, #b));

	let argument_uses = member
		.arguments
		.iter()
		.map(|a| generate_argument_serialize(&a.name, &a._type, a.optional));
	let return_type = member
		.return_type
		.as_ref()
		.map(|r| generate_argument_type(r, true))
		.unwrap_or_else(|| quote!(()));

	match _type {
		MemberType::Signal => {
			let mut body = if let Some(interface_node_id) = &interface_node_id {
				quote! {{
					let mut _fds = Vec::new();
					let data = stardust_xr::schemas::flex::serialize(&(#(#argument_uses),*))?;
					_client.message_sender_handle.signal(#interface_node_id, #aspect_id, #opcode, &data, _fds)
				}}
			} else {
				quote! {{
					let mut _fds = Vec::new();
					self.node().send_remote_signal(#aspect_id, #opcode, &(#(#argument_uses),*), _fds)
				}}
			};
			body = if let Some(ArgumentType::Node {
				_type: _,
				return_id_parameter_name: Some(return_id_parameter_name),
			}) = &member.return_type
			{
				let id_argument = Ident::new(return_id_parameter_name, Span::call_site());
				let get_client = if interface_node_id.is_some() {
					quote!(_client)
				} else {
					quote!(self.node().client()?)
				};
				quote! {
					#body?;
					Ok(#return_type::from_id(&#get_client, #id_argument, true))
				}
			} else {
				quote! {
					Ok(#body?)
				}
			};
			if interface_node_id.is_some() {
				if let Some(ArgumentType::Node { .. }) = &member.return_type {
				} else {
					return quote! {
						#[allow(clippy::all)]
						#[doc = #description]
						pub fn #name(#argument_decls) -> crate::node::NodeResult<#return_type> {
							#body
						}
					};
				}
			}
			quote! {
				#[doc = #description]
				fn #name(#argument_decls) -> crate::node::NodeResult<#return_type> {
					#body
				}
			}
		}
		MemberType::Method => {
			let body = if let Some(interface_node_id) = &interface_node_id {
				let argument_type = member.return_type.clone().unwrap_or(ArgumentType::Empty);
				let deserializeable_type = generate_argument_type(
					&convert_deserializeable_argument_type(&argument_type),
					true,
				);
				let deserialize = generate_argument_deserialize("result", &argument_type, false);
				quote! {
					let mut _fds = Vec::new();
					let data = stardust_xr::schemas::flex::serialize(&(#(#argument_uses),*))?;
					let message = _client.message_sender_handle.method(#interface_node_id, #aspect_id, #opcode, &data, _fds).await?.map_err(|e| crate::node::NodeError::ReturnedError { e })?.into_message();
					let result: #deserializeable_type = stardust_xr::schemas::flex::deserialize(&message)?;
					Ok(#deserialize)
				}
			} else {
				quote! {{
					let mut _fds = Vec::new();
					self.node().execute_remote_method(#aspect_id, #opcode, &(#(#argument_uses),*), _fds).await
				}}
			};
			if interface_node_id.is_some() {
				return quote! {
					#[doc = #description]
					pub async fn #name(#argument_decls) -> crate::node::NodeResult<#return_type> {
						#body
					}
				};
			}
			quote! {
				#[doc = #description]
				async fn #name(#argument_decls) -> crate::node::NodeResult<#return_type> {
					#body
				}
			}
		}
	}
}
fn generate_argument_deserialize(
	argument_name: &str,
	argument_type: &ArgumentType,
	optional: bool,
) -> TokenStream {
	let name = Ident::new(&argument_name.to_case(Case::Snake), Span::call_site());
	match argument_type {
		ArgumentType::Node {
			_type,
			return_id_parameter_name: _,
		} => {
			let node_type = Ident::new(&_type.to_case(Case::Pascal), Span::call_site());
			match optional {
				true => {
					quote!(#name.map(|n| #node_type::from_id(&_client, n, false)))
				}
				false => {
					quote!(#node_type::from_id(&_client, #name, false))
				}
			}
		}
		ArgumentType::Color => quote!(color::rgba_linear!(#name[0], #name[1], #name[2], #name[3])),
		ArgumentType::Vec(v) => {
			let mapping = generate_argument_deserialize("a", v, false);
			quote!(#name.into_iter().map(|a| Ok(#mapping)).collect::<Result<Vec<_>, crate::node::NodeError>>()?)
		}
		ArgumentType::Map(v) => {
			let mapping = generate_argument_deserialize("a", v, false);
			quote!(#name.into_iter().map(|(k, a)| Ok((k, #mapping))).collect::<Result<rustc_hash::FxHashMap<String, _>, crate::node::NodeError>>()?)
		}
		ArgumentType::Fd => {
			quote!(_fds.remove(0))
		}
		_ => quote!(#name),
	}
}
fn convert_deserializeable_argument_type(argument_type: &ArgumentType) -> ArgumentType {
	match argument_type {
		ArgumentType::Node { .. } => ArgumentType::NodeID,
		ArgumentType::Vec(v) => {
			ArgumentType::Vec(Box::new(convert_deserializeable_argument_type(v)))
		}
		ArgumentType::Map(v) => {
			ArgumentType::Map(Box::new(convert_deserializeable_argument_type(v)))
		}
		ArgumentType::Fd => ArgumentType::UInt,
		f => f.clone(),
	}
}

fn generate_argument_serialize(
	argument_name: &str,
	argument_type: &ArgumentType,
	optional: bool,
) -> TokenStream {
	let name = Ident::new(&argument_name.to_case(Case::Snake), Span::call_site());
	if let ArgumentType::Node { _type, .. } = argument_type {
		return match optional {
			true => quote!(#name.map(|n| n.node().id())),
			false => quote!(#name.node().id()),
		};
	}
	if optional {
		let mapping = generate_argument_serialize("o", argument_type, false);
		return quote!(#name.map(|o| Ok::<_, crate::node::NodeError>(#mapping)).transpose()?);
	}

	match argument_type {
		ArgumentType::Vec2(_) => {
			quote!(#name.into())
		}
		ArgumentType::Vec3(_) => {
			quote!(#name.into())
		}
		ArgumentType::Quat => {
			quote!(#name.into())
		}
		ArgumentType::Mat4 => {
			quote!(#name.into())
		}
		ArgumentType::Color => quote!([#name.c.r, #name.c.g, #name.c.b, #name.a]),
		ArgumentType::Vec(v) => {
			let mapping = generate_argument_serialize("a", v, false);
			quote!(#name.iter().map(|a| Ok(#mapping)).collect::<crate::node::NodeResult<Vec<_>>>()?)
		}
		ArgumentType::Map(v) => {
			let mapping = generate_argument_serialize("a", v, false);
			quote!(#name.iter().map(|(k, a)| Ok((k, #mapping))).collect::<crate::node::NodeResult<rustc_hash::FxHashMap<String, _>>>()?)
		}
		ArgumentType::Fd => {
			quote!({
				_fds.push(#name);
				(_fds.len() - 1) as u32
			})
		}
		_ => quote!(#name),
	}
}
fn generate_argument_decl(argument: &Argument, returned: bool) -> TokenStream {
	let name = Ident::new(&argument.name.to_case(Case::Snake), Span::call_site());
	let mut _type = generate_argument_type(&argument._type, returned);
	if argument.optional {
		_type = quote!(Option<#_type>);
	}
	quote!(#name: #_type)
}
fn generate_argument_type(argument_type: &ArgumentType, owned: bool) -> TokenStream {
	match argument_type {
		ArgumentType::Empty => quote!(()),
		ArgumentType::Bool => quote!(bool),
		ArgumentType::Int => quote!(i32),
		ArgumentType::UInt => quote!(u32),
		ArgumentType::Float => quote!(f32),
		ArgumentType::Vec2(t) => {
			let t = generate_argument_type(t, true);
			if !owned {
				quote!(impl Into<stardust_xr::values::Vector2<#t>>)
			} else {
				quote!(stardust_xr::values::Vector2<#t>)
			}
		}
		ArgumentType::Vec3(t) => {
			let t = generate_argument_type(t, true);
			if !owned {
				quote!(impl Into<stardust_xr::values::Vector3<#t>>)
			} else {
				quote!(stardust_xr::values::Vector3<#t>)
			}
		}
		ArgumentType::Quat => {
			if !owned {
				quote!(impl Into<stardust_xr::values::Quaternion>)
			} else {
				quote!(stardust_xr::values::Quaternion)
			}
		}
		ArgumentType::Mat4 => {
			if !owned {
				quote!(impl Into<stardust_xr::values::Mat4>)
			} else {
				quote!(stardust_xr::values::Mat4)
			}
		}
		ArgumentType::Color => quote!(stardust_xr::values::Color),
		ArgumentType::Bytes => {
			if !owned {
				quote!(&[u8])
			} else {
				quote!(Vec<u8>)
			}
		}
		ArgumentType::String => {
			if !owned {
				quote!(&str)
			} else {
				quote!(String)
			}
		}
		ArgumentType::Vec(t) => {
			let t = generate_argument_type(t, true);
			if !owned {
				quote!(&[#t])
			} else {
				quote!(Vec<#t>)
			}
		}
		ArgumentType::Map(t) => {
			let t = generate_argument_type(t, true);

			if !owned {
				quote!(&stardust_xr::values::Map<String, #t>)
			} else {
				quote!(stardust_xr::values::Map<String, #t>)
			}
		}
		ArgumentType::NodeID => quote!(u64),
		ArgumentType::Datamap => {
			if !owned {
				quote!(&stardust_xr::values::Datamap)
			} else {
				quote!(stardust_xr::values::Datamap)
			}
		}
		ArgumentType::ResourceID => {
			if !owned {
				quote!(&stardust_xr::values::ResourceID)
			} else {
				quote!(stardust_xr::values::ResourceID)
			}
		}
		ArgumentType::Enum(e) => {
			let enum_name = Ident::new(&e.to_case(Case::Pascal), Span::call_site());
			quote!(#enum_name)
		}
		ArgumentType::Union(u) => {
			let union_name = Ident::new(&u.to_case(Case::Pascal), Span::call_site());
			quote!(#union_name)
		}
		ArgumentType::Struct(s) => {
			let struct_name = Ident::new(&s.to_case(Case::Pascal), Span::call_site());
			quote!(#struct_name)
		}
		ArgumentType::Node {
			_type,
			return_id_parameter_name: _,
		} => {
			if !owned {
				let aspect = Ident::new(
					&format!("{}Aspect", _type.to_case(Case::Pascal)),
					Span::call_site(),
				);
				quote!(&impl #aspect)
			} else {
				let node = Ident::new(&_type.to_case(Case::Pascal), Span::call_site());
				quote!(#node)
			}
		}
		ArgumentType::Fd => {
			quote!(std::os::unix::io::OwnedFd)
		}
	}
}
