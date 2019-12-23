use amethyst::{
    ecs::{
        Component,
        DenseVecStorage,
        Join,
        Read,
        ReaderId,
        System,
        SystemData,
        WriteStorage,
        world::EntitiesRes,
        World,
        WorldExt,
    },
    renderer::SpriteRender,
    shrev::EventChannel,
};

use crate::states::game_state::GameEvent;

use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct PiercingBallTimer {
    end_time: Instant,
}

impl Component for PiercingBallTimer {
    type Storage = DenseVecStorage<Self>;
}

pub struct PiercingBallSystem {
    reader: ReaderId<GameEvent>,
}

impl PiercingBallSystem {
    pub fn new(world: &mut World) -> PiercingBallSystem {
        <Self as System<'_>>::SystemData::setup(world);

        PiercingBallSystem {
            reader: world.write_resource::<EventChannel<GameEvent>>().register_reader(),
        }
    }
}

impl<'a> System<'a> for PiercingBallSystem {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, PiercingBallTimer>,
        Read<'a, EntitiesRes>,
        Read<'a, EventChannel<GameEvent>>,
    );

    fn run(&mut self, (
        mut sprite_renders,
        mut timers,
        entities,
        event_channel,
    ): Self::SystemData) {
        let now = Instant::now();
        let expired_balls = (&entities, &mut timers).join()
            .filter(|(_, timer)| now >= timer.end_time)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>()
            .clone();

        for ball_entity in expired_balls {
            timers
                .remove(ball_entity)
                .expect("Failed to remove PiercingBallTimer");

            sprite_renders
                .get_mut(ball_entity)
                .expect("Failed to retrieve SpriteRender of ball")
                .sprite_number = 1;
        }

        for event in event_channel.read(&mut self.reader) {
            match event {
                GameEvent::PiercingBall(ball_entity) => {
                    let end_time = now + Duration::from_secs(5);

                    timers
                        .insert(*ball_entity, PiercingBallTimer { end_time })
                        .expect("Failed to add PiercingBallTimer");

                    sprite_renders
                        .get_mut(*ball_entity)
                        .expect("Failed to retrieve SpriteRender of ball")
                        .sprite_number = 6;
                },
                _ => {},
            }
        }
    }
}
