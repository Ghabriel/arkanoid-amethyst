use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::{
    config::GameConfig,
    entities::{
        ball::Ball,
        brick::Brick,
        paddle::Paddle,
    },
};

pub struct BallBounceSystem;

impl<'a> System<'a> for BallBounceSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Brick>,
        ReadStorage<'a, Paddle>,
        Read<'a, GameConfig>,
    );

    fn run(&mut self, (mut balls, transforms, bricks, paddles, config): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if (ball_x <= ball.radius && ball.velocity[0] < 0.0)
                || (ball_x >= config.arena.width - ball.radius && ball.velocity[0] > 0.0)
            {
                ball.velocity[0] = -ball.velocity[0];
            }

            if ball_y >= config.arena.height - ball.radius && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            }

            if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
                println!("SCORE");
            }

            for (brick, transform) in (&bricks, &transforms).join() {
                let brick_x = transform.translation().x;
                let brick_y = transform.translation().y;
                let x_left = brick_x - 0.5 * brick.width;
                let x_right = brick_x + 0.5 * brick.width;
                let y_bottom = brick_y - 0.5 * brick.height;
                let y_top = brick_y + 0.5 * brick.height;

                if circle_line_segment_collision(
                    (ball_x, ball_y),
                    ball.radius,
                    (x_left, y_top),
                    (x_left, y_bottom),
                ) && ball.velocity[0] > 0.0 {
                    ball.velocity[0] = -ball.velocity[0];
                }

                if circle_line_segment_collision(
                    (ball_x, ball_y),
                    ball.radius,
                    (x_left, y_top),
                    (x_right, y_top),
                ) && ball.velocity[1] < 0.0 {
                    ball.velocity[1] = -ball.velocity[1];
                }

                if circle_line_segment_collision(
                    (ball_x, ball_y),
                    ball.radius,
                    (x_left, y_bottom),
                    (x_right, y_bottom),
                ) && ball.velocity[1] > 0.0 {
                    ball.velocity[1] = -ball.velocity[1];
                }

                if circle_line_segment_collision(
                    (ball_x, ball_y),
                    ball.radius,
                    (x_right, y_top),
                    (x_right, y_bottom),
                ) && ball.velocity[0] < 0.0 {
                    ball.velocity[0] = -ball.velocity[0];
                }
            }

            for (paddle, transform) in (&paddles, &transforms).join() {
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
                        ball.velocity[1] = -ball.velocity[1];
                    }
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

    if pseudo_projection_length < 0.0 || pseudo_projection_length > line_direction_squared_length {
        false
    } else {
        /*
         * ||a||² = a1² + d², by the Pythagorean Theorem,
         *                    where d is the distance from the circle to the line.
         * d² = ||a||² - a1²
         * d² = ||a||² - pseudo_projection_length² / ||line_direction||²
         */
        let ppl2 = pseudo_projection_length * pseudo_projection_length;
        let d2 = dot_product(a, a) - ppl2 / line_direction_squared_length;
        d2 <= radius * radius
    }
}

fn dot_product(
    (x1, y1): (f32, f32),
    (x2, y2): (f32, f32),
) -> f32 {
    x1 * x2 + y1 * y2
}