use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Shader manager for loading, compiling, and hot-reloading shaders
pub struct ShaderManager {
    shaders: HashMap<String, wgpu::ShaderModule>,
    device: wgpu::Device,
}

impl ShaderManager {
    pub fn new(device: wgpu::Device) -> Self {
        Self {
            shaders: HashMap::new(),
            device,
        }
    }

    pub fn load_shader(&mut self, name: &str, source: &str) -> Result<&wgpu::ShaderModule> {
        let shader_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        self.shaders.insert(name.to_string(), shader_module);
        Ok(self.shaders.get(name).unwrap())
    }

    pub fn load_shader_from_file(&mut self, name: &str, path: &Path) -> Result<&wgpu::ShaderModule> {
        let source = std::fs::read_to_string(path)?;
        self.load_shader(name, &source)
    }

    pub fn get_shader(&self, name: &str) -> Option<&wgpu::ShaderModule> {
        self.shaders.get(name)
    }

    pub fn reload_shader(&mut self, name: &str, source: &str) -> Result<()> {
        self.load_shader(name, source)?;
        Ok(())
    }

    // TODO: Implement hot-reloading with file watching
    pub fn check_for_changes(&mut self) -> Result<Vec<String>> {
        // Placeholder for hot-reload functionality
        Ok(Vec::new())
    }
}