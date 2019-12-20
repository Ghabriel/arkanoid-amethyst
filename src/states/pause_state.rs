use amethyst::{
    prelude::*,
    input::InputEvent,
};

#[derive(Default)]
pub struct PauseState;

impl SimpleState for PauseState {
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Input(
                InputEvent::ActionPressed(action)
            ) if action == "pause" => Trans::Pop,
            _ => Trans::None,
        }
    }
}
