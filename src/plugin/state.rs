use bevy::prelude::*;

pub struct StatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
  Menu,
  InGame,
}

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state(GameState::Menu)
      .add_system(game_state_machine);
  }
}

fn game_state_machine(kb: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
  if kb.just_pressed(KeyCode::Space) {
    state.set(GameState::InGame).unwrap();
  }

  if kb.just_pressed(KeyCode::Escape) {
    state.set(GameState::Menu).unwrap();
  }
}
