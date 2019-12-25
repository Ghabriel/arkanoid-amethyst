use amethyst::{
    core::Transform,
    ecs::{
        Entities,
        Join,
        ReadStorage,
        System,
        Write,
        WriteStorage,
    },
    shrev::EventChannel,
};

use crate::{
    audio::{Sound, SoundKit},
    entities::{
        ball::Ball,
        brick::{Brick, BrickKind},
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

pub struct BallBrickCollisionSystem;

impl<'a> System<'a> for BallBrickCollisionSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, PiercingBallTimer>,
        ReadStorage<'a, Brick>,
        Entities<'a>,
        Write<'a, EventChannel<GameEvent>>,
        SoundKit<'a>,
    );

    fn run(&mut self, (
        mut balls,
        transforms,
        piercing_ball_timers,
        bricks,
        entities,
        mut game_event_channel,
        sound_kit,
    ): Self::SystemData) {
        for (ball_entity, mut ball, transform) in (&entities, &mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            for (brick_entity, brick, transform) in (&entities, &bricks, &transforms).join() {
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
                    let mut is_piercing_ball = piercing_ball_timers.get(ball_entity).is_some();

                    match brick.kind {
                        BrickKind::Standard => {},
                        BrickKind::FastForward => {
                            game_event_channel.single_write(GameEvent::FastForward);
                        },
                        BrickKind::BallSplit => {
                            game_event_channel.single_write(GameEvent::BallSplit(ball_entity));
                        },
                        BrickKind::PiercingBall => {
                            game_event_channel.single_write(GameEvent::PiercingBall(ball_entity));
                            is_piercing_ball = true;
                        },
                        BrickKind::SlowMode => {
                            game_event_channel.single_write(GameEvent::SlowMode);
                        },
                    }

                    if !is_piercing_ball {
                        process_collision(&mut ball, axis);
                    }

                    sound_kit.play_sound(Sound::Bounce);
                    entities.delete(brick_entity).expect("Failed to delete brick");
                }
            }
        }
    }
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
