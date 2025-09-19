pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const OWNED_ASPECT_ID: u64 = 15801764205032075891u64;
pub(crate) const OWNED_SET_ENABLED_SERVER_OPCODE: u64 = 13365497663235993822u64;
pub(crate) const OWNED_DESTROY_SERVER_OPCODE: u64 = 8637450960623370830u64;
#[derive(Debug)]
pub enum OwnedEvent {}
#[allow(clippy::all)]
///This node was created by the current client and can be disabled/destroyed
pub trait OwnedAspect: crate::node::NodeType + std::fmt::Debug {
    ///Set if this node is enabled or not. Disabled drawables won't render, input handlers won't receive input, etc.
    fn set_enabled(&self, enabled: bool) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (enabled);
        self.node()
            .send_remote_signal(
                15801764205032075891u64,
                13365497663235993822u64,
                &data,
                _fds,
            )?;
        let (enabled) = data;
        tracing::trace!(
            ? enabled, "Sent signal to server, {}::{}", "Owned", "set_enabled"
        );
        Ok(())
    }
    ///Destroy this node immediately. Not all nodes will have this method, those that don't can be dropped client-side without issue.
    fn destroy(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                15801764205032075891u64,
                8637450960623370830u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "Owned", "destroy");
        Ok(())
    }
}
