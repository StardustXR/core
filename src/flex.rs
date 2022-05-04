use flexbuffers::{Builder, VectorBuilder};

pub fn flexbuffer_from_arguments<S>(args_constructor: S) -> Vec<u8>
where
	S: Fn(&mut Builder),
{
	let mut fbb = Builder::default();
	args_constructor(&mut fbb);
	return fbb.view().to_vec();
}

pub fn flexbuffer_from_vector_arguments<S>(args_constructor: S) -> Vec<u8>
where
	S: Fn(&mut VectorBuilder),
{
	let mut fbb = Builder::default();
	let mut vec = fbb.start_vector();
	args_constructor(&mut vec);
	vec.end_vector();
	return fbb.view().to_vec();
}
