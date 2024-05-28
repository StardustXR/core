use self::parser::convert;
use kdl::{KdlDocument, KdlError};
use thiserror::Error;

mod parser;

// pub const ROOT_PROTOCOL: &'static str = include_str!("root.kdl");
pub const NODE_PROTOCOL: &'static str = include_str!("node.kdl");
pub const SPATIAL_PROTOCOL: &'static str = include_str!("spatial.kdl");
pub const FIELD_PROTOCOL: &'static str = include_str!("field.kdl");
pub const DATA_PROTOCOL: &'static str = include_str!("data.kdl");
pub const AUDIO_PROTOCOL: &'static str = include_str!("audio.kdl");
pub const DRAWABLE_PROTOCOL: &'static str = include_str!("drawable.kdl");
pub const INPUT_PROTOCOL: &'static str = include_str!("input.kdl");
pub const ITEM_PROTOCOL: &'static str = include_str!("item.kdl");

#[derive(Debug)]
pub struct Protocol {
	pub version: u32,
	pub description: String,
	pub interface: Option<Interface>,
	pub custom_enums: Vec<CustomEnum>,
	pub custom_structs: Vec<CustomStruct>,
	pub custom_unions: Vec<CustomUnion>,
	pub aspects: Vec<Aspect>,
}
impl Protocol {
	pub fn parse(sbs: &str) -> Result<Self, ParseError> {
		let parsed: KdlDocument = sbs.parse().map_err(|p: KdlError| ParseError::Kdl(p))?;
		convert(parsed)
	}
}

#[derive(Debug)]
pub struct Interface {
	pub path: String,
	pub members: Vec<Member>,
}

#[derive(Debug)]
pub struct CustomStruct {
	pub name: String,
	pub description: String,
	pub fields: Vec<Argument>,
}

#[derive(Debug)]
pub struct CustomEnum {
	pub name: String,
	pub description: String,
	pub variants: Vec<String>,
}

#[derive(Debug)]
pub struct CustomUnion {
	pub name: String,
	pub description: String,
	pub options: Vec<UnionOption>,
}

#[derive(Debug)]
pub struct UnionOption {
	pub name: Option<String>,
	pub description: Option<String>,
	pub _type: ArgumentType,
}

#[derive(Debug)]
pub struct Aspect {
	pub name: String,
	pub description: String,
	pub inherits: Vec<String>,
	pub members: Vec<Member>,
}

#[derive(Debug)]
pub struct Member {
	pub name: String,
	pub description: String,
	pub side: Side,
	pub _type: MemberType,
	pub arguments: Vec<Argument>,
	pub return_type: Option<ArgumentType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberType {
	Signal,
	Method,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
	Client,
	Server,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
	Bool,
	Int,
	UInt,
	Float,
	Vec2,
	Vec3,
	Quat,
	Color,
	String,
	Bytes,
	Vec(Box<ArgumentType>),
	Map(Box<ArgumentType>),
	Datamap,
	ResourceID,
	Enum(String),
	Union(String),
	Struct(String),
	Node {
		_type: String,
		return_info: Option<NodeReturnInfo>,
	},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeReturnInfo {
	pub parent: String,
	pub name_argument: String,
}

#[derive(Debug)]
pub struct Argument {
	pub name: String,
	pub description: Option<String>,
	pub _type: ArgumentType,
	pub optional: bool,
}

#[derive(Debug, Error)]
pub enum ParseError {
	#[error("{0}")]
	Kdl(KdlError),
	#[error("Missing protocol version")]
	MissingProtocolVersion,
	#[error("Missing protocol description")]
	MissingProtocolDescription,
	#[error("Missing interface path for signals/methods")]
	MissingProtocolInterfacePath,
	#[error("Struct {0} should be of type {1} but is not")]
	InvalidStructType(String, String),
	#[error("Field {0} should be present but is not")]
	MissingProperty(String),
	#[error("Field {field_name} should be of type {field_type} but is not")]
	InvalidPropertyType {
		field_name: String,
		field_type: String,
	},
}
