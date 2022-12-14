use bevy::{prelude::*, window::WindowResizeConstraints};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::*;
use odonata::{
    consts::APP_NAME,
    engine::{ConfigState, EnginePlugin},
    scenes::{GameState, ScenePlugin},
};

fn main() {
    let config = ConfigState::default();
    let (width, height) = config.screen_resolution();

    let window_width = width as f32 * 2.0;
    let window_height = height as f32 * 2.0;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(config.log_plugin())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: APP_NAME.to_string(),
                        width: window_width,
                        height: window_height,
                        resize_constraints: WindowResizeConstraints {
                            min_width: window_width,
                            min_height: window_height,
                            ..default()
                        },
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                })
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(EnginePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ScenePlugin)
        .add_state(GameState::Intro)
        .add_system(bevy::window::close_on_esc)
        .run();
}
