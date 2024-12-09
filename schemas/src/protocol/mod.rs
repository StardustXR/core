use self::parser::convert;
use kdl::{KdlDocument, KdlError};
use thiserror::Error;

mod parser;

pub const ROOT_PROTOCOL: &str = include_str!("root.kdl");
pub const NODE_PROTOCOL: &str = include_str!("node.kdl");
pub const SPATIAL_PROTOCOL: &str = include_str!("spatial.kdl");
pub const FIELD_PROTOCOL: &str = include_str!("field.kdl");
pub const AUDIO_PROTOCOL: &str = include_str!("audio.kdl");
pub const DRAWABLE_PROTOCOL: &str = include_str!("drawable.kdl");
pub const INPUT_PROTOCOL: &str = include_str!("input.kdl");
pub const ITEM_PROTOCOL: &str = include_str!("item.kdl");
pub const ITEM_CAMERA_PROTOCOL: &str = include_str!("item_camera.kdl");
pub const ITEM_PANEL_PROTOCOL: &str = include_str!("item_panel.kdl");

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
	pub node_id: u64,
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
	pub id: u64, // FNV hash (https://crates.io/crates/fnv) of the aspect name
	pub description: String,
	pub inherits: Vec<String>,
	pub members: Vec<Member>,
}

#[derive(Debug)]
pub struct Member {
	pub name: String,
	pub opcode: u64, // FNV hash (https://crates.io/crates/fnv) of the member name
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
	Empty,
	Bool,
	Int,
	UInt,
	Float,
	Vec2(Box<ArgumentType>),
	Vec3(Box<ArgumentType>),
	Quat,
	Mat4,
	Color,
	String,
	Bytes,
	Vec(Box<ArgumentType>),
	Map(Box<ArgumentType>),
	NodeID,
	Datamap,
	ResourceID,
	Enum(String),
	Union(String),
	Struct(String),
	Node {
		_type: String,
		return_id_parameter_name: Option<String>,
	},
	Fd,
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
