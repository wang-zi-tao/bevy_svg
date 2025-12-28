use bevy::{
    asset::{uuid_handle, Handle},
    prelude::*,
};

mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_2D_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("18acc772-e3df-11f0-b65f-5bae1d415e34");

pub use plugin::RenderPlugin;

use crate::{origin::Origin, svg::Svg};

use super::{svg_on_insert, SvgComponent};

#[derive(Component, Default)]
#[require(Mesh2d, Origin, Transform, Visibility)]
#[component(on_insert = svg_on_insert::<Svg2d>)]
pub struct Svg2d(pub Handle<Svg>);

impl SvgComponent for Svg2d {
    type MeshComponent = Mesh2d;
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
