use anyhow::Result;
use egui_wgpu::Renderer;
use egui_winit::State;
use winit::{event::WindowEvent, window::Window};

/// UI manager using egui for immediate mode GUI
pub struct UIManager {
    pub ctx: egui::Context,
    pub state: State,
    pub renderer: Renderer,
}

impl UIManager {
    pub fn new(
        device: &wgpu::Device,
        output_color_format: wgpu::TextureFormat,
        output_depth_format: Option<wgpu::TextureFormat>,
        msaa_samples: u32,
        window: &Window,
    ) -> Self {
        let ctx = egui::Context::default();
        
        let egui_state = egui_winit::State::new(
            ctx.clone(),
            egui::viewport::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
        );

        let egui_renderer = egui_wgpu::Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
        );

        Self {
            ctx,
            state: egui_state,
            renderer: egui_renderer,
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        let response = self.state.on_window_event(window, event);
        response.consumed
    }

    pub fn prepare(&mut self, window: &Window) -> Vec<egui::ClippedPrimitive> {
        let raw_input = self.state.take_egui_input(window);
        
        // Run UI rendering in a closure
        let (shapes, platform_output) = {
            let full_output = self.ctx.run(raw_input, |ctx| {
                // Render debug window
                egui::Window::new("Debug Info")
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("FPS: 60"); // TODO: Calculate actual FPS
                        ui.label("Position: (0, 0, 0)"); // TODO: Get actual position
                        ui.label("Chunks loaded: 0"); // TODO: Get actual chunk count
                    });

                // Render hotbar
                egui::Area::new(egui::Id::new("hotbar"))
                    .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::new(0.0, -20.0))
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            for i in 0..9 {
                                let selected = i == 0; // TODO: Get actual selected slot
                                let bg_color = if selected {
                                    egui::Color32::LIGHT_GRAY
                                } else {
                                    egui::Color32::DARK_GRAY
                                };
                                
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::Vec2::splat(40.0),
                                    egui::Sense::click()
                                );
                                
                                ui.painter().rect_filled(rect, 2.0, bg_color);
                                ui.painter().rect_stroke(rect, 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
                            }
                        });
                    });

                // Render crosshair
                egui::Area::new(egui::Id::new("crosshair"))
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ctx, |ui| {
                        let size = 20.0;
                        let thickness = 2.0;
                        let color = egui::Color32::WHITE;
                        
                        let center = ui.available_rect_before_wrap().center();
                        let painter = ui.painter();
                        
                        // Horizontal line
                        painter.line_segment(
                            [center + egui::Vec2::new(-size/2.0, 0.0), center + egui::Vec2::new(size/2.0, 0.0)],
                            egui::Stroke::new(thickness, color)
                        );
                        
                        // Vertical line
                        painter.line_segment(
                            [center + egui::Vec2::new(0.0, -size/2.0), center + egui::Vec2::new(0.0, size/2.0)],
                            egui::Stroke::new(thickness, color)
                        );
                    });
            });
            (full_output.shapes, full_output.platform_output)
        };
        
        self.state.handle_platform_output(window, platform_output);
        
        let primitives = self.ctx.tessellate(shapes, self.ctx.pixels_per_point());
        primitives
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        primitives: Vec<egui::ClippedPrimitive>,
        screen_descriptor: &egui_wgpu::ScreenDescriptor,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // Create render pass
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Render UI
        self.renderer.render(&mut render_pass, &primitives, screen_descriptor);
    }
}