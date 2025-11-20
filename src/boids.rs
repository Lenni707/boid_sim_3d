use bevy::prelude::*;
use rand::Rng;

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

fn update_boids (
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Boid)>,
) {






    
    for (mut transform, boid) in query.iter_mut() {
        transform.translation += boid.vel * time.delta_secs();
    }
}

// use bevy::prelude::*;
// use rand::Rng;
// use std::collections::HashMap;

// // Constants
// const VISUAL_RANGE: f32 = 75.0;
// const VISUAL_RANGE_SQ: f32 = VISUAL_RANGE * VISUAL_RANGE;
// const COHERENCE: f32 = 0.002 * 0.4;
// const AVOID_FACTOR: f32 = 0.05 * 1.5;
// const AVOID_DISTANCE: f32 = 20.0;
// const AVOID_DISTANCE_SQ: f32 = AVOID_DISTANCE * AVOID_DISTANCE;
// const ALIGNMENT_FACTOR: f32 = 0.05 * 0.5;
// const TURN_FACTOR: f32 = 0.2;
// const BOUNDARY: f32 = 50.0;
// const MAX_SPEED: f32 = 5.0;
// const MIN_SPEED: f32 = 3.0;
// const MAX_TURN: f32 = 3.0;
// const CELL_SIZE: f32 = 75.0;

// pub struct BoidPlugin;

// impl Plugin for BoidPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_resource::<SpatialGrid>()
//             .add_systems(Update, (
//                 spawn_boids_on_input,
//                 handle_keys,
//                 update_spatial_grid,
//                 update_boid_velocities,
//                 apply_velocities,
//                 rotate_boids_to_velocity,
//             ).chain());
//     }
// }

// // Components
// #[derive(Component)]
// pub struct Boid {
//     pub vel: Vec3,
// }

// // Resources
// #[derive(Resource, Default)]
// struct SpatialGrid {
//     cells: HashMap<(i32, i32, i32), Vec<Entity>>,
// }

// impl SpatialGrid {
//     fn get_cell_coords(&self, pos: Vec3) -> (i32, i32, i32) {
//         (
//             (pos.x / CELL_SIZE) as i32,
//             (pos.y / CELL_SIZE) as i32,
//             (pos.z / CELL_SIZE) as i32,
//         )
//     }

//     fn rebuild(&mut self, query: &Query<(Entity, &Transform), With<Boid>>) {
//         self.cells.clear();
//         for (entity, transform) in query.iter() {
//             let cell = self.get_cell_coords(transform.translation);
//             self.cells.entry(cell).or_insert_with(Vec::new).push(entity);
//         }
//     }

//     fn get_neighbors(&self, pos: Vec3) -> Vec<Entity> {
//         let (cx, cy, cz) = self.get_cell_coords(pos);
//         let mut neighbors = Vec::new();

//         for dx in -1..=1 {
//             for dy in -1..=1 {
//                 for dz in -1..=1 {
//                     if let Some(entities) = self.cells.get(&(cx + dx, cy + dy, cz + dz)) {
//                         neighbors.extend(entities);
//                     }
//                 }
//             }
//         }
//         neighbors
//     }
// }

// // Systems
// fn spawn_boids_on_input(
//     mut commands: Commands,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     if keyboard.just_pressed(KeyCode::KeyE) {
//         let cone_mesh = meshes.add(Cone {
//             radius: 0.2,
//             height: 0.8,
//         });

//         let material = materials.add(StandardMaterial {
//             base_color: Color::srgb_u8(137, 207, 240),
//             ..default()
//         });

//         let mut rng = rand::thread_rng();

//         for _ in 0..10 {
//             let pos = Vec3::new(
//                 rng.gen_range(-20.0..20.0),
//                 rng.gen_range(5.0..20.0),
//                 rng.gen_range(-20.0..20.0),
//             );

//             let vel = Vec3::new(
//                 rng.gen_range(-2.0..2.0),
//                 rng.gen_range(-1.0..1.0),
//                 rng.gen_range(-2.0..2.0),
//             );

//             commands.spawn((
//                 Boid { vel },
//                 Mesh3d(cone_mesh.clone()),
//                 MeshMaterial3d(material.clone()),
//                 Transform::from_translation(pos),
//             ));
//         }
//     }
// }

// fn handle_keys(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut commands: Commands,
//     boid_query: Query<Entity, With<Boid>>,
// ) {
//     if keyboard.just_pressed(KeyCode::KeyR) {
//         for entity in boid_query.iter() {
//             commands.entity(entity).despawn();
//         }
//     }
// }

