use bevy::prelude::*;
use rand::Rng;

const VISUAL_RANGE: f32 = 10.0;
const SEPARATION_DISTANCE: f32 = 2.0;

const SEPARATION_FACTOR: f32 = 0.1;
const ALIGNMENT_FACTOR: f32 = 0.02;
const COHESION_FACTOR: f32 = 0.005;

const BOUNDARY_MIN: Vec3 = Vec3::new(-10.0, 0.0, -10.0);
const BOUNDARY_MAX: Vec3 = Vec3::new(10.0, 20.0, 10.0);

const MIN_SPEED: f32 = 3.0;
const MAX_SPEED: f32 = 10.0;

const GROUND_AVOIDANCE_DISTANCE: f32 = 5.0;
const WALL_AVOIDANCE_DISTANCE: f32 = 5.0;
const GROUND_AVOIDANCE_FACTOR: f32 = 0.05;
const WALL_AVOIDANCE_FACTOR: f32 = 0.05;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_boids, update_boids));
    }
}

#[derive(Component)]
pub struct Boid {
    pub vel: Vec3,
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
            let x = rng.gen_range(BOUNDARY_MIN.x..BOUNDARY_MAX.x);
            let y = rng.gen_range(2.0..BOUNDARY_MAX.y);
            let z = rng.gen_range(BOUNDARY_MIN.z..BOUNDARY_MAX.z);

            let random_dir = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-1.0..1.0),
            )
            .normalize();

            let speed = rng.gen_range(MIN_SPEED..MAX_SPEED);
            let vel = random_dir * speed;

            commands.spawn((
                Boid { vel },
                Mesh3d(cone_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform {
                    translation: Vec3::new(x, y, z),
                    rotation: Quat::from_rotation_arc(Vec3::Z * -1.0, vel.normalize()),
                    ..default()
                },
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
                    separation += (transform.translation - other.position) / distance;
                }

                if distance < VISUAL_RANGE {
                    neighbours += 1;
                    avg_velocity += other.vel;
                    avg_position += other.position;
                }
            }
        }
        let current_vel = boid.vel;
        boid.vel += separation * SEPARATION_FACTOR;

        if neighbours > 0 {
            avg_velocity /= neighbours as f32;
            avg_position /= neighbours as f32;

            boid.vel += (avg_velocity - current_vel) * ALIGNMENT_FACTOR;
            boid.vel += (avg_position - transform.translation) * COHESION_FACTOR;
        }

        let height = transform.translation.y;

        if height < GROUND_AVOIDANCE_DISTANCE {
            boid.vel.y += GROUND_AVOIDANCE_FACTOR * (GROUND_AVOIDANCE_DISTANCE - height) / GROUND_AVOIDANCE_DISTANCE;
        }

        if height > BOUNDARY_MAX.y - GROUND_AVOIDANCE_DISTANCE {
            boid.vel.y -= GROUND_AVOIDANCE_FACTOR * (height - (BOUNDARY_MAX.y - GROUND_AVOIDANCE_DISTANCE)) / GROUND_AVOIDANCE_DISTANCE;
        }

        let pos = transform.translation;

        if pos.x - BOUNDARY_MIN.x < WALL_AVOIDANCE_DISTANCE {
            boid.vel.x += WALL_AVOIDANCE_FACTOR * (WALL_AVOIDANCE_DISTANCE - (pos.x - BOUNDARY_MIN.x)) / WALL_AVOIDANCE_DISTANCE;
        }
        if BOUNDARY_MAX.x - pos.x < WALL_AVOIDANCE_DISTANCE {
            boid.vel.x -= WALL_AVOIDANCE_FACTOR * (WALL_AVOIDANCE_DISTANCE - (BOUNDARY_MAX.x - pos.x)) / WALL_AVOIDANCE_DISTANCE;
        }
        if pos.z - BOUNDARY_MIN.z < WALL_AVOIDANCE_DISTANCE {
            boid.vel.z += WALL_AVOIDANCE_FACTOR * (WALL_AVOIDANCE_DISTANCE - (pos.z - BOUNDARY_MIN.z)) / WALL_AVOIDANCE_DISTANCE;
        }
        if BOUNDARY_MAX.z - pos.z < WALL_AVOIDANCE_DISTANCE {
            boid.vel.z -= WALL_AVOIDANCE_FACTOR * (WALL_AVOIDANCE_DISTANCE - (BOUNDARY_MAX.z - pos.z)) / WALL_AVOIDANCE_DISTANCE;
        }

        let speed = boid.vel.length();

        if speed > 0.0 {
            boid.vel = boid.vel.normalize() * speed.clamp(MIN_SPEED, MAX_SPEED);
        }

        transform.translation += boid.vel * time.delta_secs();

        if boid.vel.length_squared() > 0.0001 {
            let forward = Vec3::Z * -1.0;
            transform.rotation = Quat::from_rotation_arc(forward, boid.vel.normalize());
        }
    }
}
