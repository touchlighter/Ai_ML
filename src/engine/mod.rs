use anyhow::Result;
use log::{info, warn};
use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod state;
mod time;

pub use state::EngineState;
pub use time::TimeManager;

use crate::rendering::Renderer;
use crate::input::InputManager;
use crate::world::World;
use crate::game::GameManager;
use crate::audio::AudioManager;
use crate::ui::UIManager;

pub struct Engine {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    state: EngineState,
    time_manager: TimeManager,
}

impl Engine {
    pub fn new() -> Result<Self> {
        info!("Initializing Engine");
        
        // Create event loop and window
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Minecraft Clone - Rust Edition")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_min_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)?;

        // Initialize engine state
        let state = EngineState::new(&window)?;
        let time_manager = TimeManager::new();

        Ok(Self {
            event_loop: Some(event_loop),
            window,
            state,
            time_manager,
        })
    }

    pub fn run(mut self) -> Result<()> {
        info!("Starting main game loop");
        
        let event_loop = self.event_loop.take().unwrap();
        
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            
            match event {
                Event::WindowEvent { 
                    ref event, 
                    window_id 
                } if window_id == self.window.id() => {
                    
                    // Let UI handle input first
                    if !self.state.ui_manager.handle_event(&self.window, event) {
                        // Then handle game input
                        self.state.input_manager.handle_event(event);
                        
                        match event {
                            WindowEvent::CloseRequested => {
                                info!("Close requested");
                                elwt.exit();
                            },
                            WindowEvent::Resized(physical_size) => {
                                info!("Window resized: {:?}", physical_size);
                                self.state.renderer.resize(*physical_size);
                            },
                            WindowEvent::RedrawRequested => {
                                // Update game state
                                self.update();
                                
                                // Render frame
                                match self.render() {
                                    Ok(_) => {},
                                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                        self.state.renderer.resize(self.state.renderer.size());
                                    },
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        warn!("Out of memory!");
                                        elwt.exit();
                                    },
                                    Err(e) => {
                                        eprintln!("Render error: {:?}", e);
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                },
                Event::AboutToWait => {
                    // Request redraw for continuous rendering
                    self.window.request_redraw();
                },
                _ => {}
            }
        })?;
        
        Ok(())
    }

    fn update(&mut self) {
        // Update time
        self.time_manager.update();
        let delta_time = self.time_manager.delta_time();
        
        // Update all systems
        self.state.input_manager.update();
        self.state.game_manager.update(delta_time);
        self.state.world.update(delta_time);
        self.state.audio_manager.update();
        self.state.ui_manager.update(delta_time);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Prepare UI
        self.state.ui_manager.prepare(&self.window);
        
        // Render the frame
        self.state.renderer.render(
            &self.state.world,
            &self.state.game_manager,
            &mut self.state.ui_manager,
        )
    }
}