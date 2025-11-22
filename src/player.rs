use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct FlyCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.002,
            speed: 10.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, grab_cursor))
            .add_systems(Update, (camera_movement, camera_look));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FlyCamera::default(),
        Transform::from_xyz(0.0, 1.5, 5.0),
    ));
}

fn grab_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn camera_look(
    mut motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };
    
    for ev in motion.read() {
        camera.yaw -= ev.delta.x * camera.sensitivity;
        camera.pitch -= ev.delta.y * camera.sensitivity;
        
        // Clamp pitch to prevent camera flipping
        camera.pitch = camera.pitch.clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
        
        // Apply rotation to camera
        transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    }
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
) {
    let Ok((mut transform, camera)) = query.single_mut() else {
        return;
    };
    
    let mut direction = Vec3::ZERO;
    
    // Convert Dir3 to Vec3 by dereferencing with *
    let forward = *transform.forward();
    let right = *transform.right();
    let up = *transform.up();
    let down = *transform.down();
    
    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if keyboard.pressed(KeyCode::Space) {
        direction += up;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        direction += down;
    }
    
    if direction.length() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * camera.speed * time.delta_secs();
    }
}
