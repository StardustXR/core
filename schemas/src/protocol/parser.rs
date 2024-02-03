use super::*;
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue, NodeKey};
use std::fmt::Display;
use thiserror::Error;

pub fn convert(document: KdlDocument) -> Result<Protocol, ParseError> {
	let version = get_protocol_version(&document)?;
	let custom_enums = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "enum")
		.map(convert_enum)
		.collect::<Result<Vec<_>, ParseError>>()?;
	let custom_unions = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "union")
		.map(convert_union)
		.collect::<Result<Vec<_>, ParseError>>()?;
	let custom_structs = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "struct")
		.map(convert_struct)
		.collect::<Result<Vec<_>, ParseError>>()?;
	let interfaces = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "interface")
		.map(convert_interface)
		.collect::<Result<Vec<_>, ParseError>>()?;
	let nodes = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "node")
		.map(convert_node)
		.collect::<Result<Vec<_>, ParseError>>()?;
	let aspects = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "aspect")
		.map(convert_aspect)
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(Protocol {
		version,
		custom_enums,
		custom_unions,
		custom_structs,
		interfaces,
		nodes,
		aspects,
	})
}

fn convert_enum(custom_enum: &KdlNode) -> Result<CustomEnum, ParseError> {
	let nodes = custom_enum.children().unwrap().nodes();

	let name = get_string_property(custom_enum, 0)?.to_string();
	let description = get_description_node(custom_enum)?;
	let variants = nodes
		.iter()
		.filter(|n| n.name().value() == "variant")
		.map(|n| get_string_property(n, 0).map(ToString::to_string))
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(CustomEnum {
		name,
		description,
		variants,
	})
}
fn convert_union(custom_union: &KdlNode) -> Result<CustomUnion, ParseError> {
	let nodes = custom_union.children().unwrap().nodes();

	let name = get_string_property(custom_union, 0)?.to_string();
	let description = get_description_node(custom_union)?;
	let options = nodes
		.iter()
		.filter(|n| n.name().value() == "option")
		.map(convert_union_option)
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(CustomUnion {
		name,
		description,
		options,
	})
}
fn convert_union_option(union_option: &KdlNode) -> Result<UnionOption, ParseError> {
	let name = get_string_property(union_option, "name")
		.ok()
		.map(ToString::to_string);
	let description = get_string_property(union_option, "description")
		.ok()
		.map(ToString::to_string);
	let _type = convert_argument_type(union_option, "type")?;
	Ok(UnionOption {
		name,
		description,
		_type,
	})
}
fn convert_struct(custom_struct: &KdlNode) -> Result<CustomStruct, ParseError> {
	let nodes = custom_struct.children().unwrap().nodes();

	let name = get_string_property(custom_struct, 0)?.to_string();
	let description = get_description_node(custom_struct)?;
	let fields = nodes
		.iter()
		.filter(|n| n.name().value() == "field")
		.map(convert_argument)
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(CustomStruct {
		name,
		description,
		fields,
	})
}

