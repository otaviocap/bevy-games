use bevy::prelude::*;
use crate::utils::consts;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                if consts::DEBUG_WORLD {set_debug_world} else {build_world},
                set_world
            )
        );
    }
}

fn set_debug_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("levels/debug/debug1.gltf#Scene0"),
        ..default()
    });
}

fn set_world(mut commands: Commands, mut lights: Query<&mut PointLight>) {
    for mut light in &mut lights {
        light.shadows_enabled = true;
    }

}

fn build_world(mut commands: Commands, asset_server: Res<AssetServer>) {}
