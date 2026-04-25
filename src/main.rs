use macroquad::prelude::*;
#[macroquad::main("bountingball")]
async fn main() {
    let mut y = 0.0;
    let mut x = 2.0;
    let gravity = 0.5;
    let mut velotiy = 1.0;
    loop{
    

    clear_background(BLACK);
    velotiy = velotiy + gravity;
    y = y + velotiy;
    x = x + velotiy;
    if x > screen_width() - 40.0 {
        x = screen_width() - 40.0;
        velotiy = velotiy * - 0.75
    }
    if y > screen_height() - 40.0 {
        y = screen_height() - 40.0;
        velotiy = velotiy * -0.75
        
    }
    draw_circle(x,y, 40.0,WHITE, );
    next_frame().await;
    }
}
