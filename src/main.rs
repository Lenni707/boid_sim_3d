use bevy::prelude::*;

mod player;
mod world;
mod boids;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            world::WorldPlugin,
            player::PlayerPlugin,
            boids::BoidPlugin
        ))
        .run();
}
