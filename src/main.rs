use std::option;

use macroquad::prelude::{camera::mouse, *};
struct Ball{
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
    let mut xnewmousepos = 1.0;
    let mut ynewmousepos = 1.0;
    let mut balls:Vec<Ball> = Vec::new(); 
   
    
    loop{
        if is_mouse_button_pressed(MouseButton::Right){
            balls.push(Ball { x: (2.0), y: (2.0), xvelotiy: (0.0), yvelotiy: (0.0), radius: (40.0), color: (WHITE) });
        }
    

    clear_background(BLACK);
    for Ball in balls.iter_mut(){
    if  is_mouse_button_down(MouseButton::Left){
        (Ball.x,Ball.y) = mouse_position();
        (xoldmousepos,yoldmousepos) = mouse_position();
        
        
    }else if is_mouse_button_released(MouseButton::Left) {
    (xnewmousepos,ynewmousepos) = mouse_position();
    Ball.xvelotiy = (xnewmousepos).abs() - (xoldmousepos).abs();
    Ball.yvelotiy = (ynewmousepos).abs() - (yoldmousepos).abs();
    }else {
    
    Ball.y = Ball.y + Ball.yvelotiy;
    Ball.x = Ball.x + Ball.xvelotiy;
    Ball.yvelotiy = Ball.yvelotiy + gravity;
    }
    if Ball.x > screen_width()  {
        Ball.x = screen_width() - Ball.radius;
        Ball.xvelotiy *= -0.75
    }else if Ball.x < 0.0 {
        Ball.x = Ball.radius;
        Ball.xvelotiy *= -0.75;
        
    }
    if Ball.y > screen_height() {
        Ball.y = screen_height() - Ball.radius;
        Ball.yvelotiy *=  -0.75
        
    }else if Ball.y < 0.0 {
        Ball.y = Ball.radius; 
        Ball.yvelotiy *= -0.75;

    }
    draw_circle(Ball.x,Ball.y, Ball.radius,Ball.color, );
}
    next_frame().await;

    }
}
