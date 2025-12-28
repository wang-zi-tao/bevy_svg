use bevy::{
    asset::RenderAssetUsages, color::{Color, ColorToComponents as _}, math::Vec3, mesh::{Indices, Mesh}, render::render_resource::PrimitiveTopology, transform::components::Transform
};
use copyless::VecHelper as _;
use lyon_tessellation::{
    self, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};

use crate::Convert;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
pub type IndexType = u32;

/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub type VertexBuffers = lyon_tessellation::VertexBuffers<Vertex, IndexType>;

impl Convert<Mesh> for VertexBuffers {
    fn convert(self) -> Mesh {
        let mut positions = Vec::with_capacity(self.vertices.len());
        let mut colors = Vec::with_capacity(self.vertices.len());

        for vert in self.vertices {
            positions.alloc().init(vert.position);
            colors.alloc().init(vert.color);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U32(self.indices));

        mesh
    }
}

/// Zero-sized type used to implement various vertex construction traits from Lyon.
pub struct VertexConstructor {
    pub(crate) color: Color,
    pub(crate) transform: Transform,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let vertex = vertex.position();
        let pos = self.transform * Vec3::new(vertex.x, vertex.y, 0.0);

        Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.color.to_linear().to_f32_array(),
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let vertex = vertex.position();
        let pos = self.transform * Vec3::new(vertex.x, vertex.y, 0.0);

        Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.color.to_srgba().to_f32_array(),
        }
    }
}

pub trait BufferExt<A> {
    fn extend_one(&mut self, item: A);
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}

impl BufferExt<Self> for VertexBuffers {
    fn extend_one(&mut self, item: Self) {
        let offset = self.vertices.len() as u32;

        for vert in item.vertices {
            self.vertices.alloc().init(vert);
        }
        for idx in item.indices {
            self.indices.alloc().init(idx + offset);
        }
    }

    fn extend<T: IntoIterator<Item = Self>>(&mut self, iter: T) {
        let mut offset = self.vertices.len() as u32;

        for buf in iter {
            let num_verts = buf.vertices.len() as u32;
            for vert in buf.vertices {
                self.vertices.alloc().init(vert);
            }
            for idx in buf.indices {
                self.indices.alloc().init(idx + offset);
            }
            offset += num_verts;
        }
    }
}
