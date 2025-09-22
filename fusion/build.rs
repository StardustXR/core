use color_eyre::Result;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use stardust_xr_schemas::protocol::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::once;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
fn main() {
	// Watch for changes to KDL schema files
	let schema_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
		.parent()
		.unwrap()
		.join("schemas/src/protocol");

	// Tell cargo to rerun if any KDL files change
	println!(
		"cargo:rerun-if-changed={}",
		schema_dir.join("*.kdl").display()
	);

	// Also rerun if the codegen library itself changes
	println!("cargo:rerun-if-changed=codegen/src/lib.rs");

	// Use the codegen library directly to regenerate protocol files
	let protocols = get_all_protocols();
	let output_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/protocol.rs");

	generate_protocol_file(protocols, &output_dir, false)
		.expect("Failed to generate protocol files");

	println!("Protocol files regenerated successfully");
}

pub struct ProtocolInfo {
	pub name: &'static str,
	pub content: &'static str,
	pub generate_node: bool,
	pub partial_eq: bool,
}

pub fn get_all_protocols() -> Vec<ProtocolInfo> {
	vec![
		ProtocolInfo {
			name: "root",
			content: ROOT_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "node",
			content: NODE_PROTOCOL,
			generate_node: false,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "spatial",
			content: SPATIAL_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "field",
			content: FIELD_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "audio",
			content: AUDIO_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "drawable",
			content: DRAWABLE_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "input",
			content: INPUT_PROTOCOL,
			generate_node: true,
			partial_eq: false,
		},
		ProtocolInfo {
			name: "item",
			content: ITEM_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "item_camera",
			content: ITEM_CAMERA_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
		ProtocolInfo {
			name: "item_panel",
			content: ITEM_PANEL_PROTOCOL,
			generate_node: true,
			partial_eq: true,
		},
	]
}

pub fn generate_protocol_file(
	protocols: Vec<ProtocolInfo>,
	file_path: &Path,
	_force: bool,
) -> Result<()> {
	let mut protocols = protocols
		.into_iter()
		.map(|p| (Protocol::parse(p.content).unwrap(), p))
		.collect::<Vec<_>>();

	let mut protocol_definitions = protocols.iter_mut().map(|(p, _)| p).collect::<Vec<_>>();
	stardust_xr_schemas::protocol::resolve_inherits(&mut protocol_definitions).unwrap();

	// panic!("{protocol_definitions:# ?}");

	let protocols = protocols
		.into_iter()
		.map(|p| {
			let protocol = p.0.tokenize(p.1.generate_node, p.1.partial_eq);
			let mod_name = Ident::new(p.1.name, Span::call_site());
			quote! {
				#[allow(unused_imports)]
				use #mod_name::*;
				pub mod #mod_name {
					#[allow(unused_imports)]
					use super::*;
					#protocol
				}
			}
		})
		.reduce(|a, b| {
			quote! {
				#a


				//=====================================
				#b
			}
		})
		.unwrap_or_default();

	let file_tokens = quote! {
		#![allow(async_fn_in_trait, unused_parens, clippy::all)]
		// Generated code - do not edit manually

		use crate::node::NodeType;

		pub(crate) trait AddAspect<A> {
			fn add_aspect(registry: &crate::scenegraph::NodeRegistry, node_id: u64, aspect_id: u64);
		}

		//=====================================
		#protocols
	};

	let syn_file = syn::parse2::<syn::File>(file_tokens).unwrap();
	let formatted = prettyplease::unparse(&syn_file);

	// Generate mod.rs
	fs::write(file_path, formatted)?;

	Ok(())
}

