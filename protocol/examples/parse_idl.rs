use stardust_xr_protocol::Protocol;

fn main() {
	let kdl = include_str!("../idl/spatial.kdl");
	println!("{kdl}");
	dbg!(Protocol::parse(kdl).unwrap());
}
