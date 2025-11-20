use bevy::prelude::*;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boids);
    }
}

#[derive(Component)]
pub struct Boid {
    pub vel: Vec3
}

fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cone_mesh = meshes.add(Cone {
        radius: 0.2,
        height: 0.8,
    });
    
    commands.spawn((
        Boid {
            vel: Vec3::ZERO
        },
        Mesh3d(cone_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(137, 207, 240),
            ..default()
        })),
        Transform::from_xyz(0.5, 5.0, 0.5),
    ));
}