trait Tokenize {
	fn tokenize(&self, generate_node: bool, partial_eq: bool) -> TokenStream;
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
			.map(|a| a.blocking_read().tokenize(generate_node, partial_eq));
		let interface = self
			.interface
			.as_ref()
			.map(|p| {
				p.members
					.iter()
					.map(|m| generate_server_member(Some(p.node_id), 0, "Interface", m))
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
		let server_members = self.members.iter().filter(|m| m.side == Side::Server);

		let aspect_trait_name = Ident::new(
			&format!("{}Aspect", &self.name.to_case(Case::Pascal)),
			Span::call_site(),
		);

		let aspect_name = Ident::new(&self.name.to_case(Case::Pascal), Span::call_site());
		let event_name = Ident::new(&format!("{aspect_name}Event"), Span::call_site());
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
			.map(|i| quote!(super::#i))
			.fold(quote!(crate::node::NodeType), |a, b| quote!(#a + #b));
		let server_side_members =
			server_members.map(|m| generate_server_member(None, self.id, &self.name, m));

		fn aspect_filter(a: &Aspect) -> bool {
			a.name.to_case(Case::Snake) != "owned"
				&& a.members.iter().any(|m| m.side == Side::Client)
		}

		let conversion_functions = self
			.inherited_aspects
			.iter()
			.filter(|i| i.blocking_read().name.to_case(Case::Snake) != "owned")
			.map(|aspect| {
				let a = aspect.blocking_read();
				let inherited_aspect =
					Ident::new(&a.name.to_case(Case::UpperCamel), Span::call_site());
				let conversion_fn_name = Ident::new(
					&format!("as_{}", a.name.to_case(Case::Snake)),
					Span::call_site(),
				);
				let inherited_aspects = a
					.inherited_aspects
					.iter()
					.map(|aspect| aspect.blocking_read())
					.collect::<Vec<_>>();
				let fields = inherited_aspects
					.iter()
					.chain([&a])
					.filter(|a| aspect_filter(a))
					.map(|aspect| {
						let event_name = Ident::new(
							&format!("{}_event", aspect.name.to_case(Case::Snake)),
							Span::call_site(),
						);
						quote! {
							#event_name: self.#event_name,
						}
					});

				quote! {
					pub fn #conversion_fn_name(self) -> super::#inherited_aspect {
						super::#inherited_aspect {
							core: self.core,
							#(#fields)*
						}
					}
				}
			});
		let inherited_aspects = self
			.inherited_aspects
			.iter()
			.map(|aspect| aspect.blocking_read())
			.collect::<Vec<_>>();
		let get_aspects_iter = || {
			inherited_aspects
				.iter()
				.map(|v| v.deref())
				.chain([self])
				.filter(|a| aspect_filter(a))
		};
		let event_fields = get_aspects_iter().map(|a| {
			let event_name = Ident::new(
				&format!("{}_event", a.name.to_case(Case::Snake)),
				Span::call_site()
			);
			let event_type = Ident::new(
				&format!("{}Event", a.name.to_case(Case::Pascal)),
				Span::call_site()
			);
			quote! {
				pub(crate) #event_name: std::sync::Arc<tokio::sync::Mutex<tokio::sync::mpsc::Receiver<#event_type>>>,
			}
		});
		let from_id_add_aspects = get_aspects_iter().map(|a| {
			let aspect_id = a.id;
			let event_name = Ident::new(
				&format!("{}_event", a.name.to_case(Case::Snake)),
				Span::call_site(),
			);
			quote! {
				let #event_name = std::sync::Arc::new(client.registry.add_aspect::<>(id, #aspect_id).into());
			}
		});
		let from_id_event_fields = get_aspects_iter().map(|a| {
			let event_name = Ident::new(
				&format!("{}_event", a.name.to_case(Case::Snake)),
				Span::call_site(),
			);
			quote! {
				#event_name,
			}
		});
		let impl_traits = inherited_aspects
			.iter()
			.map(|v| v.deref())
			.chain([self])
			.map(|a| {
				let i = &a.name;
				let aspect_trait_name = Ident::new(
					&format!("{}Aspect", i.to_case(Case::Pascal)),
					Span::call_site(),
				);
				if a.members.iter().any(|m| m.side == Side::Client) {
					let recv_event_method_name = Ident::new(
						&format!("recv_{}_event", i.to_case(Case::Snake)),
						Span::call_site(),
					);
					let event_type = Ident::new(
						&format!("{}Event", i.to_case(Case::Pascal)),
						Span::call_site(),
					);
					let event_name = Ident::new(
						&format!("{}_event", i.to_case(Case::Snake)),
						Span::call_site(),
					);

					quote! {
						impl #aspect_trait_name for #node_name {
							fn #recv_event_method_name(&self) -> Option<#event_type> {
								self.#event_name.blocking_lock().try_recv().ok()
							}
						}
					}
				} else {
					quote! {
						impl #aspect_trait_name for #node_name {}
					}
				}
			})
			.reduce(|a, b| quote!(#a #b))
			.unwrap_or_default();
		let node = generate_node.then(|| quote! {
			#[doc = #description]
			#[derive(Debug, Clone)]
			pub struct #node_name {
				pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
				#(#event_fields)*
			}
			impl #node_name {
				pub(crate) fn from_id(client: &std::sync::Arc<crate::client::ClientHandle>, id: u64, owned: bool) -> Self {
					let core = std::sync::Arc::new(crate::node::NodeCore::new(client.clone(), id, owned));
					#(#from_id_add_aspects)*
					#node_name {
						core,
						#(#from_id_event_fields)*
					}
				}
				#(#conversion_functions)*
			}
			impl crate::node::NodeType for #node_name {
				fn node(&self) -> &crate::node::NodeCore {
					&self.core
				}
			}
			impl serde::Serialize for #node_name {
				fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
					serializer.serialize_u64(self.core.id)
				}
			}
			#impl_traits
		});

		let events_method = if self.members.iter().any(|m| m.side == Side::Client) {
			let recv_event_method_name = Ident::new(
				&format!("recv_{}_event", self.name.to_case(Case::Snake)),
				Span::call_site(),
			);
			let id = self.id;
			quote! {
				fn #recv_event_method_name(&self) -> Option<#event_name>;
			}
		} else {
			Default::default()
		};
		quote! {
			#node

			#aspect_events
			#aspect_event_sender_impl

			#[doc = #description]
			pub trait #aspect_trait_name: #inherit_types + std::fmt::Debug {
				#events_method
				#(#server_side_members)*
			}
		}
	}
}

