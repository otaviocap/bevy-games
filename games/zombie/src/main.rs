mod engine;
mod player;
mod world;
mod utils;

use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::*,
};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_editor_pls::prelude::*;
use crate::utils::consts;

fn main() {
    let mut app = App::new();

    app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins);

    if (consts::DEBUG) {
        app.add_plugins((
            WorldInspectorPlugin::new(),
            EditorPlugin::default(),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default()
        ));
    }

    app.add_plugins((
        world::camera::CameraPlugin,
        world::scene::ScenePlugin,
    ));

    app.run()
}