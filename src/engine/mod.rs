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
    pub window: Window,
    pub state: EngineState,
    pub time_manager: TimeManager,
}

impl Engine {
    pub async fn new() -> Result<Self> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Minecraft Clone")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)?;

        // Create state asynchronously
        let state = EngineState::new(&window).await?;
        let time_manager = TimeManager::new();

        Ok(Self {
            window,
            state,
            time_manager,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.run(move |event, target| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    
                    // Let UI handle input first
                    if !self.state.ui_manager.handle_input(&self.window, event) {
                        // Then handle game input
                        self.state.input_manager.handle_event(event);
                        
                        match event {
                            WindowEvent::CloseRequested => target.exit(),
                            WindowEvent::Resized(physical_size) => {
                                if let Err(e) = self.state.renderer.resize(*physical_size) {
                                    eprintln!("Resize error: {}", e);
                                }
                            }
                            WindowEvent::RedrawRequested => {
                                self.update();
                                
                                if let Err(e) = self.render() {
                                    eprintln!("Render error: {}", e);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::AboutToWait => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        })?;
        
        Ok(())
    }

    fn update(&mut self) {
        // Update time
        self.time_manager.update();
        let delta_time = self.time_manager.delta_time();
        
        // Update game systems
        self.state.input_manager.update();
        self.state.game_manager.update(delta_time);
        self.state.world.update(delta_time);
    }

    fn render(&mut self) -> Result<()> {
        // Get camera reference first to avoid borrow checker issues
        let camera = self.state.renderer.camera().clone();
        
        self.state.renderer.render(
            &self.window,
            &self.state.world,
            &camera,
            &self.state.game_manager,
            &mut self.state.ui_manager,
        )
    }
}