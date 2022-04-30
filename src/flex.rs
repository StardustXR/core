use flexbuffers::{Builder};

pub fn flexbuffer_from_arguments<T>(args_constructor: T) -> Vec<u8> where T: Fn(&mut Builder) {
	let mut fbb = Builder::default();
	args_constructor(&mut fbb);
	return fbb.view().to_vec();
}

