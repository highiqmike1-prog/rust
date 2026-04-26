use macroquad::prelude::*;

struct Ball {
    x: f32,
    y: f32,
    xvelotiy: f32,
    yvelotiy: f32,
    radius: f32,
    color: Color,
}

#[macroquad::main("bountingball")]
async fn main() {
    let gravity = 0.5;
    let mut xoldmousepos = 1.0;
    let mut yoldmousepos = 1.0;
    let mut balls: Vec<Ball> = Vec::new();
    let mut dragging: Option<usize> = None;

    loop {
        let (mx, my) = mouse_position(); // inside loop so it updates every frame

        if is_mouse_button_pressed(MouseButton::Right) {
            balls.push(Ball {
                x: 2.0,
                y: 2.0,
                xvelotiy: 0.0,
                yvelotiy: 0.0,
                radius: 40.0,
                color: WHITE,
            });
        }

        // find which ball was clicked
        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, ball) in balls.iter().enumerate() {
                let dx = mx - ball.x;
                let dy = my - ball.y;
                if (dx * dx + dy * dy).sqrt() <= ball.radius {
                    // + not * for correct distance
                    dragging = Some(i);
                    xoldmousepos = mx;
                    yoldmousepos = my;
                    break;
                }
            }
        }

        // release the ball and give it velocity
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(i) = dragging {
                // shit is too much on release
                balls[i].xvelotiy = (mx - xoldmousepos)- 5;
                balls[i].yvelotiy = (my - yoldmousepos) - 5;
            }
            dragging = None; // stop dragging
        }

        clear_background(BLACK);

        for (i, ball) in balls.iter_mut().enumerate() {
            if is_mouse_button_down(MouseButton::Left) && dragging == Some(i) {
                // only move the ball we clicked on
                ball.x = mx;
                ball.y = my;
            } else {
                ball.y += ball.yvelotiy;
                ball.x += ball.xvelotiy;
                ball.yvelotiy += gravity;
            }

            // bouncing - inside the for loop
            if ball.x > screen_width() {
                ball.x = screen_width() - ball.radius;
                ball.xvelotiy *= -0.75;
            } else if ball.x < 0.0 {
                ball.x = ball.radius;
                ball.xvelotiy *= -0.75;
            }
            if ball.y > screen_height() {
                ball.y = screen_height() - ball.radius;
                ball.yvelotiy *= -0.75;
            } else if ball.y < 0.0 {
                ball.y = ball.radius;
                ball.yvelotiy *= -0.75;
            }

            draw_circle(ball.x, ball.y, ball.radius, ball.color);
        }

        next_frame().await; // inside the main loop
    }
}
