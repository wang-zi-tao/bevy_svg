//! Contains the plugin and its helper types.
//!
//! The [`Svg2dBundle`](crate::bundle::Svg2dBundle) provides a way to display an `SVG`-file
//! with minimal boilerplate.
//!
//! ## How it works
//! The user creates/loades a [`Svg2dBundle`](crate::bundle::Svg2dBundle) in a system.
//!
//! Then, in the [`Set::SVG`](Set::SVG), a mesh is created for each loaded [`Svg`] bundle.
//! Each mesh is then extracted in the [`RenderSet::Extract`](bevy::render::RenderSet) and added to the
//! [`RenderWorld`](bevy::render::RenderWorld).
//! Afterwards it is queued in the [`RenderSet::Queue`](bevy::render::RenderSet) for actual drawing/rendering.
use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin},
    asset::{AssetEvent, Assets},
    ecs::{
        change_detection::DetectChanges,
        event::EventReader,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res},
        world::Ref,
    },
    prelude::{Last, PostUpdate},
};

use crate::{
    origin,
    render::{self, SvgComponent},
    svg::Svg,
};

/// Sets for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Set {
    /// Set in which [`Svg2dBundle`](crate::bundle::Svg2dBundle)s get drawn.
    SVG,
}

/// A plugin that makes sure your [`Svg`]s get rendered
#[derive(Default)]
pub struct SvgRenderPlugin<C: SvgComponent>(PhantomData<C>);

impl<C: SvgComponent> Plugin for SvgRenderPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (origin::add_origin_state::<C>.in_set(Set::SVG),),
        )
        .add_systems(
            Last,
            (
                origin::apply_origin::<C>,
                svg_mesh_linker::<C>.in_set(Set::SVG),
            ),
        );
    }
}

/// Bevy system which queries for all [`Svg`] bundles and adds the correct [`Mesh`] to them.
fn svg_mesh_linker<C: SvgComponent>(
    mut svg_events: EventReader<AssetEvent<Svg>>,
    svgs: Res<Assets<Svg>>,
    mut svg_component: Query<(Ref<C>, &mut C::MeshComponent, &mut C::MaterialComponent)>,
) {
    let changed_handles = svg_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Ensure all correct meshes are set for entities which have had modified handles
    for (svg_component, mut mesh, mut material) in svg_component.iter_mut() {
        if svg_component.is_changed() {
            *material = C::new_material(svg_component.get_handle().clone());
        }
        if changed_handles.contains(&svg_component.get_handle().id()) {
            if let Some(svg) = svgs.get(svg_component.get_handle()) {
                *C::get_mesh_mut(&mut mesh) = svg.mesh.clone();
            }
        }
    }
}