fn generate_event_sender_impl(aspect: &Aspect) -> TokenStream {
	let aspect_name = aspect.name.to_case(Case::Pascal);
	let event_name = Ident::new(&format!("{aspect_name}Event"), Span::call_site());
	let opcode = aspect.id;

	let (signal_matches, method_matches): (Vec<_>, Vec<_>) = aspect
		.members
		.iter()
		.filter(|m| m.side == Side::Client)
		.map(|member| -> (MemberType, TokenStream) {
			let opcode = member.opcode;
			let variant_str = member.name.to_case(Case::Snake);
			let variant_name = Ident::new(&member.name.to_case(Case::Pascal), Span::call_site());
			let field_names = member
				.arguments
				.iter()
				.map(|a| Ident::new(&a.name.to_case(Case::Snake), Span::call_site()));
			let fields_debug = member
				.arguments
				.iter()
				.map(|a| Ident::new(&a.name.to_case(Case::Snake), Span::call_site()))
				.map(|a| quote!(?#a))
				.reduce(|a, b| quote!(#a, #b))
				.map(|a| quote!(#a,));
			let deserialize_types = member
				.arguments
				.iter()
				.map(|a| generate_argument_type(&convert_deserializeable_argument_type(&a._type), true));
			let field_uses: Vec<_> = member
				.arguments
				.iter()
				.map(|a| {
					let name_ident = Ident::new(&a.name.to_case(Case::Snake), Span::call_site());
					let argument_deserialize =
						generate_argument_deserialize(&a.name, &a._type, a.optional);
					quote!(#name_ident: #argument_deserialize)
				})
				.collect();

			let response_sender = if member._type == MemberType::Method {
				quote! {
					response: crate::TypedMethodResponse(response, std::marker::PhantomData),
				}
			} else {
				quote!()
			};

			let debug = if member._type == MemberType::Signal {
				quote! {
					tracing::trace!(#fields_debug "Got signal from server, {}::{}", #aspect_name, #variant_str);
				}
			} else {
				quote! {
					tracing::trace!(#fields_debug "Method called from server, {}::{}", #aspect_name, #variant_str);
				}
			};

			(
				member._type,
				quote! {
					#opcode => {
						let (#(#field_names),*): (#(#deserialize_types),*) = stardust_xr::schemas::flex::deserialize(_data)?;

						#debug
						Ok(#event_name::#variant_name { #(#field_uses,)* #response_sender })
					}
				},
			)
		})
		.partition(|(_ty, _)| _ty == &MemberType::Signal);

	let signal_matches = signal_matches.into_iter().map(|t| t.1);
	let method_matches = method_matches.into_iter().map(|t| t.1);

	quote! {
		impl crate::scenegraph::EventParser for #event_name {
			const ASPECT_ID: u64 = #opcode;

			fn parse_signal(_client: &std::sync::Arc<crate::client::ClientHandle>, signal_id: u64, _data: &[u8], _fds: Vec<std::os::fd::OwnedFd>) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
				match signal_id {
					#(#signal_matches)*
					_ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
				}
			}
			fn parse_method(_client: &std::sync::Arc<crate::client::ClientHandle>, method_id: u64, _data: &[u8], _fds: Vec<std::os::fd::OwnedFd>, response: stardust_xr::messenger::MethodResponse) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
				match method_id {
					#(#method_matches)*
					_ => {
						let _ = response.send(Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound));
						Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
					}
				}
			}
		}
	}
}

fn generate_server_member(
	interface_node_id: Option<u64>,
	aspect_id: u64,
	aspect_name: &str,
	member: &Member,
) -> TokenStream {
	let opcode = member.opcode;
	let name_str = member.name.to_case(Case::Snake);
	let name = Ident::new(&name_str, Span::call_site());
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

	let arguments = member
		.arguments
		.iter()
		.map(|a| Ident::new(&a.name.to_case(Case::Snake), Span::call_site()));
	let arguments_debug = member
		.arguments
		.iter()
		.map(|a| Ident::new(&a.name.to_case(Case::Snake), Span::call_site()))
		.map(|a| quote!(?#a))
		.reduce(|a, b| quote!(#a, #b))
		.map(|a| quote!(#a,));
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
				quote! {
					let mut _fds = Vec::new();
					let data = (#(#argument_uses),*);
					let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
					_client.message_sender_handle.signal(#interface_node_id, #aspect_id, #opcode, &serialized_data, _fds)?;

					let (#(#arguments),*) = data;
					tracing::trace!(#arguments_debug "Sent signal to server, {}::{}", #aspect_name, #name_str);
				}
			} else {
				quote! {
					let mut _fds = Vec::new();
					let data = (#(#argument_uses),*);
					self.node().send_signal(#aspect_id, #opcode, &data, _fds)?;

					let (#(#arguments),*) = data;
					tracing::trace!(#arguments_debug "Sent signal to server, {}::{}", #aspect_name, #name_str);
				}
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
					quote!(&self.node().client)
				};
				quote! {
					{ #body }
					Ok(#return_type::from_id(#get_client, #id_argument, true))
				}
			} else {
				quote! {
					#body
					Ok(())
				}
			};
			if interface_node_id.is_some() {
				return quote! {
					#[doc = #description]
					pub fn #name(#argument_decls) -> crate::node::NodeResult<#return_type> {
						#body
					}
				};
			}
			quote! {
				#[doc = #description]
				fn #name(#argument_decls) -> crate::node::NodeResult<#return_type> {
					#body
				}
			}
		}
		MemberType::Method => {
			let argument_type = member.return_type.clone().unwrap_or(ArgumentType::Empty);
			let deserializeable_type = generate_argument_type(
				&convert_deserializeable_argument_type(&argument_type),
				true,
			);
			let deserialize = generate_argument_deserialize("result", &argument_type, false);
			let body = if let Some(interface_node_id) = &interface_node_id {
				quote! {
					let mut _fds = Vec::new();
					let data = (#(#argument_uses),*);
					{
						let (#(#arguments),*) = &data;
						tracing::trace!(#arguments_debug "Called method on server, {}::{}", #aspect_name, #name_str);
					}
					let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
					let message = _client.message_sender_handle.method(#interface_node_id, #aspect_id, #opcode, &serialized_data, _fds).await?.map_err(|e| crate::node::NodeError::ReturnedError { e })?.into_message();
					let result: #deserializeable_type = stardust_xr::schemas::flex::deserialize(&message)?;
					let deserialized = #deserialize;
					tracing::trace!("return" = ?deserialized, "Method return from server, {}::{}", #aspect_name, #name_str);
					Ok(deserialized)
				}
			} else {
				quote! {{
					let mut _fds = Vec::new();
					let data = (#(#argument_uses),*);
					{
						let (#(#arguments),*) = &data;
						tracing::trace!(#arguments_debug "Called method on server, {}::{}", #aspect_name, #name_str);
					}
					let result: #deserializeable_type = self.node().call_method(#aspect_id, #opcode, &data, _fds).await?;
					let deserialized = #deserialize;
					tracing::trace!("return" = ?deserialized, "Method return from server, {}::{}", #aspect_name, #name_str);
					Ok(deserialized)
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
					quote!(#name.map(|n| #node_type::from_id(_client, n, false)))
				}
				false => {
					quote!(#node_type::from_id(_client, #name, false))
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
			quote!(#name.into_iter().map(|(k, a)| Ok((k, #mapping))).collect::<Result<stardust_xr::values::Map<String, _>, crate::node::NodeError>>()?)
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
			true => quote!(#name.map(|n| n.node().id)),
			false => quote!(#name.node().id),
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
