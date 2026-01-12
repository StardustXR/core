use self::parser::convert;
use kdl::{KdlDocument, KdlError};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

mod parser;

pub const ROOT_PROTOCOL: &str = include_str!("../idl/root.kdl");
pub const NODE_PROTOCOL: &str = include_str!("../idl/node.kdl");
pub const SPATIAL_PROTOCOL: &str = include_str!("../idl/spatial.kdl");
pub const FIELD_PROTOCOL: &str = include_str!("../idl/field.kdl");
pub const AUDIO_PROTOCOL: &str = include_str!("../idl/audio.kdl");
pub const DRAWABLE_PROTOCOL: &str = include_str!("../idl/drawable.kdl");
pub const INPUT_PROTOCOL: &str = include_str!("../idl/input.kdl");
pub const ITEM_PROTOCOL: &str = include_str!("../idl/item.kdl");
pub const CAMERA_PROTOCOL: &str = include_str!("../idl/camera.kdl");
pub const ITEM_PANEL_PROTOCOL: &str = include_str!("../idl/item_panel.kdl");

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

#[derive(Debug, Clone)]
pub struct Aspect {
	pub name: String,
	pub id: u64, // FNV hash (https://crates.io/crates/fnv) of the aspect name
	pub description: String,
	pub inherits: Vec<String>,
	pub members: Vec<Member>,
	pub inherited_aspects: Vec<Aspect>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub struct ResolvedAspect {
	pub name: String,
	pub id: u64, // FNV hash (https://crates.io/crates/fnv) of the aspect name
	pub description: String,
	pub inherits: Vec<String>,
	pub members: Vec<Member>,
}

/// Resolves inheritance dependencies across all protocols in-place.
/// Each aspect's inherits list will be expanded to include all transitive dependencies.
pub fn resolve_inherits(protocols: &mut [&mut Protocol]) -> Result<(), String> {
	// Create a map of aspect name -> protocol index and aspect index for fast lookups
	let mut aspect_locations: HashMap<String, (usize, usize)> = HashMap::new();

	// Build the aspect location map
	for (protocol_idx, protocol) in protocols.iter().enumerate() {
		for (aspect_idx, aspect) in protocol.aspects.iter().enumerate() {
			if aspect_locations
				.insert(aspect.name.clone(), (protocol_idx, aspect_idx))
				.is_some()
			{
				return Err(format!("Duplicate aspect name: {}", aspect.name));
			}
		}
	}

	// For each aspect, resolve its full inheritance chain
	for protocol_idx in 0..protocols.len() {
		let aspect_count = protocols[protocol_idx].aspects.len();
		for aspect_idx in 0..aspect_count {
			// Get the current aspect's inherits list (we need to clone to avoid borrow checker issues)
			let current_inherits = protocols[protocol_idx].aspects[aspect_idx].inherits.clone();

			// Resolve the full inheritance chain
			let resolved_inherits = resolve_aspect_inheritance_chain(
				&protocols[protocol_idx].aspects[aspect_idx].name,
				&current_inherits,
				&aspect_locations,
				protocols,
			)?;

			// Update the aspect's inherits list with the resolved chain
			protocols[protocol_idx].aspects[aspect_idx].inherits = resolved_inherits
				.iter()
				.map(|aspect| aspect.name.clone())
				.collect();
			protocols[protocol_idx].aspects[aspect_idx].inherited_aspects = resolved_inherits;
		}
	}

	Ok(())
}

/// Recursively resolve the inheritance chain for a single aspect
fn resolve_aspect_inheritance_chain(
	aspect_name: &str,
	direct_inherits: &[String],
	aspect_locations: &HashMap<String, (usize, usize)>,
	protocols: &[&mut Protocol],
) -> Result<Vec<Aspect>, String> {
	let mut resolved = Vec::new();
	let mut visited = HashSet::new();
	let mut visiting = HashSet::new(); // Track aspects currently being processed to detect cycles

	// Use a stack to do depth-first traversal
	let mut to_visit: Vec<String> = direct_inherits.to_vec();

	while let Some(current_aspect) = to_visit.pop() {
		// Check for circular dependencies
		if visiting.contains(&current_aspect) {
			return Err(format!(
				"Circular inheritance detected involving aspect: {aspect_name} -> {current_aspect}",
			));
		}

		// Skip if already resolved
		if visited.contains(&current_aspect) {
			continue;
		}

		visiting.insert(current_aspect.clone());

		// Find the aspect and add it to resolved list
		if let Some(&(protocol_idx, aspect_idx)) = aspect_locations.get(&current_aspect) {
			resolved.push(protocols[protocol_idx].aspects[aspect_idx].clone());
			visited.insert(current_aspect.clone());

			// Add its dependencies to the stack
			let inherited_aspects = &protocols[protocol_idx].aspects[aspect_idx].inherits;
			for inherited in inherited_aspects {
				if !visited.contains(inherited) {
					to_visit.push(inherited.clone());
				}
			}
		} else {
			return Err(format!(
				"Aspect '{aspect_name}' inherits from unknown aspect '{current_aspect}'",
			));
		}

		visiting.remove(&current_aspect);
	}

	Ok(resolved)
}
