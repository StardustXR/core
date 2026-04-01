use crate::protocol::spatial::{PartialTransform, Transform};
use crate::protocol::types::{Quatf, Vec3F};

impl Transform {
    pub const IDENTITY: Transform = Transform {
        position: Vec3F {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quatf {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
        scale: 1.0,
    };

    pub fn from_position(position: impl Into<Vec3F>) -> Self {
        Transform {
            position: position.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_rotation(rotation: impl Into<Quatf>) -> Self {
        Transform {
            rotation: rotation.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_scale(scale: f32) -> Self {
        Transform {
            scale,
            ..Self::IDENTITY
        }
    }
    pub fn from_position_rotation(
        position: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
    ) -> Self {
        Transform {
            position: position.into(),
            rotation: rotation.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_position_scale(position: impl Into<Vec3F>, scale: f32) -> Self {
        Transform {
            position: position.into(),
            scale,
            ..Self::IDENTITY
        }
    }
    pub fn from_rotation_scale(rotation: impl Into<Quatf>, scale: f32) -> Self {
        Transform {
            rotation: rotation.into(),
            scale,
            ..Self::IDENTITY
        }
    }
    pub fn from_position_rotation_scale(
        position: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
        scale: f32,
    ) -> Self {
        Transform {
            position: position.into(),
            rotation: rotation.into(),
            scale,
        }
    }
}

impl PartialTransform {
    pub const NONE: PartialTransform = PartialTransform {
        position: None,
        rotation: None,
        scale: None,
    };

    pub fn from_position(position: impl Into<Vec3F>) -> Self {
        PartialTransform {
            position: Some(position.into()),
            ..Self::NONE
        }
    }
    pub fn from_rotation(rotation: impl Into<Quatf>) -> Self {
        PartialTransform {
            rotation: Some(rotation.into()),
            ..Self::NONE
        }
    }
    pub fn from_scale(scale: f32) -> Self {
        PartialTransform {
            scale: Some(scale),
            ..Self::NONE
        }
    }
    pub fn from_position_rotation(
        position: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
    ) -> Self {
        PartialTransform {
            position: Some(position.into()),
            rotation: Some(rotation.into()),
            ..Self::NONE
        }
    }
    pub fn from_position_scale(position: impl Into<Vec3F>, scale: f32) -> Self {
        PartialTransform {
            position: Some(position.into()),
            scale: Some(scale),
            ..Self::NONE
        }
    }
    pub fn from_rotation_scale(rotation: impl Into<Quatf>, scale: f32) -> Self {
        PartialTransform {
            rotation: Some(rotation.into()),
            scale: Some(scale),
            ..Self::NONE
        }
    }
    pub fn from_position_rotation_scale(
        position: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
        scale: f32,
    ) -> Self {
        PartialTransform {
            position: Some(position.into()),
            rotation: Some(rotation.into()),
            scale: Some(scale),
        }
    }
}

impl From<Transform> for PartialTransform {
    fn from(t: Transform) -> Self {
        PartialTransform {
            position: Some(t.position),
            rotation: Some(t.rotation),
            scale: Some(t.scale),
        }
    }
}