// fn update_spatial_grid(
//     mut grid: ResMut<SpatialGrid>,
//     boid_query: Query<(Entity, &Transform), With<Boid>>,
// ) {
//     grid.rebuild(&boid_query);
// }

// fn update_boid_velocities(
//     mut boid_query: Query<(Entity, &mut Boid, &Transform)>,
//     grid: Res<SpatialGrid>,
//     time: Res<Time>,
// ) {
//     let all_boids: Vec<(Entity, Vec3, Vec3)> = boid_query
//         .iter()
//         .map(|(e, b, t)| (e, t.translation, b.vel))
//         .collect();

//     for (self_entity, mut boid, transform) in boid_query.iter_mut() {
//         let self_pos = transform.translation;
//         let neighbors = grid.get_neighbors(self_pos);

//         let mut align_sum = Vec3::ZERO;
//         let mut cohesion_sum = Vec3::ZERO;
//         let mut separate_vel = Vec3::ZERO;
//         let mut neighbor_count = 0.0;

//         for &neighbor_entity in &neighbors {
//             if self_entity == neighbor_entity {
//                 continue;
//             }

//             if let Some(&(_, other_pos, other_vel)) = all_boids
//                 .iter()
//                 .find(|(e, _, _)| *e == neighbor_entity)
//             {
//                 let diff = self_pos - other_pos;
//                 let dist_sq = diff.length_squared();

//                 if dist_sq < VISUAL_RANGE_SQ {
//                     align_sum += other_vel;
//                     cohesion_sum += other_pos;
//                     neighbor_count += 1.0;
//                 }

//                 if dist_sq < AVOID_DISTANCE_SQ && dist_sq > 0.0 {
//                     let distance = dist_sq.sqrt();
//                     separate_vel += diff / distance;
//                 }
//             }
//         }

//         let mut adjustment = Vec3::ZERO;

//         if neighbor_count > 0.0 {
//             let avg_vel = align_sum / neighbor_count;
//             adjustment += (avg_vel - boid.vel) * ALIGNMENT_FACTOR;

//             let avg_pos = cohesion_sum / neighbor_count;
//             adjustment += (avg_pos - self_pos) * COHERENCE;
//         }

//         adjustment += separate_vel * AVOID_FACTOR;

//         // Boundary avoidance
//         if self_pos.x.abs() > BOUNDARY {
//             adjustment.x += if self_pos.x > 0.0 { -TURN_FACTOR } else { TURN_FACTOR };
//         }
//         if self_pos.y < 2.0 || self_pos.y > 40.0 {
//             adjustment.y += if self_pos.y > 20.0 { -TURN_FACTOR } else { TURN_FACTOR };
//         }
//         if self_pos.z.abs() > BOUNDARY {
//             adjustment.z += if self_pos.z > 0.0 { -TURN_FACTOR } else { TURN_FACTOR };
//         }

//         // Add randomness
//         let mut rng = rand::thread_rng();
//         adjustment += Vec3::new(
//             rng.gen_range(-0.1..0.1),
//             rng.gen_range(-0.1..0.1),
//             rng.gen_range(-0.1..0.1),
//         );

//         let old_vel = boid.vel;
//         boid.vel += adjustment;

//         // Speed limits
//         let speed = boid.vel.length();
//         if speed > MAX_SPEED {
//             boid.vel = boid.vel.normalize() * MAX_SPEED;
//         }
//         if speed < MIN_SPEED && speed > 0.0 {
//             boid.vel = boid.vel.normalize() * MIN_SPEED;
//         }

//         // Max turn rate
//         let vel_change = boid.vel - old_vel;
//         if vel_change.length() > MAX_TURN {
//             boid.vel = old_vel + vel_change.normalize() * MAX_TURN;
//         }
//     }
// }

// fn apply_velocities(
//     mut boid_query: Query<(&Boid, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     for (boid, mut transform) in boid_query.iter_mut() {
//         transform.translation += boid.vel * time.delta_secs();
//     }
// }

// fn rotate_boids_to_velocity(
//     mut boid_query: Query<(&Boid, &mut Transform)>,
// ) {
//     for (boid, mut transform) in boid_query.iter_mut() {
//         if boid.vel.length() > 0.1 {
//             transform.rotation = Quat::from_rotation_arc(Vec3::Z, boid.vel.normalize());
//         }
//     }
// }
