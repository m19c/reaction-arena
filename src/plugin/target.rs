use bevy::prelude::*;
use rand::prelude::*;
use std::time::Instant;

use super::state::GameState;

#[derive(Default)]
struct SpawnEvent;
struct HitEvent(Target);

#[derive(Component, Clone)]
struct Target(Instant);

struct Frequency(Timer);

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<SpawnEvent>()
      .add_event::<HitEvent>()
      .insert_resource(Frequency(Timer::from_seconds(2.0, true)))
      .add_system_set(
        SystemSet::on_update(GameState::InGame)
          .with_system(spawn_listener)
          .with_system(spawn_event_emitter)
          .with_system(hit_event_emitter)
          .with_system(hit_listener),
      );
  }
}

fn hit_listener(mut er: EventReader<HitEvent>, mut freq: ResMut<Frequency>) {
  for event in er.iter() {
    println!(
      "hitted after {} seconds",
      event.0 .0.elapsed().as_secs_f32()
    );

    // ensure that the timer is resetted so we've time to replace it.
    freq.0.reset();

    let next_duration = freq.0.duration().as_secs_f32() * 0.99;
    println!("adjusting frequency to {}", next_duration);

    *freq = Frequency(Timer::from_seconds(next_duration, true));
  }
}

fn hit_event_emitter(
  mut cmd: Commands,
  mouse_input: Res<Input<MouseButton>>,
  windows: ResMut<Windows>,
  q_target: Query<(&Target, &Sprite, &Transform, Entity)>,
  q_camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
  mut emitter: EventWriter<HitEvent>,
) {
  if !mouse_input.just_pressed(MouseButton::Left) {
    return;
  }

  let (camera, camera_transform) = q_camera.single();
  let primary_window = windows.get_primary().unwrap();
  if let Some(screen_pos) = primary_window.cursor_position() {
    let window_size = Vec2::new(
      primary_window.width() as f32,
      primary_window.height() as f32,
    );
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    let clicked_pos: Vec2 = world_pos.truncate();

    for (target, sprite, transform, entity) in &mut q_target.iter() {
      let custom_size = sprite.custom_size.unwrap();
      let width = custom_size.x;
      let height = custom_size.y;

      if clicked_pos.x >= transform.translation.x - width / 2.0
        && clicked_pos.x <= transform.translation.x + width / 2.0
        && clicked_pos.y >= transform.translation.y - height / 2.0
        && clicked_pos.y <= transform.translation.y + height / 2.0
      {
        emitter.send(HitEvent(target.clone()));
        cmd.entity(entity).despawn();
      }
    }
  }
}

fn spawn_event_emitter(
  time: Res<Time>,
  mut freq: ResMut<Frequency>,
  mut emitter: EventWriter<SpawnEvent>,
) {
  if freq.0.tick(time.delta()).just_finished() {
    emitter.send_default();
  }
}

fn spawn_listener(
  mut cmd: Commands,
  mut er: EventReader<SpawnEvent>,
  q_target: Query<Entity, With<Target>>,
  windows: ResMut<Windows>,
) {
  for _ in er.iter() {
    // despawn the latest target right before we're creating a new one.
    for item in q_target.iter() {
      cmd.entity(item).despawn();
    }

    let primary_window = windows.get_primary().unwrap();

    // right now we're just spawning sprites with a static size of 50.
    // once we change it, we can replace this variable and use the
    // random generated one.
    let sprite_size: f32 = 50.0;

    let mut rng = thread_rng();

    cmd
      .spawn_bundle(SpriteBundle {
        transform: Transform {
          translation: Vec3::new(
            rng.gen_range(
              -primary_window.width() / 2.0 + sprite_size
                ..primary_window.width() / 2.0 - sprite_size,
            ),
            rng.gen_range(
              -primary_window.height() / 2.0 + sprite_size
                ..primary_window.height() / 2.0 - sprite_size,
            ),
            0.0,
          ),
          ..default()
        },
        sprite: Sprite {
          color: Color::rgb(0.25, 0.25, 0.75),
          custom_size: Some(Vec2::new(sprite_size, sprite_size)),
          ..default()
        },
        ..default()
      })
      .insert(Target(Instant::now()));
  }
}
