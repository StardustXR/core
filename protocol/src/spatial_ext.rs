use crate::protocol::spatial::{PartialTransform, Transform};
use crate::protocol::types::{Quatf, Vec3F};

impl Transform {
    pub const IDENTITY: Transform = Transform {
        translation: Vec3F {
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
        scale: Vec3F {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    };

    pub fn from_translation(translation: impl Into<Vec3F>) -> Self {
        Transform {
            translation: translation.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_rotation(rotation: impl Into<Quatf>) -> Self {
        Transform {
            rotation: rotation.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_scale(scale: impl Into<Vec3F>) -> Self {
        Transform {
            scale: scale.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_translation_rotation(
        translation: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
    ) -> Self {
        Transform {
            translation: translation.into(),
            rotation: rotation.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_translation_scale(translation: impl Into<Vec3F>, scale: impl Into<Vec3F>) -> Self {
        Transform {
            translation: translation.into(),
            scale: scale.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_rotation_scale(rotation: impl Into<Quatf>, scale: impl Into<Vec3F>) -> Self {
        Transform {
            rotation: rotation.into(),
            scale: scale.into(),
            ..Self::IDENTITY
        }
    }
    pub fn from_translation_rotation_scale(
        translation: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
        scale: impl Into<Vec3F>,
    ) -> Self {
        Transform {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}

impl PartialTransform {
    pub const NONE: PartialTransform = PartialTransform {
        translation: None,
        rotation: None,
        scale: None,
    };

    pub fn from_translation(translation: impl Into<Vec3F>) -> Self {
        PartialTransform {
            translation: Some(translation.into()),
            ..Self::NONE
        }
    }
    pub fn from_rotation(rotation: impl Into<Quatf>) -> Self {
        PartialTransform {
            rotation: Some(rotation.into()),
            ..Self::NONE
        }
    }
    pub fn from_scale(scale: impl Into<Vec3F>) -> Self {
        PartialTransform {
            scale: Some(scale.into()),
            ..Self::NONE
        }
    }
    pub fn from_translation_rotation(
        translation: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
    ) -> Self {
        PartialTransform {
            translation: Some(translation.into()),
            rotation: Some(rotation.into()),
            ..Self::NONE
        }
    }
    pub fn from_translation_scale(translation: impl Into<Vec3F>, scale: impl Into<Vec3F>) -> Self {
        PartialTransform {
            translation: Some(translation.into()),
            scale: Some(scale.into()),
            ..Self::NONE
        }
    }
    pub fn from_rotation_scale(rotation: impl Into<Quatf>, scale: impl Into<Vec3F>) -> Self {
        PartialTransform {
            rotation: Some(rotation.into()),
            scale: Some(scale.into()),
            ..Self::NONE
        }
    }
    pub fn from_translation_rotation_scale(
        translation: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
        scale: impl Into<Vec3F>,
    ) -> Self {
        PartialTransform {
            translation: Some(translation.into()),
            rotation: Some(rotation.into()),
            scale: Some(scale.into()),
        }
    }
}

impl From<Transform> for PartialTransform {
    fn from(t: Transform) -> Self {
        PartialTransform {
            translation: Some(t.translation),
            rotation: Some(t.rotation),
            scale: Some(t.scale),
        }
    }
}
