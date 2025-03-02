//! Bevy [`Bundle`] representing an SVG entity.

use bevy::{
    ecs::bundle::Bundle,
    render::{
        mesh::Mesh2d,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    transform::components::{GlobalTransform, Transform},
};

use crate::{origin::Origin, render::svg2d::Svg2d};

/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct Svg3dBundle {
    pub svg: Svg2d,
    pub mesh: Mesh2d,
    /// [`Origin`] of the coordinate system and as such the origin for the Bevy position.
    pub origin: Origin,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl Default for Svg3dBundle {
    /// Creates a default [`Svg3dBundle`].
    fn default() -> Self {
        Self {
            svg: Default::default(),
            mesh: Default::default(),
            origin: Default::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
