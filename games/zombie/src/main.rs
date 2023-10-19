//! Loads and renders a glTF file as a scene.

use std::f32::consts::*;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_spectator::{Spectator, SpectatorPlugin};


fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(Msaa::Off)
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), SpectatorPlugin))
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::ReinhardLuminance,
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // EnvironmentMapLight {
        //     diffuse_map: asset_server.load("environments/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //     specular_map: asset_server.load("environments/pisa_specular_rgb9e5_zstd.ktx2"),
        // },
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
        Spectator
    ))
    // .insert(TemporalAntiAliasBundle::default())
    .insert(ScreenSpaceAmbientOcclusionBundle::default());

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     // This is a relatively small scene, so use tighter shadow
    //     // cascade bounds than the default for better quality.
    //     // We also adjusted the shadow map to be larger since we're
    //     // only using a single cascade.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         num_cascades: 1,
    //         maximum_distance: 1.6,
    //         ..default()
    //     }
    //         .into(),
    //     ..default()
    // });
    commands.spawn(SceneBundle {
        scene: asset_server.load("levels/debug/debug1.gltf#Scene0"),
        ..default()
    });
}
