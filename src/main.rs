mod plugin;

use bevy::prelude::*;
use bevy::window::WindowMode;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

// mabe use mouse plugin to get the actual position: https://docs.rs/bevy_mouse_tracking_plugin/latest/bevy_mouse_tracking_plugin/

fn main() {
  let height = 900.0;

  App::new()
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(WindowDescriptor {
      width: height * RESOLUTION,
      height: height,
      title: "Reaction Arena".to_string(),
      resizable: false,
      mode: WindowMode::Windowed,
      ..default()
    })
    .add_plugin(plugin::debug::DebugPlugin)
    .add_plugin(plugin::state::StatePlugin)
    .add_plugin(plugin::target::TargetPlugin)
    .add_plugins(DefaultPlugins)
    .add_startup_system(camera)
    .add_system(frame_limiter)
    .run();
}

// https://github.com/bevyengine/bevy/issues/1343
fn frame_limiter() {
  use std::{thread, time};
  thread::sleep(time::Duration::from_millis(10));
}

#[derive(Component)]
struct MainCamera;

fn camera(mut cmd: Commands) {
  cmd
    .spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(MainCamera);
}
