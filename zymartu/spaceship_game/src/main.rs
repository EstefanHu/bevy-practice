use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position {x: 0.0, y 0.0 }, Velocity {x:1.0, y:1.0}));
}
