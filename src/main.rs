use std::time::Duration;
use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod components;
mod grid;
mod snake;
mod food;

use snake::{snake_movement, spawn_snake};

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_plugins(DefaultPlugins)
        .add_systems(Update, snake_movement)
        .add_systems(Update, food::spawn_system.run_if(on_timer(Duration::from_secs_f32(1.0))))
        .add_systems(PostUpdate, (grid::position_translation, grid::size_scaling))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
