mod plugin;
pub(crate) mod tessellation;
mod vertex_buffer;
use crate::svg::Svg;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

#[cfg(feature = "2d")]
pub(crate) mod svg2d;
#[cfg(feature = "3d")]
pub(crate) mod svg3d;

pub use plugin::SvgPlugin;

pub(crate) trait SvgComponent: Component {
    type MeshComponent: Component;
    type MaterialComponent: Component;

    fn get_handle(&self) -> &Handle<Svg>;
    fn new_material(svg: Handle<Svg>) -> Self::MaterialComponent;
    fn get_mesh_mut(mesh: &mut Self::MeshComponent) -> &mut Handle<Mesh>;
}

fn svg_on_insert<C: SvgComponent>(
    mut world: DeferredWorld,
    entity: Entity,
    _component_id: ComponentId,
) {
    let component = world.entity(entity).get_components::<&C>().unwrap();
    let handle = component.get_handle().clone();
    let entity = world.entity(entity).id();
    let mut commands = world.commands();
    commands.entity(entity).insert(C::new_material(handle));
}
