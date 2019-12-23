use amethyst::{
    ecs::{Join, Read, ReaderId, System, SystemData, WriteStorage, World, WorldExt},
    shrev::EventChannel,
};

use crate::{
    entities::ball::{Ball, change_speed},
    states::game_state::GameEvent,
};

pub struct FastForwardSystem {
    reader: ReaderId<GameEvent>,
}

impl FastForwardSystem {
    pub fn new(world: &mut World) -> FastForwardSystem {
        <Self as System<'_>>::SystemData::setup(world);

        FastForwardSystem {
            reader: world.write_resource::<EventChannel<GameEvent>>().register_reader(),
        }
    }
}

impl<'a> System<'a> for FastForwardSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        Read<'a, EventChannel<GameEvent>>,
    );

    fn run(&mut self, (mut balls, event_channel): Self::SystemData) {
        for event in event_channel.read(&mut self.reader) {
            match event {
                GameEvent::FastForward => {
                    for ball in (&mut balls).join() {
                        change_speed(ball, |v| v * 1.1);
                    }
                },
                _ => {},
            }
        }
    }
}
