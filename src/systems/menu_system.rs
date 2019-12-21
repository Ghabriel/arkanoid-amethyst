use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    ecs::{Read, ReadExpect, ReaderId, System, WriteExpect, WriteStorage},
    shrev::EventChannel,
    ui::UiText,
};

use crate::{
    audio::{play_sound, Sounds},
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
        Read<'a, AssetStorage<Source>>,
        ReadExpect<'a, Sounds>,
        Option<Read<'a, Output>>,
    );

    fn run(&mut self, (
        mut texts,
        event_channel,
        mut event_reader,
        mut menu,
        storage,
        sounds,
        audio_output,
    ): Self::SystemData) {
        for event in event_channel.read(&mut event_reader) {
            let action = &event.action_pressed;
            match action.as_str() {
                "pause" => {
                    play_sound(&sounds.select_option_sfx, &storage, &audio_output);
                    menu.selected = true;
                },
                "menu_up" | "menu_down" => {
                    play_sound(&sounds.select_option_sfx, &storage, &audio_output);

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
