use amethyst::{
    ecs::{
        Read,
        ReaderId,
        System,
        SystemData,
        WriteStorage,
        world::EntitiesRes,
        World,
        WorldExt,
    },
    core::Transform,
    shrev::EventChannel,
    renderer::SpriteRender,
};

use crate::{
    entities::ball::{Ball, change_direction},
    states::game_state::GameEvent,
    systems::powerups::piercing_ball_system::PiercingBallTimer,
};

pub struct BallSplitSystem {
    reader: ReaderId<GameEvent>,
}

impl BallSplitSystem {
    pub fn new(world: &mut World) -> BallSplitSystem {
        <Self as System<'_>>::SystemData::setup(world);

        BallSplitSystem {
            reader: world.write_resource::<EventChannel<GameEvent>>().register_reader(),
        }
    }
}

impl<'a> System<'a> for BallSplitSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, PiercingBallTimer>,
        Read<'a, EntitiesRes>,
        Read<'a, EventChannel<GameEvent>>,
    );

    fn run(&mut self, (
        mut balls,
        mut transforms,
        mut sprite_renders,
        mut piercing_ball_timers,
        entities,
        event_channel,
    ): Self::SystemData) {
        for event in event_channel.read(&mut self.reader) {
            match event {
                GameEvent::BallSplit(old_entity) => {
                    let mut old_ball = balls
                        .get_mut(*old_entity)
                        .expect("Failed to retrieve old ball");
                    change_direction(&mut old_ball, |angle| angle + 15f32.to_radians());

                    let mut ball = Ball {
                        radius: old_ball.radius,
                        velocity: old_ball.velocity,
                    };
                    change_direction(&mut ball, |angle| angle - 15f32.to_radians());

                    let old_translation = transforms
                        .get(*old_entity)
                        .expect("Failed to retrieve Transform of old ball")
                        .translation()
                        .clone();
                    let transform = Transform::from(old_translation);

                    let sprite_render = sprite_renders
                        .get(*old_entity)
                        .expect("Failed to retrieve SpriteRender of old ball")
                        .clone();

                    let new_entity = entities.create();
                    balls.insert(new_entity, ball).unwrap();
                    transforms.insert(new_entity, transform).unwrap();
                    sprite_renders.insert(new_entity, sprite_render).unwrap();

                    let piercing_ball_timer = piercing_ball_timers.get(*old_entity).cloned();
                    if let Some(timer) = piercing_ball_timer {
                        piercing_ball_timers.insert(new_entity, timer).unwrap();
                    }
                },
                _ => {},
            }
        }
    }
}
