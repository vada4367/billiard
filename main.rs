use macroquad::prelude::*;

mod balls;

fn window_conf() -> Conf {
    Conf {
        window_title: "Billiard phys".to_owned(),
        window_width: 800,
        window_height: 430,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]

async fn main() { 

    let mut ball1 
        :balls::Ball = 
        balls::Ball::init(&(screen_width() / 1.1f32 - 17f32), 
                          &(screen_height() / 2f32 + 60f32), 
                          &10f32);
    
    let mut ball2
        :balls::Ball = 
        balls::Ball::init(&(screen_width() / 1.1f32), 
                          &(screen_height() / 2f32), 
                          &10f32);


    balls::Ball::impulse(&mut ball1, &vec![0f32, -16f32]);
    balls::Ball::impulse(&mut ball2, &vec![0.001f32, 0.001f32]);

    loop
    {
        clear_background(GREEN);
        
        balls::Ball::update(&mut ball2);
        balls::Ball::update(&mut ball1);

        if balls::Ball::ball_colls(&ball1, &ball2)
        {
            balls::Ball::solve_coll_balls(&mut ball1, &mut ball2);
        }

        balls::Ball::print(&ball1, &ball2);
        balls::Ball::print(&ball2, &ball1);
            
        let mut Str :String = (balls::Ball::change_type_vec(&ball1.speed_x, &ball1.speed_y)[0] as i32).to_string();
        Str.push_str(&(balls::Ball::change_type_vec(&ball1.speed_x, &ball1.speed_y)[1] as i32).to_string());


        draw_text(&Str, screen_width() / 2f32, screen_height() / 2f32, 50.0, WHITE);

        next_frame().await
    }
}
