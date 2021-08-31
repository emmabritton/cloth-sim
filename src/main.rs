mod controller;
mod renderer;
mod models;

use ggez::{ContextBuilder, event};
use ggez::conf::{WindowMode, WindowSetup};
use std::env;
use std::path::PathBuf;
use crate::controller::SimController;

pub const SCREEN_WIDTH: f32 = 1920.;
pub const SCREEN_HEIGHT: f32 = 1440.;

fn main() {
    let (ctx, event_loop) = setup_ggez()
        .build()
        .expect("Could not create ggez context!");

    let mut system = SimController::new();
    system.setup();

    event::run(ctx, event_loop, system)
}

fn setup_ggez() -> ContextBuilder {
    let mut cb = ContextBuilder::new("Cloth Sim", "Ray Britton")
        .window_mode(WindowMode {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..WindowMode::default()
        })
        .window_setup(WindowSetup {
            title: String::from("Cloth Sim"),
            ..WindowSetup::default()
        });

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    } else {
        PathBuf::from("./resources");
    }

    cb
}
