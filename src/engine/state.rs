use anyhow::Result;
use winit::window::Window;

use crate::rendering::Renderer;
use crate::input::InputManager;
use crate::world::World;
use crate::game::GameManager;
use crate::audio::AudioManager;
use crate::ui::UIManager;

/// Central state container for all engine subsystems
pub struct EngineState {
    pub renderer: Renderer,
    pub input_manager: InputManager,
    pub world: World,
    pub game_manager: GameManager,
    pub audio_manager: AudioManager,
    pub ui_manager: UIManager,
}

impl EngineState {
    pub async fn new_async(window: &Window) -> Result<Self> {
        // Initialize renderer first as other systems may depend on it
        let renderer = Renderer::new(window).await?;
        
        // Initialize other systems
        let input_manager = InputManager::new();
        let world = World::new();
        let game_manager = GameManager::new();
        let audio_manager = AudioManager::new()?;
        let ui_manager = UIManager::new(&renderer);

        Ok(Self {
            renderer,
            input_manager,
            world,
            game_manager,
            audio_manager,
            ui_manager,
        })
    }
    
    pub fn new(window: &Window) -> Result<Self> {
        pollster::block_on(Self::new_async(window))
    }
}