use amethyst::{
    core::Transform,
    ecs::{
        Entity,
        Join,
        Read,
        ReadStorage,
        System,
        world::EntitiesRes,
        Write,
        WriteStorage,
    },
    shrev::EventChannel,
};

use crate::{
    audio::{Sound, SoundKit},
    config::GameConfig,
    entities::{
        ball::Ball,
        brick::{Brick, BrickKind},
        paddle::Paddle,
    },
    states::game_state::GameEvent,
    systems::powerups::piercing_ball_system::PiercingBallTimer,
};

enum CollisionAxis {
    Horizontal,
    Vertical,
}

fn process_collision(ball: &mut Ball, collision_axis: CollisionAxis) {
    match collision_axis {
        CollisionAxis::Horizontal => ball.velocity[0] = -ball.velocity[0],
        CollisionAxis::Vertical => ball.velocity[1] = -ball.velocity[1],
    }
}

pub struct BallBounceSystem;

impl<'a> System<'a> for BallBounceSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, PiercingBallTimer>,
        ReadStorage<'a, Brick>,
        ReadStorage<'a, Paddle>,
        Read<'a, EntitiesRes>,
        Read<'a, GameConfig>,
        Write<'a, EventChannel<GameEvent>>,
        SoundKit<'a>,
    );

    fn run(&mut self, (
        mut balls,
        transforms,
        piercing_ball_timers,
        bricks,
        paddles,
        entities,
        config,
        mut game_event_channel,
        sound_kit,
    ): Self::SystemData) {
        for (entity, mut ball, transform) in (&entities, &mut balls, &transforms).join() {
            let piercing_ball_timer = piercing_ball_timers.get(entity);

            Self::handle_brick_collisions(
                &entity,
                &mut ball,
                &transform,
                &piercing_ball_timer,
                &transforms,
                &bricks,
                &entities,
                &mut game_event_channel,
                &sound_kit,
            );

            Self::handle_paddle_collisions(
                &mut ball,
                &transform,
                &transforms,
                &paddles,
                &sound_kit,
            );

            Self::handle_wall_collisions(
                &entity,
                &mut ball,
                &transform,
                &entities,
                &config,
                &sound_kit,
            );
        }
    }
}

