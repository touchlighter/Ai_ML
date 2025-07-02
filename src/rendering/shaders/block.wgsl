// Vertex shader inputs
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) texture_id: u32,
    @location(4) light_level: f32,
}

// Vertex shader outputs / Fragment shader inputs
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
    @location(3) texture_id: u32,
    @location(4) light_level: f32,
}

// Uniform buffer for camera
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Texture atlas
@group(1) @binding(0)
var texture_atlas: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

// Vertex shader
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    out.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
    out.tex_coords = input.tex_coords;
    out.world_normal = input.normal;
    out.world_position = input.position;
    out.texture_id = input.texture_id;
    out.light_level = input.light_level;
    
    return out;
}

// Fragment shader
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate texture coordinates in atlas
    // For now, assume 16x16 texture atlas (256 textures total)
    let atlas_size = 16.0;
    let texture_size = 1.0 / atlas_size;
    
    let tex_x = f32(input.texture_id % 16u);
    let tex_y = f32(input.texture_id / 16u);
    
    let atlas_coords = vec2<f32>(
        (tex_x + input.tex_coords.x) * texture_size,
        (tex_y + input.tex_coords.y) * texture_size
    );
    
    // Sample the texture
    var color = textureSample(texture_atlas, texture_sampler, atlas_coords);
    
    // Basic lighting calculation
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3)); // Sun direction
    let ambient = 0.3;
    let diffuse = max(dot(input.world_normal, light_dir), 0.0) * 0.7;
    let lighting = ambient + diffuse;
    
    // Apply lighting and block light level
    color = color * lighting * input.light_level;
    
    // Fog calculation
    let distance = length(camera.view_pos.xyz - input.world_position);
    let fog_start = 80.0;
    let fog_end = 120.0;
    let fog_factor = clamp((distance - fog_start) / (fog_end - fog_start), 0.0, 1.0);
    let fog_color = vec3<f32>(0.5, 0.8, 1.0); // Sky blue
    
    color = vec4<f32>(mix(color.rgb, fog_color, fog_factor), color.a);
    
    return color;
}