use amethyst::{
    ecs::{Read, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::{UiText},
};

use crate::{
    action_trigger_limiter::ActionTriggerLimiter,
    states::menu_state::Menu,
};

use std::ops::Deref;

#[derive(Default)]
pub struct MenuSystem {
    pub menu_up_action: ActionTriggerLimiter,
    pub menu_down_action: ActionTriggerLimiter,
    pub menu_select_action: ActionTriggerLimiter,
}

impl<'a> System<'a> for MenuSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        Read<'a, InputHandler<StringBindings>>,
        WriteExpect<'a, Menu>,
    );

    fn run(&mut self, (mut texts, input, mut menu): Self::SystemData) {
        let menu_up = self.menu_up_action.action_is_down(
            input.deref(),
            "menu_up",
        );

        let menu_down = self.menu_down_action.action_is_down(
            input.deref(),
            "menu_down",
        );

        if menu_up || menu_down {
            let text = texts.get_mut(
                if menu.focused_item == 0 {
                    menu.new_game
                } else {
                    menu.about
                }
            ).expect("Failed to retrieve old menu item");
            text.color = [1., 1., 1., 0.01];

            if menu.focused_item == 0 {
                menu.focused_item = 1;
            } else {
                menu.focused_item = 0;
            }

            let text = texts.get_mut(
                if menu.focused_item == 0 {
                    menu.new_game
                } else {
                    menu.about
                }
            ).expect("Failed to retrieve old menu item");
            text.color = [1., 1., 1., 1.];
        }

        let menu_select = self.menu_select_action.action_is_down(
            input.deref(),
            "pause",
        );

        if menu_select {
            menu.selected = true;
        }
    }
}
