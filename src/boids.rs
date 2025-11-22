use bevy::prelude::*;
use rand::Rng;

const VISUAL_RANGE: f32 = 10.0;
const SEPARATION_DISTANCE: f32 = 2.0;

const SEPARATION_FACTOR: f32 = 0.01;
const ALIGNMENT_FACTOR: f32 = 0.001;
const COHESION_FACTOR: f32 = 0.0005;


pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_boids, update_boids));
    }
}

#[derive(Component)]
pub struct Boid {
    pub vel: Vec3
}

fn spawn_boids(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        let cone_mesh = meshes.add(Cone {
            radius: 0.2,
            height: 0.8,
        });
        
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb_u8(137, 207, 240),
            ..default()
        });
        
        let mut rng = rand::thread_rng();
        
        for _ in 0..10 {
            let x = rng.gen_range(-10.0..10.0);
            let y = rng.gen_range(2.0..20.0);
            let z = rng.gen_range(-10.0..10.0);
            
            commands.spawn((
                Boid {
                    vel: Vec3::ZERO
                },
                Mesh3d(cone_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x, y, z),
            ));
        }
    }
}

#[derive(Clone, Copy)]
struct BoidData {
    pub position: Vec3,
    pub vel: Vec3,
}

fn update_boids(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Boid)>,
) {
    let boid_data: Vec<BoidData> = query
    .iter()
    .map(|(transform, boid)| BoidData {
        position: transform.translation,
        vel: boid.vel,
    })
    .collect();

    for (i, (mut transform, mut boid)) in query.iter_mut().enumerate() {
        let mut avg_velocity = Vec3::ZERO;
        let mut avg_position = Vec3::ZERO;
        let mut separation = Vec3::ZERO;
        let mut neighbours: u32 = 0;
        
        for (j, &other) in boid_data.iter().enumerate() {
            if i != j {
                let distance = transform.translation.distance(other.position);

                if distance < SEPARATION_DISTANCE && distance > 0.0 {
                    let diff = transform.translation - other.position;
                    separation += diff / distance;  // näher -> stärker wegpushen
                }

                if distance < VISUAL_RANGE {
                    neighbours += 1;
                    avg_velocity += other.vel;      
                    avg_position += other.position; 
                }
            } 
        }

        boid.vel += separation * SEPARATION_FACTOR;

        if neighbours > 0 {
            avg_velocity /= neighbours as f32;
            avg_position /= neighbours as f32;
            
            let alignment = (avg_velocity - boid.vel) * ALIGNMENT_FACTOR;
            
            let cohesion = (avg_position - transform.translation) * COHESION_FACTOR;
    
            boid.vel += alignment + cohesion;
        }
        
        // Update position
        transform.translation += boid.vel * time.delta_secs();
    }

}
