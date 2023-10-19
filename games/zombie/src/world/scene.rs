use bevy::prelude::*;
use crate::utils::consts;

#[derive(Resource)]
struct SetWorldMapTimer(Timer);


pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SetWorldMapTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .add_systems(
                Startup,
                if consts::DEBUG_WORLD {set_debug_world} else {build_world}
            )
            .add_systems(Update, set_world)
        ;
    }
}

fn set_debug_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("levels/debug/debug1.gltf#Scene0"),
        ..default()
    });
}

fn set_world(mut commands: Commands, mut lights: Query<&mut PointLight>, mut timer: ResMut<SetWorldMapTimer>, time: Res<Time>) {

    if timer.0.tick(time.delta()).just_finished() {
        println!("Looking for lights!");
        for mut light in &mut lights {
            println!("{:?}", light);
            light.shadows_enabled = true;
        }
    }

}

fn build_world(mut commands: Commands, asset_server: Res<AssetServer>) {}
