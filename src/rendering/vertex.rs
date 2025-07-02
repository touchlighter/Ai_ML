use bytemuck::{Pod, Zeroable};
use wgpu::VertexAttribute;

/// Generic vertex trait for all vertex types
pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

/// Vertex for rendering blocks with texture coordinates and lighting
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BlockVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
    texture_id: u32,
    light_level: f32,
}

impl BlockVertex {
    pub fn new(
        position: [f32; 3],
        tex_coords: [f32; 2],
        normal: [f32; 3],
        texture_id: u32,
        light_level: f32,
    ) -> Self {
        Self {
            position,
            tex_coords,
            normal,
            texture_id,
            light_level,
        }
    }
}

impl Vertex for BlockVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<BlockVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Texture coordinates
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Normal
                VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Texture ID
                VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
                // Light level
                VertexAttribute {
                    offset: (mem::size_of::<[f32; 8]>() + mem::size_of::<u32>()) as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Face directions for cube faces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

impl Face {
    pub fn normal(&self) -> [f32; 3] {
        match self {
            Face::Top => [0.0, 1.0, 0.0],
            Face::Bottom => [0.0, -1.0, 0.0],
            Face::Front => [0.0, 0.0, 1.0],
            Face::Back => [0.0, 0.0, -1.0],
            Face::Left => [-1.0, 0.0, 0.0],
            Face::Right => [1.0, 0.0, 0.0],
        }
    }

    pub fn vertices(&self, x: f32, y: f32, z: f32, texture_id: u32, light_level: f32) -> [BlockVertex; 4] {
        let normal = self.normal();
        match self {
            Face::Top => [
                BlockVertex::new([x, y + 1.0, z], [0.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y + 1.0, z], [1.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y + 1.0, z + 1.0], [1.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x, y + 1.0, z + 1.0], [0.0, 1.0], normal, texture_id, light_level),
            ],
            Face::Bottom => [
                BlockVertex::new([x, y, z + 1.0], [0.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y, z + 1.0], [1.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y, z], [1.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x, y, z], [0.0, 1.0], normal, texture_id, light_level),
            ],
            Face::Front => [
                BlockVertex::new([x, y, z + 1.0], [0.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x, y + 1.0, z + 1.0], [0.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y + 1.0, z + 1.0], [1.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y, z + 1.0], [1.0, 0.0], normal, texture_id, light_level),
            ],
            Face::Back => [
                BlockVertex::new([x + 1.0, y, z], [0.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y + 1.0, z], [0.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x, y + 1.0, z], [1.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x, y, z], [1.0, 0.0], normal, texture_id, light_level),
            ],
            Face::Left => [
                BlockVertex::new([x, y, z], [0.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x, y + 1.0, z], [0.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x, y + 1.0, z + 1.0], [1.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x, y, z + 1.0], [1.0, 0.0], normal, texture_id, light_level),
            ],
            Face::Right => [
                BlockVertex::new([x + 1.0, y, z + 1.0], [0.0, 0.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y + 1.0, z + 1.0], [0.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y + 1.0, z], [1.0, 1.0], normal, texture_id, light_level),
                BlockVertex::new([x + 1.0, y, z], [1.0, 0.0], normal, texture_id, light_level),
            ],
        }
    }

    pub fn indices(&self, start_vertex: u32) -> [u32; 6] {
        [
            start_vertex,
            start_vertex + 1,
            start_vertex + 2,
            start_vertex,
            start_vertex + 2,
            start_vertex + 3,
        ]
    }

    pub fn all() -> [Face; 6] {
        [Face::Top, Face::Bottom, Face::Front, Face::Back, Face::Left, Face::Right]
    }
}

/// Mesh data for a chunk
pub struct ChunkMesh {
    pub vertices: Vec<BlockVertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: u32,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_buffer: None,
            index_buffer: None,
            index_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.index_count = 0;
    }

    pub fn add_face(&mut self, face: Face, x: f32, y: f32, z: f32, texture_id: u32, light_level: f32) {
        let start_vertex = self.vertices.len() as u32;
        let face_vertices = face.vertices(x, y, z, texture_id, light_level);
        let face_indices = face.indices(start_vertex);

        self.vertices.extend_from_slice(&face_vertices);
        self.indices.extend_from_slice(&face_indices);
        self.index_count += 6;
    }

    pub fn finalize(&mut self, device: &wgpu::Device) {
        use wgpu::util::DeviceExt;

        if !self.vertices.is_empty() {
            self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }));

            self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            }));
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let (Some(vertex_buffer), Some(index_buffer)) = (&self.vertex_buffer, &self.index_buffer) {
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.index_count, 0, 0..1);
        }
    }
}

impl Default for ChunkMesh {
    fn default() -> Self {
        Self::new()
    }
}