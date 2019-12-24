use amethyst::{
    ecs::{Join, Read, ReaderId, System, SystemData, WriteStorage, World, WorldExt},
    shrev::EventChannel,
};

use crate::{
    entities::ball::{Ball, change_speed},
    states::game_state::GameEvent,
};

pub struct SlowModeSystem {
    reader: ReaderId<GameEvent>,
}

impl SlowModeSystem {
    pub fn new(world: &mut World) -> SlowModeSystem {
        <Self as System<'_>>::SystemData::setup(world);

        SlowModeSystem {
            reader: world.write_resource::<EventChannel<GameEvent>>().register_reader(),
        }
    }
}

impl<'a> System<'a> for SlowModeSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        Read<'a, EventChannel<GameEvent>>,
    );

    fn run(&mut self, (mut balls, event_channel): Self::SystemData) {
        for event in event_channel.read(&mut self.reader) {
            match event {
                GameEvent::SlowMode => {
                    for ball in (&mut balls).join() {
                        change_speed(ball, |v| v * 0.9);
                    }
                },
                _ => {},
            }
        }
    }
}
