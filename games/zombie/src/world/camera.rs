use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy_spectator::Spectator;
use crate::utils::consts;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

fn add_camera(mut commands: Commands) {
    let mut camera_settings = commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::ReinhardLuminance,
            transform: Transform {
                translation: Vec3::new(8.2, 4.0, 11.5),
                rotation: Quat::from_xyzw(0.0, 5.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        BloomSettings {
            intensity: 0.25,
            ..default()
        },
        FogSettings {
            falloff: FogFalloff::Exponential {
                density: 0.07
            },
            color: Color::rgba(0.05, 0.05, 0.05, 1.0),
            ..default()
        },
        // EnvironmentMapLight {
        //     diffuse_map: asset_server.load("environments/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //     specular_map: asset_server.load("environments/pisa_specular_rgb9e5_zstd.ktx2"),
        // },
    ));
    camera_settings
        .insert(TemporalAntiAliasBundle::default())
        .insert(ScreenSpaceAmbientOcclusionBundle::default());

    if (consts::SPECTATOR_MODE) {
        camera_settings.insert(Spectator);
    }
}