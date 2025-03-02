use bevy::{asset::Handle, prelude::*, render::render_resource::Shader};

mod bundle;
mod plugin;

/// Handle to the custom shader with a unique random ID
pub const SVG_3D_SHADER_HANDLE:  Handle<Shader> = Handle::weak_from_u128(8_514_826_640_451_853_414);

pub use bundle::Svg3dBundle;
pub use plugin::RenderPlugin;

use crate::{origin::Origin, svg::Svg};

use super::{svg_on_insert, SvgComponent};

#[derive(Component, Default)]
#[require(Mesh3d, Origin, MeshMaterial3d<Svg>)]
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
