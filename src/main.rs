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
    let mut xoldmousepos = 1.0; // mouse x position when we first grabbed a ball
    let mut yoldmousepos = 1.0; // mouse y position when we first grabbed a ball
    let mut balls: Vec<Ball> = Vec::new(); // list of all balls
    let mut dragging: Option<usize> = None; // index of the ball being dragged, or None

    loop {
        let (mx, my) = mouse_position(); // get mouse position every frame so it stays updated

        // --- COLLISION BETWEEN BALLS ---
        // we check every pair of balls (i and j) exactly once
        // i starts at 0, j starts one ahead of i so we never check the same pair twice
        for i in 0..balls.len() {
            for j in (i + 1)..balls.len() {
                let dx = balls[i].x - balls[j].x; // horizontal distance between centers
                let dy = balls[i].y - balls[j].y; // vertical distance between centers
                let distance = (dx * dx + dy * dy).sqrt(); // straight line distance between centers

                // if distance is less than both radii added, they are overlapping!
                if distance < balls[i].radius + balls[j].radius {
                    // nx, ny = the direction from ball j to ball i
                    // dividing by distance makes it length 1 (normalized) so it is just a direction
                    let nx = dx / distance;
                    let ny = dy / distance;

                    // overlap = how many pixels the balls are sinking into each other
                    let overlap = (balls[i].radius + balls[j].radius) - distance;

                    // push ball i away by half the overlap in the direction nx, ny
                    balls[i].x += nx * overlap / 2.0;
                    balls[i].y += ny * overlap / 2.0;

                    // push ball j away by half the overlap in the OPPOSITE direction
                    balls[j].x -= nx * overlap / 3.0;
                    balls[j].y -= ny * overlap / 2.0;

                    // dot1 = how fast ball i is moving TOWARD ball j along the collision direction
                    let dot1 = balls[i].xvelotiy * nx + balls[i].yvelotiy * ny;

                    // dot2 = how fast ball j is moving TOWARD ball i along the collision direction
                    let dot2 = balls[j].xvelotiy * nx + balls[j].yvelotiy * ny;

                    // swap their speeds along the collision direction so they bounce off each other
                    // (dot2 - dot1) means: remove ball i's toward-speed, add ball j's toward-speed
                    balls[i].xvelotiy += (dot2 - dot1) * nx;
                    balls[i].yvelotiy += (dot2 - dot1) * ny;

                    // (dot1 - dot2) is the opposite swap for ball j
                    balls[j].xvelotiy += (dot1 - dot2) * nx;
                    balls[j].yvelotiy += (dot1 - dot2) * ny;
                }
            }
        }

        // --- SPAWN BALL ON RIGHT CLICK ---
        if is_mouse_button_pressed(MouseButton::Right) {
            balls.push(Ball {
                x: mx,
                y: my,
                xvelotiy: 0.0,
                yvelotiy: 0.0,
                radius: 40.0,
                color: WHITE,
            });
        }

        // --- FIND WHICH BALL WAS CLICKED ---
        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, ball) in balls.iter().enumerate() {
                let dx = mx - ball.x; // horizontal gap between cursor and ball center
                let dy = my - ball.y; // vertical gap between cursor and ball center

                // if the distance from cursor to ball center is less than the radius
                // the cursor is inside the ball so we start dragging it
                if (dx * dx + dy * dy).sqrt() <= ball.radius {
                    dragging = Some(i); // remember this ball's index
                    xoldmousepos = mx; // save where we grabbed it from
                    yoldmousepos = my;
                    break; // stop looking, we found our ball
                }
            }
        }

        // --- RELEASE THE BALL AND THROW IT ---
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(i) = dragging {
                // velocity = how far the mouse moved since we grabbed the ball
                // this makes it feel like you are throwing it
                balls[i].xvelotiy = mx - xoldmousepos;
                balls[i].yvelotiy = my - yoldmousepos;
            }
            dragging = None; // we are no longer dragging anything
        }

        clear_background(BLACK);

        // --- UPDATE AND DRAW EACH BALL ---
        for (i, ball) in balls.iter_mut().enumerate() {
            if is_mouse_button_down(MouseButton::Left) && dragging == Some(i) {
                // this is the ball we are dragging so move it to the cursor
                ball.x = mx;
                ball.y = my;
            } else {
                // not being dragged so apply physics
                ball.x += ball.xvelotiy; // move by its horizontal speed
                ball.y += ball.yvelotiy; // move by its vertical speed
                ball.yvelotiy += gravity; // gravity pulls it down every frame
            }

            // --- WALL BOUNCING ---
            if ball.x > screen_width() {
                ball.x = screen_width() - ball.radius; // stop it going off screen
                ball.xvelotiy *= -0.75; // flip speed and lose some energy
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

        next_frame().await; // wait for the next frame (keeps the game running at the right speed)
    }
}
