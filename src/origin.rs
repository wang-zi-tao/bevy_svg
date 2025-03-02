use bevy::{
    prelude::*,
    asset::Assets,
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{DetectChanges, Ref},
    transform::components::{GlobalTransform, Transform},
};

use crate::{render::SvgComponent, svg::Svg};

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
/// Origin of the coordinate system.
pub enum Origin {
    /// Bottom left of the image or viewbox.
    BottomLeft,
    /// Bottom right of the image or viewbox.
    BottomRight,
    /// Center of the image or viewbox.
    Center,
    #[default]
    /// Top left of the image or viewbox, this is the default for a SVG.
    TopLeft,
    /// Top right of the image or viewbox.
    TopRight,
}

impl Origin {
    /// Computes the translation for an origin. The resulting translation needs to be added
    /// to the translation of the SVG.
    pub fn compute_translation(&self, scaled_size: Vec2) -> Vec3 {
        match self {
            Origin::BottomLeft => Vec3::new(0.0, scaled_size.y, 0.0),
            Origin::BottomRight => Vec3::new(-scaled_size.x, scaled_size.y, 0.0),
            Origin::Center => Vec3::new(-scaled_size.x * 0.5, scaled_size.y * 0.5, 0.0),
            // Standard SVG origin is top left, so we don't need to do anything
            Origin::TopLeft => Vec3::ZERO,
            Origin::TopRight => Vec3::new(-scaled_size.x, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub(crate) struct OriginState {
    previous: Origin,
}

/// Checkes if a "new" SVG bundle was added by looking for a missing OriginState
/// and then adds it to the entity.
pub(crate) fn add_origin_state<C: SvgComponent>(
    mut commands: Commands,
    query: Query<Entity, (With<C>, With<C::MeshComponent>, Without<OriginState>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(OriginState {
            previous: Origin::default(),
        });
    }
}

/// Gets all SVGs with a changed origin or transform and checks if the origin offset
/// needs to be applied.
pub(crate) fn apply_origin<C: SvgComponent>(
    svgs: Res<Assets<Svg>>,
    mut query: Query<
        (
            Entity,
            &C,
            &Origin,
            &mut OriginState,
            Ref<Transform>,
            &mut GlobalTransform,
        ),
        Or<(
            Changed<Origin>,
            Changed<Transform>,
            Changed<C::MeshComponent>,
        )>,
    >,
) {
    for (_, svg_component, origin, mut origin_state, transform, mut global_transform) in &mut query
    {
        if let Some(svg) = svgs.get(svg_component.get_handle()) {
            if origin_state.previous != *origin {
                let scaled_size = svg.size * transform.scale.xy();
                let reverse_origin_translation =
                    origin_state.previous.compute_translation(scaled_size);
                let origin_translation = origin.compute_translation(scaled_size);

                let mut gtransf = global_transform.compute_transform();
                gtransf.translation.x += origin_translation.x - reverse_origin_translation.x;
                gtransf.translation.y += origin_translation.y - reverse_origin_translation.y;
                gtransf.translation.z += origin_translation.z - reverse_origin_translation.z;
                *global_transform = GlobalTransform::from(gtransf);

                origin_state.previous = origin.clone();
            } else if transform.is_changed() {
                let scaled_size = svg.size * transform.scale.xy();
                let origin_translation = origin.compute_translation(scaled_size);

                let mut gtransf = global_transform.compute_transform();
                gtransf.translation.x += origin_translation.x;
                gtransf.translation.y += origin_translation.y;
                gtransf.translation.z += origin_translation.z;
                *global_transform = GlobalTransform::from(gtransf);
            }
        }
    }
}
