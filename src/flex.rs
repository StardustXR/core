use flexbuffers::Builder;

pub fn flexbuffer_from_arguments<S>(args_constructor: S) -> Vec<u8>
where
	S: Fn(&mut Builder),
{
	let mut fbb = Builder::default();
	args_constructor(&mut fbb);
	return fbb.view().to_vec();
}
