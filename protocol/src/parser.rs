use super::*;
use fnv::FnvHasher;
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue, NodeKey};
use std::{
	fmt::Display,
	hash::{Hash, Hasher},
};

pub fn convert(document: KdlDocument) -> Result<Protocol, ParseError> {
	let version = get_protocol_version(&document)?;
	let description = get_string_property(
		document
			.get("description")
			.ok_or(ParseError::MissingProtocolDescription)?,
		0,
	)?
	.to_owned();
	let interface_path = document
		.get("interface")
		.and_then(|n| get_int_property(n, 0).ok().map(|i| i as u64));
	if document.nodes().iter().any(|m| check_member(&m)) && interface_path.is_none() {
		return Err(ParseError::MissingProtocolInterfacePath);
	}
	let interface = interface_path
		.map(|node_id| {
			let members = document
				.nodes()
				.iter()
				.filter(check_member)
				.map(convert_member)
				.collect::<Result<Vec<_>, ParseError>>()?;
			Ok(Interface { node_id, members })
		})
		.transpose()?;

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

	let aspects = document
		.nodes()
		.iter()
		.filter(|n| n.name().value() == "aspect")
		.map(convert_aspect)
		.collect::<Result<Vec<_>, ParseError>>()?;
	Ok(Protocol {
		version,
		description,
		interface,
		custom_enums,
		custom_unions,
		custom_structs,
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

fn convert_aspect(aspect: &KdlNode) -> Result<Aspect, ParseError> {
	let nodes = aspect.children().unwrap().nodes();

	let name = get_string_property(aspect, 0)?.to_string();
	let description = get_description_node(aspect)?;
	let inherits = nodes
		.iter()
		.filter(|n| n.name().value() == "inherits")
		.map(|n| get_string_property(n, 0).map(ToString::to_string))
		.collect::<Result<Vec<_>, ParseError>>()?;
	let members = nodes
		.iter()
		.filter(check_member)
		.map(convert_member)
		.collect::<Result<Vec<_>, ParseError>>()?;
	let id = {
		let mut hasher = FnvHasher::default();
		name.hash(&mut hasher);
		hasher.finish()
	};
	Ok(Aspect {
		name,
		id,
		description,
		inherits,
		members,
		// populated later
		inherited_aspects: Vec::new(),
	})
}
fn check_member(member: &&KdlNode) -> bool {
	let name = member.name().value();
	name == "signal" || name == "method"
}
fn convert_member(member: &KdlNode) -> Result<Member, ParseError> {
	let nodes = member.children().unwrap().nodes();

	let _type = member.name().value();
	let _type = match _type {
		"signal" => MemberType::Signal,
		"method" => MemberType::Method,
		_ => {
			return Err(ParseError::InvalidStructType(
				member.name().value().to_string(),
				"signal or method".to_string(),
			));
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
			});
		}
	};

	let name = get_string_property(member, 0)?.to_string();
	let mut hasher = FnvHasher::default();
	name.hash(&mut hasher);
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
		opcode: hasher.finish(),
		description,
		side,
		_type,
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
		"empty" => ArgumentType::Empty,
		"bool" => ArgumentType::Bool,
		"int" => ArgumentType::Int,
		"uint" => ArgumentType::UInt,
		"float" => ArgumentType::Float,
		"vec2" => ArgumentType::Vec2(Box::new(
			convert_argument_type(argument, "component_type").unwrap_or(ArgumentType::Float),
		)),
		"vec3" => ArgumentType::Vec3(Box::new(
			convert_argument_type(argument, "component_type").unwrap_or(ArgumentType::Float),
		)),
		"quat" => ArgumentType::Quat,
		"mat4" => ArgumentType::Mat4,
		"string" => ArgumentType::String,
		"color" => ArgumentType::Color,
		"bytes" => ArgumentType::Bytes,
		"vec" => ArgumentType::Vec(Box::new(convert_argument_type(argument, "member_type")?)),
		"map" => ArgumentType::Map(Box::new(convert_argument_type(argument, "value_type")?)),
		"id" => ArgumentType::NodeID,
		"datamap" => ArgumentType::Datamap,
		"resource" => ArgumentType::ResourceID,
		"enum" => ArgumentType::Enum(get_string_property(argument, "enum")?.to_string()),
		"union" => ArgumentType::Union(get_string_property(argument, "union")?.to_string()),
		"struct" => ArgumentType::Struct(get_string_property(argument, "struct")?.to_string()),
		"node" => ArgumentType::Node {
			_type: get_string_property(argument, "aspect")
				.or_else(|_| get_string_property(argument, "node"))?
				.to_string(),
			return_id_parameter_name: get_string_property(argument, "id_argument")
				.map(ToString::to_string)
				.ok(),
		},
		"fd" => ArgumentType::Fd,
		t => {
			return Err(ParseError::InvalidPropertyType {
				field_name: argument.name().value().to_string(),
				field_type: t.to_string(),
			});
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

fn get_bool_property(
	node: &KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<bool, ParseError> {
	get_property(node, key)?
		.as_bool()
		.ok_or_else(|| ParseError::InvalidPropertyType {
			field_name: node.name().value().to_string(),
			field_type: "bool".to_string(),
		})
}
fn get_int_property(
	node: &KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<i64, ParseError> {
	get_property(node, key)?
		.as_i64()
		.ok_or_else(|| ParseError::InvalidPropertyType {
			field_name: node.name().value().to_string(),
			field_type: "int".to_string(),
		})
}
fn get_string_property(
	node: &KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<&str, ParseError> {
	get_property(node, key)?
		.as_string()
		.ok_or_else(|| ParseError::InvalidPropertyType {
			field_name: node.name().value().to_string(),
			field_type: "string".to_string(),
		})
}
fn get_property(
	node: &KdlNode,
	key: impl Into<NodeKey> + Display + Clone,
) -> Result<&KdlValue, ParseError> {
	node.get(key.clone())
		.map(KdlEntry::value)
		.ok_or_else(|| ParseError::MissingProperty(key.to_string()))
}