impl BallBounceSystem {
    fn handle_wall_collisions(
        ball_entity: &Entity,
        ball: &mut Ball,
        transform: &Transform,
        entities: &Read<EntitiesRes>,
        config: &GameConfig,
        sound_kit: &SoundKit,
    ) {
        let ball_x = transform.translation().x;
        let ball_y = transform.translation().y;

        if (ball_x <= ball.radius && ball.velocity[0] < 0.0)
            || (ball_x >= config.arena.width - ball.radius && ball.velocity[0] > 0.0)
        {
            sound_kit.play_sound(Sound::Bounce);
            ball.velocity[0] = -ball.velocity[0];
        }

        if ball_y >= config.arena.height - ball.radius && ball.velocity[1] > 0.0 {
            sound_kit.play_sound(Sound::Bounce);
            ball.velocity[1] = -ball.velocity[1];
        }

        if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
            sound_kit.play_sound(Sound::GameOver);
            entities.delete(*ball_entity).expect("Failed to delete ball");
        }
    }

    fn handle_brick_collisions(
        ball_entity: &Entity,
        ball: &mut Ball,
        transform: &Transform,
        piercing_ball_timer: &Option<&PiercingBallTimer>,
        transforms: &ReadStorage<Transform>,
        bricks: &ReadStorage<Brick>,
        entities: &Read<EntitiesRes>,
        game_event_channel: &mut EventChannel<GameEvent>,
        sound_kit: &SoundKit,
    ) {
        let ball_x = transform.translation().x;
        let ball_y = transform.translation().y;

        for (entity, brick, transform) in (entities, bricks, transforms).join() {
            let brick_x = transform.translation().x;
            let brick_y = transform.translation().y;
            let x_left = brick_x - 0.5 * brick.width;
            let x_right = brick_x + 0.5 * brick.width;
            let y_bottom = brick_y - 0.5 * brick.height;
            let y_top = brick_y + 0.5 * brick.height;
            let mut collision_axis = None;

            if circle_line_segment_collision(
                (ball_x, ball_y),
                ball.radius,
                (x_left, y_top),
                (x_left, y_bottom),
            ) && ball.velocity[0] > 0.0 {
                collision_axis = Some(CollisionAxis::Horizontal);
            }

            if circle_line_segment_collision(
                (ball_x, ball_y),
                ball.radius,
                (x_left, y_top),
                (x_right, y_top),
            ) && ball.velocity[1] < 0.0 {
                collision_axis = Some(CollisionAxis::Vertical);
            }

            if circle_line_segment_collision(
                (ball_x, ball_y),
                ball.radius,
                (x_left, y_bottom),
                (x_right, y_bottom),
            ) && ball.velocity[1] > 0.0 {
                collision_axis = Some(CollisionAxis::Vertical);
            }

            if circle_line_segment_collision(
                (ball_x, ball_y),
                ball.radius,
                (x_right, y_top),
                (x_right, y_bottom),
            ) && ball.velocity[0] < 0.0 {
                collision_axis = Some(CollisionAxis::Horizontal);
            }

            if let Some(axis) = collision_axis {
                let mut is_piercing_ball = piercing_ball_timer.is_some();

                match brick.kind {
                    BrickKind::Standard => {},
                    BrickKind::FastForward => {
                        game_event_channel.single_write(GameEvent::FastForward);
                    },
                    BrickKind::BallSplit => {
                        game_event_channel.single_write(GameEvent::BallSplit(*ball_entity));
                    },
                    BrickKind::PiercingBall => {
                        game_event_channel.single_write(GameEvent::PiercingBall(*ball_entity));
                        is_piercing_ball = true;
                    },
                    BrickKind::SlowMode => {
                        game_event_channel.single_write(GameEvent::SlowMode);
                    },
                }

                if !is_piercing_ball {
                    process_collision(ball, axis);
                }

                sound_kit.play_sound(Sound::Bounce);
                entities.delete(entity).expect("Failed to delete brick");
            }
        }
    }

    fn handle_paddle_collisions(
        ball: &mut Ball,
        transform: &Transform,
        transforms: &ReadStorage<Transform>,
        paddles: &ReadStorage<Paddle>,
        sound_kit: &SoundKit,
    ) {
        let ball_x = transform.translation().x;
        let ball_y = transform.translation().y;

        for (paddle, transform) in (paddles, transforms).join() {
            let paddle_x = transform.translation().x;
            let paddle_y = transform.translation().y;

            if point_in_rect(
                ball_x,
                ball_y,
                paddle_x - 0.5 * paddle.width - ball.radius,
                paddle_y - 0.5 * paddle.height - ball.radius,
                paddle_x + 0.5 * paddle.width + ball.radius,
                paddle_y + 0.5 * paddle.height + ball.radius,
            ) {
                if ball.velocity[1] < 0.0 {
                    sound_kit.play_sound(Sound::Bounce);

                    let ball_velocity = ball.velocity[0].hypot(ball.velocity[1]);
                    let dx = ball_x - paddle_x;
                    let dy = ball_y - (paddle_y - 5.0);
                    let angle = dy.atan2(dx);
                    let (sin, cos) = angle.sin_cos();
                    ball.velocity[0] = ball_velocity * cos;
                    ball.velocity[1] = ball_velocity * sin;
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn circle_line_segment_collision(
    (xc, yc): (f32, f32),
    radius: f32,
    (x1, y1): (f32, f32),
    (x2, y2): (f32, f32),
) -> bool {
    let a = (xc - x1, yc - y1);
    let line_direction = (x2 - x1, y2 - y1);
    /*
     * The projection length (a1) is given by:
     *     a1 = ||a||.cos(theta) = a.line_direction^,
     * where:
     *     theta is the angle between "a" and the line segment;
     *     line_direction^ is the normalized version of line_direction.
     * Or, equivalently:
     *     a1 = a.line_direction / ||line_direction||
     * where "." here denotes the dot product. Calculating the magnitude requires
     * a square root calculation, which is always undesirable from a performance
     * standpoint. We can optimize it away by using:
     *     pseudo_projection_length = a1 * ||line_direction||
     *                              = a.line_direction
     * instead of "a1" in our calculations, making some adjustments to the
     * formulas along the way. Always keep in mind that:
     *     real projection length (a1) = pseudo_projection_length / ||line_direction||
     */
    let pseudo_projection_length = dot_product(a, line_direction);

    /*
     * The projection is outside the line segment on the negative direction if:
     *     a1 < 0
     *     pseudo_projection_length / ||line_direction|| < 0
     *     pseudo_projection_length < 0
     *
     * And it's outside the line segment on the positive direction if:
     *     a1 > ||line_direction||
     *     pseudo_projection_length / ||line_direction|| > ||line_direction||
     *     pseudo_projection_length > ||line_direction||²
     * Note that squaring a magnitude eliminates the need for calculating a
     * square root.
     */
    let line_direction_squared_length = dot_product(line_direction, line_direction);

    let radius_squared = radius * radius;

    if pseudo_projection_length < 0.0 {
        let distance_circle_p1_squared = dot_product((xc, yc), (x1, y1));
        distance_circle_p1_squared <= radius_squared
    } else if pseudo_projection_length > line_direction_squared_length {
        let distance_circle_p2_squared = dot_product((xc, yc), (x2, y2));
        distance_circle_p2_squared <= radius_squared
    } else {
        /*
         * ||a||² = a1² + d², by the Pythagorean Theorem,
         *                    where d is the distance from the circle to the line.
         * d² = ||a||² - a1²
         * d² = ||a||² - pseudo_projection_length² / ||line_direction||²
         */
        let ppl2 = pseudo_projection_length * pseudo_projection_length;
        let d2 = dot_product(a, a) - ppl2 / line_direction_squared_length;
        d2 <= radius_squared
    }
}

fn dot_product(
    (x1, y1): (f32, f32),
    (x2, y2): (f32, f32),
) -> f32 {
    x1 * x2 + y1 * y2
}
