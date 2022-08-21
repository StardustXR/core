#[derive(Clone)]
pub struct Resource {
	pub namespace: String,
	pub path: String,
}
impl Resource {
	pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
		Resource {
			namespace: namespace.into(),
			path: path.into(),
		}
	}
}
