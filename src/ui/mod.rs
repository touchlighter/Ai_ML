use anyhow::Result;
use egui_wgpu::Renderer;
use egui_winit::State;
use winit::{event::WindowEvent, window::Window};

/// UI manager using egui for immediate mode GUI
pub struct UIManager {
    context: egui::Context,
    state: State,
    renderer: Renderer,
}

impl UIManager {
    pub fn new(renderer: &crate::rendering::Renderer) -> Self {
        let context = egui::Context::default();
        let state = State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            renderer.device(),
            None,
            Some(1.0),
        );
        
        let egui_renderer = Renderer::new(
            renderer.device(),
            wgpu::TextureFormat::Bgra8UnormSrgb, // TODO: Use correct format
            None,
            1,
        );

        Self {
            context,
            state,
            renderer: egui_renderer,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.state.on_window_event(window, event);
        response.consumed
    }

    pub fn update(&mut self, _delta_time: f32) {
        // Update UI state
    }

    pub fn prepare(&mut self, window: &Window) {
        let raw_input = self.state.take_egui_input(window);
        self.context.begin_frame(raw_input);
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // End the frame and get the output
        let output = self.context.end_frame();
        
        // Handle platform output
        self.state.handle_platform_output(
            &winit::window::Window::default(), // TODO: Pass actual window
            output.platform_output,
        );

        // Render UI
        let paint_jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        
        // Create screen descriptor
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [1280, 720], // TODO: Get actual size
            pixels_per_point: output.pixels_per_point,
        };

        // Update textures
        for (id, image_delta) in &output.textures_delta.set {
            self.renderer.update_texture(device, queue, *id, image_delta);
        }

        // Record render commands
        self.renderer.render(
            encoder,
            view,
            &paint_jobs,
            &screen_descriptor,
        );

        // Free textures
        for id in &output.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }

    pub fn show_debug_window(&mut self, game_manager: &crate::game::GameManager, world: &crate::world::World) {
        if game_manager.is_debug_mode() {
            egui::Window::new("Debug Info")
                .default_open(true)
                .resizable(true)
                .show(&self.context, |ui| {
                    ui.label("Debug Information");
                    ui.separator();
                    
                    ui.label(format!("Game Mode: {:?}", game_manager.game_mode()));
                    ui.label(format!("Player Position: {:?}", game_manager.player().position()));
                    ui.label(format!("Loaded Chunks: {}", world.loaded_chunks().len()));
                    ui.label(format!("Render Distance: {}", world.render_distance()));
                    
                    if let Some(target) = game_manager.breaking_target() {
                        ui.label(format!("Breaking Block: {:?}", target));
                        ui.label(format!("Breaking Progress: {:.1}%", game_manager.breaking_progress() * 100.0));
                    }
                });
        }
    }

    pub fn show_hotbar(&mut self, game_manager: &crate::game::GameManager) {
        let hotbar = game_manager.player().inventory().hotbar();
        let selected_slot = game_manager.player().selected_hotbar_slot();

        egui::Area::new("hotbar")
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -20.0))
            .show(&self.context, |ui| {
                ui.horizontal(|ui| {
                    for (i, item) in hotbar.iter().enumerate() {
                        let is_selected = i == selected_slot;
                        
                        let color = if is_selected {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::GRAY
                        };

                        ui.scope(|ui| {
                            ui.visuals_mut().widgets.inactive.bg_fill = color;
                            ui.visuals_mut().widgets.hovered.bg_fill = color;
                            ui.visuals_mut().widgets.active.bg_fill = color;

                            let button_text = if item.is_empty() {
                                format!("{}", i + 1)
                            } else {
                                format!("{}\n{}", item.item_type.name(), item.count)
                            };

                            ui.button(button_text);
                        });
                    }
                });
            });
    }

    pub fn show_crosshair(&mut self) {
        let screen_center = self.context.screen_rect().center();
        
        egui::Area::new("crosshair")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(&self.context, |ui| {
                ui.painter().text(
                    screen_center,
                    egui::Align2::CENTER_CENTER,
                    "+",
                    egui::FontId::proportional(20.0),
                    egui::Color32::WHITE,
                );
            });
    }
}