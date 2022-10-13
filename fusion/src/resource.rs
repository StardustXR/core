use std::{fmt::Debug, path::Path};

pub trait Resource: Debug {
	fn parse(&self) -> String;
}
impl<R: AsRef<Path> + Debug> Resource for R {
	fn parse(&self) -> String {
		self.as_ref().to_str().unwrap().to_string()
	}
}

#[derive(Debug, Clone)]
pub struct NamespacedResource {
	pub namespace: String,
	pub path: String,
}
impl NamespacedResource {
	pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
		NamespacedResource {
			namespace: namespace.into(),
			path: path.into(),
		}
	}
}
impl Resource for NamespacedResource {
	fn parse(&self) -> String {
		format!("{}:{}", self.namespace, self.path)
	}
}
