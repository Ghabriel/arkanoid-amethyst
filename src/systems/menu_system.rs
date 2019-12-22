use amethyst::{
    ecs::{Read, ReaderId, System, WriteExpect, WriteStorage},
    shrev::EventChannel,
    ui::UiText,
};

use crate::{
    audio::{Sound, SoundKit},
    states::menu_state::{Menu, MenuEvent},
};

#[derive(Default)]
pub struct MenuSystem;

impl<'a> System<'a> for MenuSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        Read<'a, EventChannel<MenuEvent>>,
        WriteExpect<'a, ReaderId<MenuEvent>>,
        WriteExpect<'a, Menu>,
        SoundKit<'a>,
    );

    fn run(&mut self, (
        mut texts,
        event_channel,
        mut event_reader,
        mut menu,
        sound_kit,
    ): Self::SystemData) {
        for event in event_channel.read(&mut event_reader) {
            let action = &event.action_pressed;
            match action.as_str() {
                "pause" => {
                    sound_kit.play_sound(Sound::SelectOption);
                    menu.selected = true;
                },
                "menu_up" | "menu_down" => {
                    sound_kit.play_sound(Sound::SelectOption);

                    texts
                        .get_mut(menu.items[menu.focused_item])
                        .expect("Failed to retrieve old menu item")
                        .color = [1., 1., 1., 0.01];

                    menu.focused_item = {
                        let count_items = menu.items.len();

                        if action == "menu_up" {
                            (count_items + menu.focused_item - 1) % count_items
                        } else {
                            (menu.focused_item + 1) % count_items
                        }
                    };

                    texts
                        .get_mut(menu.items[menu.focused_item])
                        .expect("Failed to retrieve new menu item")
                        .color = [1., 1., 1., 1.];
                },
                _ => {},
            }
        }
    }
}
