use amethyst::{
    prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::{
    action_trigger_limiter::ActionTriggerLimiter,
};

use std::ops::Deref;

#[derive(Default)]
pub struct PauseState {
    pub unpause_action: ActionTriggerLimiter,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.unpause_action.last_action_state = true;
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let unpause = self.unpause_action.action_is_down(
            data.world.read_resource::<InputHandler<StringBindings>>().deref(),
            "pause",
        );

        if unpause {
            Trans::Pop
        } else {
            Trans::None
        }
    }
}
