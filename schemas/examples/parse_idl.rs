use stardust_xr_schemas::protocol::Protocol;

fn main() {
	let kdl = include_str!("../src/protocol/drawable.kdl");
	println!("{kdl}");
	dbg!(Protocol::parse(kdl).unwrap());
}
