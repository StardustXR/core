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

#[derive(Default, Clone)]
pub struct OwnedBuffer(Vec<u8>);
impl OwnedBuffer {
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}
}
impl From<Vec<u8>> for OwnedBuffer {
	fn from(vec: Vec<u8>) -> Self {
		OwnedBuffer(vec)
	}
}
impl From<&[u8]> for OwnedBuffer {
	fn from(slice: &[u8]) -> Self {
		OwnedBuffer(slice.to_vec())
	}
}
impl From<OwnedBuffer> for Vec<u8> {
	fn from(buf: OwnedBuffer) -> Vec<u8> {
		buf.0
	}
}

impl std::ops::Deref for OwnedBuffer {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl flexbuffers::Buffer for OwnedBuffer {
	type BufferString = String;
	fn slice(&self, range: std::ops::Range<usize>) -> Option<Self> {
		self.0
			.as_slice()
			.get(range)
			.map(|slice| OwnedBuffer(slice.into()))
	}
	#[inline]
	fn shallow_copy(&self) -> Self {
		self.clone()
	}
	fn empty() -> Self {
		Default::default()
	}
	#[inline]
	fn empty_str() -> Self::BufferString {
		Self::empty().buffer_str().unwrap()
	}
	fn buffer_str(&self) -> Result<Self::BufferString, std::str::Utf8Error> {
		String::from_utf8(self.0.clone()).map_err(|e| e.utf8_error())
	}
}
