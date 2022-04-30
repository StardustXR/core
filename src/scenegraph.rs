pub trait Scenegraph {
	fn send_signal(&self, path: &str, method: &str, data: &flexbuffers::Reader<&[u8]>) {
		self.execute_method(path, method, data);
	}
	fn execute_method(&self, path: &str, method: &str, data: &flexbuffers::Reader<&[u8]>) -> Vec<u8>;
}

pub struct SampleScenegraph {
	test: u8,
}

impl SampleScenegraph {
	pub fn new() -> SampleScenegraph {
		SampleScenegraph {test: 1,}
	}
}

impl Scenegraph for SampleScenegraph {
	fn execute_method(&self, path: &str, method: &str, data: &flexbuffers::Reader<&[u8]>) -> Vec<u8> {
		Vec::<u8>::new()
	}
}
