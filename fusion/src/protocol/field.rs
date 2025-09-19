pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 2u64;
///The shape of a given field.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Shape {
    ///Box with a given size in meters
    Box(stardust_xr::values::Vector3<f32>),
    ///Cylinder aligned to the XZ plane
    Cylinder(CylinderShape),
    ///Sphere with a given radius in meters
    Sphere(f32),
    ///Torus aligned to the XZ plane
    Torus(TorusShape),
}
///Information about raymarching a field
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RayMarchResult {
    pub ray_origin: stardust_xr::values::Vector3<f32>,
    pub ray_direction: stardust_xr::values::Vector3<f32>,
    pub min_distance: f32,
    pub deepest_point_distance: f32,
    pub ray_length: f32,
    pub ray_steps: u32,
}
///Cylinder shape info
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CylinderShape {
    pub length: f32,
    pub radius: f32,
}
///Torus shape info
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TorusShape {
    pub radius_a: f32,
    pub radius_b: f32,
}
#[allow(clippy::all)]
///A node that is spatial and contains an SDF
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FieldRef(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl FieldRef {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        FieldRef(node)
    }
    pub fn as_spatial_ref(self) -> super::SpatialRef {
        super::SpatialRef(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for FieldRef {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for FieldRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl FieldRefAspect for FieldRef {}
pub(crate) const FIELD_REF_ASPECT_ID: u64 = 10662923473076663509u64;
pub(crate) const FIELD_REF_DISTANCE_SERVER_OPCODE: u64 = 12706699825100237095u64;
pub(crate) const FIELD_REF_NORMAL_SERVER_OPCODE: u64 = 10933809934326220183u64;
pub(crate) const FIELD_REF_CLOSEST_POINT_SERVER_OPCODE: u64 = 13473947755141124846u64;
pub(crate) const FIELD_REF_RAY_MARCH_SERVER_OPCODE: u64 = 7352457860499612292u64;
#[derive(Debug)]
pub enum FieldRefEvent {}
#[allow(clippy::all)]
///A node that is spatial and contains an SDF
pub trait FieldRefAspect: crate::node::NodeType + super::SpatialRefAspect + std::fmt::Debug {
    ///Get the distance to the surface of this field relative to the `point` in `space`
    async fn distance(
        &self,
        space: &impl SpatialRefAspect,
        point: impl Into<stardust_xr::values::Vector3<f32>>,
    ) -> crate::node::NodeResult<f32> {
        {
            let mut _fds = Vec::new();
            let data = (space.node().id(), point.into());
            {
                let (space, point) = &data;
                tracing::trace!(
                    ? space, ? point, "Called method on server, {}::{}", "FieldRef",
                    "distance"
                );
            }
            let result: f32 = self
                .node()
                .execute_remote_method(
                    10662923473076663509u64,
                    12706699825100237095u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "FieldRef", "distance"
            );
            Ok(deserialized)
        }
    }
    ///Get a vector pointing away from surface of this field relative to the `point` in `space`
    async fn normal(
        &self,
        space: &impl SpatialRefAspect,
        point: impl Into<stardust_xr::values::Vector3<f32>>,
    ) -> crate::node::NodeResult<stardust_xr::values::Vector3<f32>> {
        {
            let mut _fds = Vec::new();
            let data = (space.node().id(), point.into());
            {
                let (space, point) = &data;
                tracing::trace!(
                    ? space, ? point, "Called method on server, {}::{}", "FieldRef",
                    "normal"
                );
            }
            let result: stardust_xr::values::Vector3<f32> = self
                .node()
                .execute_remote_method(
                    10662923473076663509u64,
                    10933809934326220183u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "FieldRef", "normal"
            );
            Ok(deserialized)
        }
    }
    ///Get the closest point on the surface of this field relative to the `point` in `space`
    async fn closest_point(
        &self,
        space: &impl SpatialRefAspect,
        point: impl Into<stardust_xr::values::Vector3<f32>>,
    ) -> crate::node::NodeResult<stardust_xr::values::Vector3<f32>> {
        {
            let mut _fds = Vec::new();
            let data = (space.node().id(), point.into());
            {
                let (space, point) = &data;
                tracing::trace!(
                    ? space, ? point, "Called method on server, {}::{}", "FieldRef",
                    "closest_point"
                );
            }
            let result: stardust_xr::values::Vector3<f32> = self
                .node()
                .execute_remote_method(
                    10662923473076663509u64,
                    13473947755141124846u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "FieldRef", "closest_point"
            );
            Ok(deserialized)
        }
    }
    ///Get information from the server raymarching the given ray in `space` through this field such as steps, closest/deepest distance, etc.
    async fn ray_march(
        &self,
        space: &impl SpatialRefAspect,
        ray_origin: impl Into<stardust_xr::values::Vector3<f32>>,
        ray_direction: impl Into<stardust_xr::values::Vector3<f32>>,
    ) -> crate::node::NodeResult<RayMarchResult> {
        {
            let mut _fds = Vec::new();
            let data = (space.node().id(), ray_origin.into(), ray_direction.into());
            {
                let (space, ray_origin, ray_direction) = &data;
                tracing::trace!(
                    ? space, ? ray_origin, ? ray_direction,
                    "Called method on server, {}::{}", "FieldRef", "ray_march"
                );
            }
            let result: RayMarchResult = self
                .node()
                .execute_remote_method(
                    10662923473076663509u64,
                    7352457860499612292u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "FieldRef", "ray_march"
            );
            Ok(deserialized)
        }
    }
}
#[allow(clippy::all)]
///An owned field with adjustable shape
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Field(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Field {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Field(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
    pub fn as_field_ref(self) -> super::FieldRef {
        super::FieldRef(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Field {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Field {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl FieldAspect for Field {}
pub(crate) const FIELD_ASPECT_ID: u64 = 3948434400034960392u64;
pub(crate) const FIELD_SET_SHAPE_SERVER_OPCODE: u64 = 10076774457453995458u64;
pub(crate) const FIELD_EXPORT_FIELD_SERVER_OPCODE: u64 = 939650650519133349u64;
#[derive(Debug)]
pub enum FieldEvent {}
#[allow(clippy::all)]
///An owned field with adjustable shape
pub trait FieldAspect: crate::node::NodeType + super::SpatialAspect + super::FieldRefAspect + std::fmt::Debug {
    ///Set the shape of this field (and its parameters)
    fn set_shape(&self, shape: Shape) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (shape);
        self.node()
            .send_remote_signal(
                3948434400034960392u64,
                10076774457453995458u64,
                &data,
                _fds,
            )?;
        let (shape) = data;
        tracing::trace!(? shape, "Sent signal to server, {}::{}", "Field", "set_shape");
        Ok(())
    }
    ///Return a UUID representing this node's FieldRef that you can send to other clients
    async fn export_field(&self) -> crate::node::NodeResult<u64> {
        {
            let mut _fds = Vec::new();
            let data = ();
            {
                let () = &data;
                tracing::trace!(
                    "Called method on server, {}::{}", "Field", "export_field"
                );
            }
            let result: u64 = self
                .node()
                .execute_remote_method(
                    3948434400034960392u64,
                    939650650519133349u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}", "Field",
                "export_field"
            );
            Ok(deserialized)
        }
    }
}
pub(crate) const INTERFACE_IMPORT_FIELD_REF_SERVER_OPCODE: u64 = 5844955584634021418u64;
///Import a FieldRef from a UUID generated by Field::export_field
pub async fn import_field_ref(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    uid: u64,
) -> crate::node::NodeResult<FieldRef> {
    let mut _fds = Vec::new();
    let data = (uid);
    {
        let (uid) = &data;
        tracing::trace!(
            ? uid, "Called method on server, {}::{}", "Interface", "import_field_ref"
        );
    }
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    let message = _client
        .message_sender_handle
        .method(2u64, 0u64, 5844955584634021418u64, &serialized_data, _fds)
        .await?
        .map_err(|e| crate::node::NodeError::ReturnedError {
            e,
        })?
        .into_message();
    let result: u64 = stardust_xr::schemas::flex::deserialize(&message)?;
    let deserialized = FieldRef::from_id(&_client, result, false);
    tracing::trace!(
        "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
        "import_field_ref"
    );
    Ok(deserialized)
}
pub(crate) const INTERFACE_CREATE_FIELD_SERVER_OPCODE: u64 = 3216373392735127623u64;
///Create a field with the shape of a box
fn create_field(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    shape: Shape,
) -> crate::node::NodeResult<Field> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, shape);
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(2u64, 0u64, 3216373392735127623u64, &serialized_data, _fds)?;
        let (id, parent, transform, shape) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? shape, "Sent signal to server, {}::{}",
            "Interface", "create_field"
        );
    }
    Ok(Field::from_id(_client, id, true))
}
