use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use split_iter::Splittable;
use stardust_xr_schemas::protocol::*;

fn fold_tokens(a: TokenStream, b: TokenStream) -> TokenStream {
	quote!(#a #b)
}

#[proc_macro]
pub fn codegen_root_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ROOT_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_node_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(NODE_PROTOCOL, false)
}
#[proc_macro]
pub fn codegen_spatial_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(SPATIAL_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_field_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(FIELD_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_data_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(DATA_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_audio_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(AUDIO_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_drawable_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(DRAWABLE_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_input_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(INPUT_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_item_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ITEM_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_item_camera_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ITEM_CAMERA_PROTOCOL, true)
}
#[proc_macro]
pub fn codegen_item_panel_protocol(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	codegen_client_protocol(ITEM_PANEL_PROTOCOL, true)
}

fn codegen_client_protocol(protocol: &'static str, generate_node: bool) -> proc_macro::TokenStream {
	let protocol = Protocol::parse(protocol).unwrap();
	let protocol_version = protocol.version;
	let protocol_version = quote!(pub(crate) const INTERFACE_VERSION: u32 = #protocol_version;);
	let interface_node_id = protocol
		.interface
		.as_ref()
		.map(|i| {
			let id = i.node_id;
			quote!(pub(crate) const INTERFACE_NODE_ID: u64 = #id;)
		})
		.unwrap_or_default();
	let custom_enums = protocol
		.custom_enums
		.iter()
		.map(generate_custom_enum)
		.reduce(fold_tokens)
		.unwrap_or_default();
	let custom_unions = protocol
		.custom_unions
		.iter()
		.map(generate_custom_union)
		.reduce(fold_tokens)
		.unwrap_or_default();
	let custom_structs = protocol
		.custom_structs
		.iter()
		.map(generate_custom_struct)
		.reduce(fold_tokens)
		.unwrap_or_default();
	let aspects = protocol
		.aspects
		.iter()
		.map(|a| generate_aspect(a, generate_node))
		.reduce(fold_tokens)
		.unwrap_or_default();
	let interface = protocol
		.interface
		.and_then(|p| {
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

					let member = generate_member(Some(p.node_id), m);
					quote! {
						pub(crate) const #name: u64 = #opcode;
						#member
					}
				})
				.reduce(fold_tokens)
		})
		.unwrap_or_default();
	quote!(#protocol_version #interface_node_id #custom_enums #custom_unions #custom_structs #aspects #interface)
		.into()
}

fn generate_custom_enum(custom_enum: &CustomEnum) -> TokenStream {
	let name = Ident::new(&custom_enum.name.to_case(Case::Pascal), Span::call_site());
	let description = &custom_enum.description;

	let argument_decls = custom_enum
		.variants
		.iter()
		.map(|a| Ident::new(&a.to_case(Case::Pascal), Span::call_site()).to_token_stream())
		.reduce(|a, b| quote!(#a, #b))
		.unwrap_or_default();

	quote! {
		#[doc = #description]
		#[derive(Debug, Clone, Copy, serde_repr::Deserialize_repr, serde_repr::Serialize_repr)]
		#[repr(u32)]
		pub enum #name {#argument_decls}
	}
}
fn generate_custom_union(custom_union: &CustomUnion) -> TokenStream {
	let name = Ident::new(&custom_union.name.to_case(Case::Pascal), Span::call_site());
	let description = &custom_union.description;

	let option_decls = custom_union
		.options
		.iter()
		.map(generate_union_option)
		.reduce(|a, b| quote!(#a, #b))
		.unwrap_or_default();

	quote! {
		#[doc = #description]
		#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
		#[serde(untagged)]
		pub enum #name {#option_decls}
	}
}
fn generate_union_option(union_option: &UnionOption) -> TokenStream {
	let name = union_option
		.name
		.as_ref()
		.map(|n| n.to_case(Case::Pascal))
		.unwrap_or_else(|| argument_type_option_name(&union_option._type));
	let description = union_option
		.description
		.as_ref()
		.map(|d| quote!(#[doc = #d]))
		.unwrap_or_default();
	let identifier = Ident::new(&name, Span::call_site());
	let _type = generate_argument_type(&union_option._type, true);
	quote! (#description #identifier(#_type))
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
		ArgumentType::Vec(v) => format!("{}Vector", argument_type_option_name(&v)),
		ArgumentType::Map(m) => format!("{}Map", argument_type_option_name(&m)),
		ArgumentType::NodeID => "Node ID".to_string(),
		ArgumentType::Datamap => "Datamap".to_string(),
		ArgumentType::ResourceID => "ResourceID".to_string(),
		ArgumentType::Enum(e) => e.clone(),
		ArgumentType::Union(u) => u.clone(),
		ArgumentType::Struct(s) => s.clone(),
		ArgumentType::Node { _type, .. } => _type.clone(),
	}
}
fn generate_custom_struct(custom_struct: &CustomStruct) -> TokenStream {
	let name = Ident::new(&custom_struct.name.to_case(Case::Pascal), Span::call_site());
	let description = &custom_struct.description;

	let argument_decls = custom_struct
		.fields
		.iter()
		.map(|a| generate_argument_decl(a, true))
		.map(|d| quote!(pub #d))
		.reduce(|a, b| quote!(#a, #b))
		.unwrap_or_default();

	quote! {
		#[doc = #description]
		#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
		pub struct #name {#argument_decls}
	}
}

fn generate_aspect(aspect: &Aspect, generate_node: bool) -> TokenStream {
	let node_name = Ident::new(&aspect.name, Span::call_site());
	let description = &aspect.description;
	let (client_members, server_members) = aspect.members.iter().split(|m| m.side == Side::Server);

	let aspect_handler_name = Ident::new(
		&format!("{}Handler", &aspect.name.to_case(Case::Pascal)),
		Span::call_site(),
	);
	let aspect_trait_name = Ident::new(
		&format!("{}Aspect", &aspect.name.to_case(Case::Pascal)),
		Span::call_site(),
	);

	let opcodes = aspect
		.members
		.iter()
		.map(|m| {
			let aspect_name = aspect.name.to_case(Case::ScreamingSnake);
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
		})
		.reduce(fold_tokens)
		.unwrap_or_default();

	let client_side = client_members
		.map(|m| generate_member(None, m))
		.reduce(fold_tokens)
		.map(|t| {
			quote! {
				#[doc = #description]
				pub trait #aspect_handler_name: Send + Sync + 'static {
					#t
				}
			}
		})
		.unwrap_or_default();
	let aspect_wrap = aspect
		.members
		.iter()
		.filter(|m| m.side == Side::Client)
		.map(generate_handler)
		.reduce(fold_tokens).map(|handlers| {
			quote! {
				#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
				fn wrap<H: #aspect_handler_name>(self, handler: H) -> NodeResult<crate::HandlerWrapper<Self, H>> {
					#aspect_trait_name::wrap_raw(self, std::sync::Arc::new(parking_lot::Mutex::new(handler)))
				}
				#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
				fn wrap_raw<H: #aspect_handler_name>(self, handler: std::sync::Arc<parking_lot::Mutex<H>>) -> NodeResult<crate::HandlerWrapper<Self, H>> {
					let handler_wrapper = crate::HandlerWrapper::new_raw(self, handler);
					<Self as #aspect_trait_name>::add_handlers(&handler_wrapper)?;
					Ok(handler_wrapper)
				}
				fn add_handlers<N: crate::node::NodeType, H: #aspect_handler_name>(handler_wrapper: &crate::HandlerWrapper<N, H>) -> NodeResult<()> {
					#handlers
					Ok(())
				}
			}
		}).unwrap_or_default();

	let inherit_types = aspect
		.inherits
		.iter()
		.map(|m| Ident::new(&format!("{m}Aspect"), Span::call_site()))
		.fold(quote!(crate::node::NodeType), |a, b| quote!(#a + #b));
	let server_side_members = server_members
		.map(|m| generate_member(None, m))
		.reduce(fold_tokens)
		.unwrap_or_default();
	let node = generate_node
		.then_some(quote! {
			#[doc = #description]
			#[derive(Debug)]
			pub struct #node_name (crate::node::Node);
			impl crate::node::NodeType for #node_name {
				fn node(&self) -> &crate::node::Node {
					&self.0
				}
				fn alias(&self) -> Self {
					#node_name(self.0.alias())
				}
				fn from_id(client: &std::sync::Arc<crate::client::Client>, id: u64, destroyable: bool) -> Self {
					#node_name(crate::node::Node::from_id(client, id, destroyable))
				}
			}
			impl serde::Serialize for #node_name {
				fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
					let node_id = self.0.get_id().map_err(|e| serde::ser::Error::custom(e))?;
					serializer.serialize_u64(node_id)
				}
			}
			impl #aspect_trait_name for #node_name {}
		})
		.unwrap_or_default();
	quote! {
		#node
		#opcodes
		#client_side
		#[doc = #description]
		pub trait #aspect_trait_name: #inherit_types {
			#aspect_wrap
			#server_side_members
		}
	}
}

fn generate_member(interface_node_id: Option<u64>, member: &Member) -> TokenStream {
	let opcode = member.opcode;
	let name = Ident::new(&member.name.to_case(Case::Snake), Span::call_site());
	let description = &member.description;

	let side = member.side;
	let _type = member._type;

	let first_arg = if interface_node_id.is_some() {
		quote!(client: &std::sync::Arc<crate::client::Client>)
	} else {
		if member.side == Side::Server {
			quote!(&self)
		} else {
			quote!(&mut self)
		}
	};
	let argument_decls = member
		.arguments
		.iter()
		.map(|a| generate_argument_decl(a, member.side == Side::Client))
		.fold(first_arg, |a, b| quote!(#a, #b));

	let argument_uses = member
		.arguments
		.iter()
		.map(|a| generate_argument_serialize(&a.name, &a._type, a.optional))
		.reduce(|a, b| quote!(#a, #b))
		.unwrap_or_default();
	let return_type = member
		.return_type
		.as_ref()
		.map(|r| generate_argument_type(&r, true))
		.unwrap_or_else(|| quote!(()));

	match (side, _type) {
		(Side::Server, MemberType::Method) => {
			let body = if let Some(interface_node_id) = &interface_node_id {
				quote! {
					let data = stardust_xr::schemas::flex::serialize(&(#argument_uses))?;
					let result = client.message_sender_handle.method(#interface_node_id, #opcode, &data, Vec::new())?.await?;
					Ok(stardust_xr::schemas::flex::deserialize(&result.into_message())?)
				}
			} else {
				quote! {
					self.node().execute_remote_method(#opcode, &(#argument_uses)).await
				}
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
		(Side::Server, MemberType::Signal) => {
			let mut body = if let Some(interface_node_id) = &interface_node_id {
				quote! {
					client.message_sender_handle.signal(#interface_node_id, #opcode, &stardust_xr::schemas::flex::serialize(&(#argument_uses))?, Vec::new())
				}
			} else {
				quote! {
					self.node().send_remote_signal(#opcode, &(#argument_uses))
				}
			};
			body = if let Some(ArgumentType::Node {
				_type: _,
				return_id_parameter_name,
			}) = &member.return_type
			{
				if let Some(return_id_parameter_name) = return_id_parameter_name {
					let id_argument = Ident::new(&return_id_parameter_name, Span::call_site());
					let get_client = if interface_node_id.is_some() {
						quote!(client)
					} else {
						quote!(self.node().client()?)
					};
					quote! {
						#body?;
						Ok(<#return_type as crate::node::NodeType>::from_id(&#get_client, #id_argument, true))
					}
				} else {
					quote! {
						Ok(#body?)
					}
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
		(Side::Client, MemberType::Method) => {
			quote! {
				#[doc = #description]
				fn #name(#argument_decls) -> crate::node::MethodResult<#return_type>;
			}
		}
		(Side::Client, MemberType::Signal) => {
			quote! {
				#[doc = #description]
				fn #name(#argument_decls);
			}
		}
	}
}
fn generate_handler(member: &Member) -> TokenStream {
	let name = &member.name;
	let opcode = member.opcode;
	let name_ident = Ident::new(&name, Span::call_site());

	let argument_names = member
		.arguments
		.iter()
		.map(generate_argument_name)
		.reduce(|a, b| quote!(#a, #b));
	let argument_types = member
		.arguments
		.iter()
		.map(|a| &a._type)
		.map(convert_deserializeable_argument_type)
		.map(|a| generate_argument_type(&a, true))
		.reduce(|a, b| quote!(#a, #b));
	// dbg!(&argument_types);
	let deserialize = argument_names
		.clone()
		.zip(argument_types)
		.map(|(argument_names, argument_types)| {
			quote!(let (#argument_names): (#argument_types) = stardust_xr::schemas::flex::deserialize(_data)?;)
		})
		.unwrap_or_default();
	let argument_uses = member
		.arguments
		.iter()
		.map(|a| generate_argument_deserialize(&a.name, &a._type, a.optional))
		.fold(TokenStream::default(), |a, b| quote!(#a, #b));
	match member._type {
		MemberType::Signal => quote! {
			handler_wrapper.add_handled_signal(#opcode, |_node, _handler, _data, _fds| {
				#deserialize
				let _client = _node.client()?;
				let mut _handler_lock = _handler.lock();
				Ok(H::#name_ident(&mut *_handler_lock #argument_uses))
			})?;
		},
		MemberType::Method => {
			let serialize =
				generate_argument_serialize("value", member.return_type.as_ref().unwrap(), false);
			quote! {
				handler_wrapper.add_handled_method(#opcode, |_node, _handler, _data, _fds| {
					#deserialize
					let _client = _node.client()?;
					let mut _handler_lock = _handler.lock();
					let value = H::#name_ident(&mut *_handler_lock #argument_uses)?;
					let data = stardust_xr::schemas::flex::serialize(&(#serialize))?;
					Ok((data, vec![]))
				})?;
			}
		}
	}
}
fn generate_argument_name(argument: &Argument) -> TokenStream {
	Ident::new(&argument.name.to_case(Case::Snake), Span::call_site()).to_token_stream()
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
				true => quote!(#name.map(|n| #node_type::from_id(&_client, n, false))),
				false => quote!(#node_type::from_id(&_client, #name, false)),
			}
		}
		ArgumentType::Color => quote!(color::rgba_linear!(#name[0], #name[1], #name[2], #name[3])),
		ArgumentType::Vec(v) => {
			let mapping = generate_argument_deserialize("a", v, false);
			quote!(#name.iter().map(|a| Ok(#mapping)).collect::<Result<Vec<_>, crate::node::NodeError>>()?)
		}
		ArgumentType::Map(v) => {
			let mapping = generate_argument_deserialize("a", v, false);
			quote!(#name.iter().map(|(k, a)| Ok((k, #mapping))).collect::<Result<rustc_hash::FxHashMap<String, _>, crate::node::NodeError>>()?)
		}
		_ => quote!(#name),
	}
}
fn convert_deserializeable_argument_type(argument_type: &ArgumentType) -> ArgumentType {
	match argument_type {
		ArgumentType::Node { .. } => ArgumentType::NodeID,
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
			true => quote!(#name.map(|n| n.node().get_id()).transpose()?),
			false => quote!(#name.node().get_id()?),
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
			let t = generate_argument_type(&t, true);
			if !owned {
				quote!(impl Into<stardust_xr::values::Vector2<#t>>)
			} else {
				quote!(stardust_xr::values::Vector2<#t>)
			}
		}
		ArgumentType::Vec3(t) => {
			let t = generate_argument_type(&t, true);
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
			let t = generate_argument_type(&t, true);
			if !owned {
				quote!(&[#t])
			} else {
				quote!(Vec<#t>)
			}
		}
		ArgumentType::Map(t) => {
			let t = generate_argument_type(&t, true);

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
	}
}