fn convert_interface(interface: &KdlNode) -> Result<Interface, ParseError> {
	let nodes = interface.children().unwrap().nodes();

	let path = get_string_property(interface, 0)?.to_string();
	let description = get_description_node(interface)?;
	let members = nodes
		.iter()
		.filter(check_member)
		.map(|m| convert_member(m, Some(&path)))
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(Interface {
		path,
		description,
		members,
	})
}
fn convert_node(node: &KdlNode) -> Result<Node, ParseError> {
	let nodes = node.children().unwrap().nodes();

	let name = get_string_property(node, 0)?.to_string();
	let description = get_description_node(node)?;
	let aspects = nodes
		.iter()
		.filter(|n| n.name().value() == "aspect")
		.map(|n| get_string_property(n, 0).map(ToString::to_string))
		.collect::<Result<Vec<_>, ParseError>>()?;
	let members = nodes
		.iter()
		.filter(check_member)
		.map(|m| convert_member(m, None))
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(Node {
		name,
		description,
		aspects,
		members,
	})
}
fn convert_aspect(aspect: &KdlNode) -> Result<Aspect, ParseError> {
	let nodes = aspect.children().unwrap().nodes();

	let name = get_string_property(aspect, 0)?.to_string();
	let description = get_description_node(aspect)?;
	let members = nodes
		.iter()
		.filter(check_member)
		.map(|m| convert_member(m, None))
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(Aspect {
		name,
		description,
		members,
	})
}
fn check_member(member: &&KdlNode) -> bool {
	let name = member.name().value();
	name == "signal" || name == "method"
}
fn convert_member(member: &KdlNode, interface_path: Option<&str>) -> Result<Member, ParseError> {
	let nodes = member.children().unwrap().nodes();

	let _type = member.name().value();
	let _type = match _type {
		"signal" => MemberType::Signal,
		"method" => MemberType::Method,
		_ => {
			return Err(ParseError::InvalidStructType(
				member.name().value().to_string(),
				"signal or method".to_string(),
			))
		}
	};
	let side = get_string_property(member, "side")?;
	let side = match side {
		"server" => Side::Server,
		"client" => Side::Client,
		_ => {
			return Err(ParseError::InvalidPropertyType {
				field_name: member.name().value().to_string(),
				field_type: "side".to_string(),
			})
		}
	};

	let name = get_string_property(member, 0)?.to_string();
	let description = get_description_node(member)?;
	let arguments = nodes
		.iter()
		.filter(|n| n.name().value() == "argument")
		.map(convert_argument)
		.collect::<Result<Vec<_>, ParseError>>()?;

	let return_type = member
		.children()
		.unwrap()
		.nodes()
		.iter()
		.find(|n| n.name().value() == "return")
		.map(|return_node| convert_argument_type(return_node, "type"))
		.transpose()?;
	Ok(Member {
		name,
		description,
		side,
		_type,
		interface_path: interface_path.map(ToString::to_string),
		arguments,
		return_type,
	})
}
fn convert_argument(argument: &KdlNode) -> Result<Argument, ParseError> {
	let name = get_string_property(argument, 0)?.to_string();
	let description = get_string_property(argument, "description")
		.ok()
		.map(ToString::to_string);
	let _type = convert_argument_type(argument, "type")?;
	let optional = get_bool_property(argument, "optional")
		.ok()
		.unwrap_or(false);
	Ok(Argument {
		name,
		description,
		_type,
		optional,
	})
}
fn convert_argument_type(argument: &KdlNode, key: &str) -> Result<ArgumentType, ParseError> {
	Ok(match get_string_property(argument, key)? {
		"bool" => ArgumentType::Bool,
		"float" => ArgumentType::Float,
		"int" => ArgumentType::Int,
		"uint" => ArgumentType::UInt,
		"vec2" => ArgumentType::Vec2,
		"vec3" => ArgumentType::Vec3,
		"quat" => ArgumentType::Quat,
		"string" => ArgumentType::String,
		"color" => ArgumentType::Color,
		"bytes" => ArgumentType::Bytes,
		"vec" => ArgumentType::Vec(Box::new(convert_argument_type(argument, "member_type")?)),
		"map" => ArgumentType::Map(Box::new(convert_argument_type(argument, "value_type")?)),
		"datamap" => ArgumentType::Datamap,
		"resource" => ArgumentType::ResourceID,
		"enum" => ArgumentType::Enum(get_string_property(argument, "enum")?.to_string()),
		"union" => ArgumentType::Union(get_string_property(argument, "union")?.to_string()),
		"struct" => ArgumentType::Struct(get_string_property(argument, "struct")?.to_string()),
		"node" => ArgumentType::Node {
			_type: get_string_property(argument, "aspect")
				.or_else(|_| get_string_property(argument, "node"))?
				.to_string(),
			return_info: get_string_property(argument, "parent")
				.ok()
				.zip(get_string_property(argument, "name_argument").ok())
				.map(|(parent, name_argument)| NodeReturnInfo {
					parent: parent.to_string(),
					name_argument: name_argument.to_string(),
				}),
		},
		t => {
			return Err(ParseError::InvalidPropertyType {
				field_name: argument.name().value().to_string(),
				field_type: t.to_string(),
			})
		}
	})
}

fn get_protocol_version(document: &KdlDocument) -> Result<u32, ParseError> {
	let version_node = document
		.nodes()
		.iter()
		.find(|n| n.name().value() == "version")
		.ok_or(ParseError::MissingProtocolVersion)?;
	let version_argument = version_node
		.get(0)
		.ok_or(ParseError::MissingProtocolVersion)?;
	let version = version_argument
		.value()
		.as_i64()
		.ok_or(ParseError::MissingProtocolVersion)? as u32;
	Ok(version)
}

fn get_description_node(node: &KdlNode) -> Result<String, ParseError> {
	node.children()
		.unwrap()
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "description")
		.map(|n| get_string_property(n, 0).map(ToString::to_string))
		.find(|n| n.is_ok())
		.ok_or_else(|| ParseError::MissingProperty("description".to_string()))?
}

fn get_bool_property<'a>(
	node: &'a KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<bool, ParseError> {
	get_property(node, key)?
		.as_bool()
		.ok_or_else(|| ParseError::InvalidPropertyType {
			field_name: node.name().value().to_string(),
			field_type: "bool".to_string(),
		})
}
fn get_int_property<'a>(
	node: &'a KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<i64, ParseError> {
	get_property(node, key)?
		.as_i64()
		.ok_or_else(|| ParseError::InvalidPropertyType {
			field_name: node.name().value().to_string(),
			field_type: "int".to_string(),
		})
}
fn get_string_property<'a>(
	node: &'a KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<&'a str, ParseError> {
	get_property(node, key)?
		.as_string()
		.ok_or_else(|| ParseError::InvalidPropertyType {
			field_name: node.name().value().to_string(),
			field_type: "string".to_string(),
		})
}
fn get_property<'a>(
	node: &'a KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<&'a KdlValue, ParseError> {
	node.get(key.clone())
		.map(KdlEntry::value)
		.ok_or_else(|| ParseError::MissingProperty(key.to_string()))
}
