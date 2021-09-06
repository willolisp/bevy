use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::Quat,
    prelude::*,
    render::camera::Camera,
    sprite::SpriteSettings,
};

use rand::Rng;

const CAMERA_SPEED: f32 = 1000.0;

/// This example is for performance testing purposes.
/// See https://github.com/bevyengine/bevy/pull/1492
fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(SpriteSettings {
            // NOTE: this is an experimental feature that doesn't work in all cases
            frustum_culling_enabled: true,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tick_system.label("Tick"))
        .add_system(move_camera_system.after("Tick"))
        .run()
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(64.0);
    let map_size = Vec2::splat(320.0);

    let half_x = (map_size.x / 2.0) as i32;
    let half_y = (map_size.y / 2.0) as i32;

    let sprite_handle = materials.add(assets.load("branding/icon.png").into());

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Timer::from_seconds(1.0, true))
        .insert(Transform::from_xyz(0.0, 0.0, 1000.0));

    for y in -half_y..half_y {
        for x in -half_x..half_x {
            let position = Vec2::new(x as f32, y as f32);
            let translation = (position * tile_size).extend(rng.gen::<f32>());
            let rotation = Quat::from_rotation_z(rng.gen::<f32>());
            let scale = Vec3::splat(rng.gen::<f32>() * 2.0);

            commands.spawn().insert_bundle(SpriteBundle {
                material: sprite_handle.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                sprite: Sprite::new(tile_size),
                ..Default::default()
            });
        }
    }
}

// System for rotating and translating the camera
fn move_camera_system(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut().unwrap();
    camera_transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 0.5));
    *camera_transform =
        *camera_transform * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_seconds());
}

// System for printing the number of sprites on every tick of the timer
fn tick_system(time: Res<Time>, sprites_query: Query<&Sprite>, mut timer_query: Query<&mut Timer>) {
    let mut timer = timer_query.single_mut().unwrap();
    timer.tick(time.delta());

    if timer.just_finished() {
        info!("Sprites: {}", sprites_query.iter().count(),);
    }
}
