mod math;

pub struct Ball
{
    r: f32,
    x: f32,
    y: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    elasticity :f32,
}

impl Ball
{
    pub fn ball_colls(ball1 :&Ball, ball2 :&Ball) -> bool
    {
        if math::dist(vec![ball1.x, ball1.y], vec![ball2.x, ball2.y]) < ball1.r + ball2.r
        {
            return true;
        }
        false
    }

    pub fn solve_coll_balls(ball1 :&mut Ball, ball2 :&mut Ball)
    {
        for i in 2..100
        {
            if Ball::ball_colls(ball1, ball2)
            {
                ball1.x = ball1.x - ball1.speed_x / (i as f32 / 100f32) as f32;
                ball1.y = ball1.y - ball1.speed_y / (i as f32 / 100f32) as f32;
                ball2.x = ball2.x - ball2.speed_x / (i as f32 / 100f32) as f32;
                ball2.y = ball2.y - ball2.speed_y / (i as f32 / 100f32) as f32;
            } else { break; }
        }


        let vec1 :Vec<f32> = Ball::change_type_vec(&ball1.speed_x, &ball1.speed_y);
        let n2 :f32 = Ball::change_type_vec(&(ball2.x - ball1.x), &(ball2.y - ball1.y))[0];
        let vec2 :Vec<f32> = vec![n2 + vec1[0] - 180f32, vec1[1]];
        let vec3 :Vec<f32> = Ball::vec_to_s_xy(&vec2);
        ball1.speed_x = vec3[0];
        ball1.speed_y = vec3[1];
    }

    pub fn change_type_vec(speed_x :&f32, speed_y :&f32) -> Vec<f32>
    {
        let l :f32 = (speed_x * speed_x + speed_y * speed_y).sqrt();
        let n :f32 = (speed_x/l).acos();
        vec![n, l]
    }

    pub fn vec_to_s_xy(x_y :&Vec<f32>) -> Vec<f32>
    {
        let x_s = x_y[0].cos() * x_y[1];
        let y_s = x_y[0].sin() * x_y[1];
        vec![x_s, y_s]
    }

    fn resolve_collision(ball :&mut Ball)
    {
        use macroquad::prelude::*;

        if ball.x + ball.r > screen_width() || 
           ball.x - ball.r < 0f32
        {
            ball.speed_x = 
                -ball.speed_x / (ball.elasticity / 0.9);
        }

        if ball.y + ball.r > screen_height() || 
            ball.y - ball.r < 0f32
        {
            ball.speed_y = 
                -ball.speed_y / (ball.elasticity / 0.999);
        }
    }

    pub fn init(x :&f32, y :&f32, r :&f32) -> Ball
    {
        return 
            Ball {x : *x, 
                  y : *y, 
                  r : *r, 
                  speed_x : 0f32, 
                  speed_y : 0f32, 
                  elasticity : 1f32};
    }

    pub fn impulse(ball :&mut Ball, vec_imp :&Vec<f32>)
    {
        ball.speed_x += vec_imp[0];
        ball.speed_y += vec_imp[1];
    }

    pub fn print(ball :&Ball, ball2 :&Ball)
    {
        use macroquad::prelude::*;
        if Ball::ball_colls(ball, ball2) {
            draw_circle(ball.x, ball.y, ball.r, RED);
        } else {
            draw_circle(ball.x, ball.y, ball.r, WHITE);
        }
    }

    pub fn update(ball :&mut Ball)
    {
        ball.x += ball.speed_x;
        ball.y += ball.speed_y;
        ball.speed_x /= ball.elasticity * 1.02;
        ball.speed_y /= ball.elasticity * 1.02;
        Ball::resolve_collision(ball);
    }
}
