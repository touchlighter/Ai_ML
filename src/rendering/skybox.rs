use wgpu::util::DeviceExt;
use crate::rendering::vertex::{Vertex, BlockVertex};

/// Skybox renderer for drawing the sky background
pub struct Skybox {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Skybox {
    pub fn new(device: &wgpu::Device) -> Self {
        // Create a large cube that surrounds the world
        let vertices = Self::create_skybox_vertices();
        let indices = Self::create_skybox_indices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }

    fn create_skybox_vertices() -> Vec<BlockVertex> {
        let size = 500.0; // Large cube
        let texture_id = 255; // Special texture ID for sky
        let light_level = 1.0;

        vec![
            // Front face
            BlockVertex::new([-size, -size,  size], [0.0, 0.0], [0.0, 0.0, 1.0], texture_id, light_level),
            BlockVertex::new([ size, -size,  size], [1.0, 0.0], [0.0, 0.0, 1.0], texture_id, light_level),
            BlockVertex::new([ size,  size,  size], [1.0, 1.0], [0.0, 0.0, 1.0], texture_id, light_level),
            BlockVertex::new([-size,  size,  size], [0.0, 1.0], [0.0, 0.0, 1.0], texture_id, light_level),

            // Back face
            BlockVertex::new([ size, -size, -size], [0.0, 0.0], [0.0, 0.0, -1.0], texture_id, light_level),
            BlockVertex::new([-size, -size, -size], [1.0, 0.0], [0.0, 0.0, -1.0], texture_id, light_level),
            BlockVertex::new([-size,  size, -size], [1.0, 1.0], [0.0, 0.0, -1.0], texture_id, light_level),
            BlockVertex::new([ size,  size, -size], [0.0, 1.0], [0.0, 0.0, -1.0], texture_id, light_level),

            // Left face
            BlockVertex::new([-size, -size, -size], [0.0, 0.0], [-1.0, 0.0, 0.0], texture_id, light_level),
            BlockVertex::new([-size, -size,  size], [1.0, 0.0], [-1.0, 0.0, 0.0], texture_id, light_level),
            BlockVertex::new([-size,  size,  size], [1.0, 1.0], [-1.0, 0.0, 0.0], texture_id, light_level),
            BlockVertex::new([-size,  size, -size], [0.0, 1.0], [-1.0, 0.0, 0.0], texture_id, light_level),

            // Right face
            BlockVertex::new([ size, -size,  size], [0.0, 0.0], [1.0, 0.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size, -size, -size], [1.0, 0.0], [1.0, 0.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size,  size, -size], [1.0, 1.0], [1.0, 0.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size,  size,  size], [0.0, 1.0], [1.0, 0.0, 0.0], texture_id, light_level),

            // Top face
            BlockVertex::new([-size,  size,  size], [0.0, 0.0], [0.0, 1.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size,  size,  size], [1.0, 0.0], [0.0, 1.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size,  size, -size], [1.0, 1.0], [0.0, 1.0, 0.0], texture_id, light_level),
            BlockVertex::new([-size,  size, -size], [0.0, 1.0], [0.0, 1.0, 0.0], texture_id, light_level),

            // Bottom face
            BlockVertex::new([-size, -size, -size], [0.0, 0.0], [0.0, -1.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size, -size, -size], [1.0, 0.0], [0.0, -1.0, 0.0], texture_id, light_level),
            BlockVertex::new([ size, -size,  size], [1.0, 1.0], [0.0, -1.0, 0.0], texture_id, light_level),
            BlockVertex::new([-size, -size,  size], [0.0, 1.0], [0.0, -1.0, 0.0], texture_id, light_level),
        ]
    }

    fn create_skybox_indices() -> Vec<u32> {
        vec![
            // Front face
            0, 1, 2, 2, 3, 0,
            // Back face
            4, 5, 6, 6, 7, 4,
            // Left face
            8, 9, 10, 10, 11, 8,
            // Right face
            12, 13, 14, 14, 15, 12,
            // Top face
            16, 17, 18, 18, 19, 16,
            // Bottom face
            20, 21, 22, 22, 23, 20,
        ]
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}