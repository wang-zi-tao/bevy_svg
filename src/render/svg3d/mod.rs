use bevy::{
    asset::{uuid_handle, Handle},
    prelude::*,
};

mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("c7d158fe-e3de-11f0-87bd-33279dc14325");

pub use plugin::RenderPlugin;

use crate::{origin::Origin, svg::Svg};

use super::{svg_on_insert, SvgComponent};

#[derive(Component, Default)]
#[require(Mesh3d, Origin, Transform, Visibility)]
#[component(on_insert = svg_on_insert::<Svg3d>)]
pub struct Svg3d(pub Handle<Svg>);

impl SvgComponent for Svg3d {
    type MeshComponent = Mesh3d;
    type MaterialComponent = MeshMaterial2d<Svg>;

    fn get_handle(&self) -> &Handle<Svg> {
        &self.0
    }

    fn new_material(svg: Handle<Svg>) -> Self::MaterialComponent {
        MeshMaterial2d(svg)
    }

    fn get_mesh_mut(mesh: &mut Self::MeshComponent) -> &mut Handle<Mesh> {
        &mut mesh.0
    }
}
