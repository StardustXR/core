pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 10u64;
#[allow(clippy::all)]
///Simple spatial audio source
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Sound(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Sound {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Sound(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Sound {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Sound {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl SoundAspect for Sound {}
pub(crate) const SOUND_ASPECT_ID: u64 = 17761155925539609649u64;
pub(crate) const SOUND_PLAY_SERVER_OPCODE: u64 = 18267594382511242772u64;
pub(crate) const SOUND_STOP_SERVER_OPCODE: u64 = 4968801543080236686u64;
#[derive(Debug)]
pub enum SoundEvent {}
#[allow(clippy::all)]
///Simple spatial audio source
pub trait SoundAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    ///Play sound effect
    fn play(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                17761155925539609649u64,
                18267594382511242772u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "Sound", "play");
        Ok(())
    }
    ///Stop sound effect
    fn stop(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                17761155925539609649u64,
                4968801543080236686u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "Sound", "stop");
        Ok(())
    }
}
pub(crate) const INTERFACE_CREATE_SOUND_SERVER_OPCODE: u64 = 3197851813257440734u64;
///Create a sound node. WAV and MP3 are supported.
fn create_sound(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    resource: &stardust_xr::values::ResourceID,
) -> crate::node::NodeResult<Sound> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, resource);
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(10u64, 0u64, 3197851813257440734u64, &serialized_data, _fds)?;
        let (id, parent, transform, resource) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? resource, "Sent signal to server, {}::{}",
            "Interface", "create_sound"
        );
    }
    Ok(Sound::from_id(_client, id, true))
}
