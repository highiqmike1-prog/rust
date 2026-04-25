use std::option;

use macroquad::prelude::*;
#[macroquad::main("bountingball")]
async fn main() {
    let mut y = 0.0;
    let mut x = 2.0;
    let gravity = 0.5;
    let mut yvelotiy = 1.0;
    let mut xvelotiy = 1.0;
    let mut xoldmousepos = 1.0;
    let mut yoldmousepos = 1.0;
    let mut xnewmousepos = 1.0;
    let mut ynewmousepos = 1.0;
    loop{
    

    clear_background(BLACK);
    
    if  is_mouse_button_down(MouseButton::Left){
        (x,y) = mouse_position();
        (xoldmousepos,yoldmousepos) = mouse_position();
        
        
    }else if is_mouse_button_released(MouseButton::Left) {
    (xnewmousepos,ynewmousepos) = mouse_position();
    xvelotiy = (xnewmousepos).abs() - (xoldmousepos).abs();
    yvelotiy = (ynewmousepos).abs() - (yoldmousepos).abs();
    }else {
    
    y = y + yvelotiy;
    x = x +xvelotiy;
    yvelotiy = yvelotiy + gravity;
    }
    if x > screen_width()  {
        x = screen_width() - 40.0;
        xvelotiy = xvelotiy * - 0.75
    }
    if y > screen_height() {
        y = screen_height() - 40.0;
        yvelotiy = yvelotiy * -0.75
        
    }
    draw_circle(x,y, 40.0,WHITE, );
    next_frame().await;

    }
}
