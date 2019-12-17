use amethyst::{
    input::{BindingTypes, InputHandler},
};

use std::{
    borrow::Borrow,
    hash::Hash,
};

/**
 * Limits an action so that it only happens once per keypress.
 */
#[derive(Default)]
pub struct ActionTriggerLimiter {
    pub last_action_state: bool,
}

impl ActionTriggerLimiter {
    pub fn action_is_down<T, A>(
        &mut self,
        input_handler: &InputHandler<T>,
        action: &A,
    ) -> bool
        where T: BindingTypes,
              T::Action: Borrow<A>,
              A: Hash + Eq + ?Sized,
    {
        let was_pressed_previously = self.last_action_state;

        self.last_action_state = input_handler
            .action_is_down(action)
            .unwrap_or(false);

        if was_pressed_previously {
            false
        } else {
            self.last_action_state
        }
    }
}
