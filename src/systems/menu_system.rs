use amethyst::{
    assets::AssetStorage,
    audio::{ output::Output, Source },
    ecs::{Read, ReadExpect, ReaderId, System, SystemData, World, WriteExpect, WriteStorage},
    input::{InputEvent, StringBindings},
    shrev::EventChannel,
    ui::{UiText},
};

use crate::{
    audio::{play_select_option_sound, Sounds},
    states::menu_state::Menu,
};

pub struct MenuSystem {
    reader: ReaderId<InputEvent<StringBindings>>,
}

impl MenuSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System>::SystemData::setup(world);

        MenuSystem {
            reader: world.fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        }
    }
}

impl<'a> System<'a> for MenuSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        WriteExpect<'a, Menu>,
        Read<'a, AssetStorage<Source>>,
        ReadExpect<'a, Sounds>,
        Option<Read<'a, Output>>,
    );

    fn run(&mut self, (
        mut texts,
        event_channel,
        mut menu,
        storage,
        sounds,
        audio_output,
    ): Self::SystemData) {
        for event in event_channel.read(&mut self.reader) {
            if let InputEvent::ActionPressed(action) = event {
                match action.as_str() {
                    "pause" => {
                        play_select_option_sound(&sounds, &storage, &audio_output);
                        menu.selected = true;
                    },
                    "menu_up" | "menu_down" => {
                        play_select_option_sound(&sounds, &storage, &audio_output);

                        let text = texts
                            .get_mut(menu.items[menu.focused_item])
                            .expect("Failed to retrieve old menu item");
                        text.color = [1., 1., 1., 0.01];

                        let count_items = menu.items.len();
                        if action == "menu_up" {
                            menu.focused_item = (count_items + menu.focused_item - 1) % count_items;
                        } else {
                            menu.focused_item = (menu.focused_item + 1) % count_items;
                        }

                        let text = texts
                            .get_mut(menu.items[menu.focused_item])
                            .expect("Failed to retrieve new menu item");
                        text.color = [1., 1., 1., 1.];
                    },
                    _ => {},
                }
            }
        }
    }
}
