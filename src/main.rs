use anyhow::Result;
use env_logger;
use log::info;

mod engine;
mod game;
mod world;
mod rendering;
mod input;
mod audio;
mod ui;
mod networking;
mod utils;

use engine::Engine;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Minecraft Clone");

    // Create and run the game engine
    let mut engine = Engine::new()?;
    engine.run()?;

    Ok(())
}
