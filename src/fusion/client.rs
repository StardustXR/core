use crate::messenger::Messenger;
use crate::scenegraph::Scenegraph;

pub struct Client<'a> {
	messenger: Messenger<'a>,
	// scenegraph: Scenegraph<'a>,
}
