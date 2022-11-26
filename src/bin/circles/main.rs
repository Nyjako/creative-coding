use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

struct Circle {
    position: Point2,
    velocity: Point2,
    speed:    f32,
    radius:   f32,
    color:    Color
}

struct Model {
    circles: Vec<Circle>
}
const CIRCLE_LIMIT: i32 = 150;

fn model(_app: &App) -> Model {
    let mut model = Model {
        circles: Vec::new()
    };

    while model.circles.len() < CIRCLE_LIMIT as usize {
        let vel_x = random::<f32>() * 2.0 - 1.0;
        let vel_y = random::<f32>() * 2.0 - 1.0;

        let new_circle = Circle {
            position: pt2(0.0, 0.0),
            velocity: pt2(vel_x, vel_y),
            speed: random::<f32>() * 10.0 + 10.0,
            radius: random::<f32>() * 50.0 + 50.0,
            color: Color {
                r: random::<f32>(),
                g: random::<f32>(),
                b: random::<f32>(),
                a: random::<f32>()
            }
        };

        model.circles.push(new_circle);
    }

    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();

    for i in 0..model.circles.len() {

        let circle = &model.circles[i];
        let mut new_pos = circle.position + circle.velocity * circle.speed;
        let mut new_vel = circle.velocity;

        if  new_pos.x > boundary.right() + circle.radius + 0.1 ||
            new_pos.x < boundary.left() - circle.radius - 0.1 ||
            new_pos.y > boundary.top() + circle.radius + 0.1 || 
            new_pos.y < boundary.bottom() - circle.radius - 0.1 
        {
            new_pos = pt2(0.0, 0.0);
        }
        else if  new_pos.x > boundary.right() - circle.radius ||
                 new_pos.x < boundary.left() + circle.radius {
            new_vel.x *= -1.0;
        }
        else if new_pos.y > boundary.top() - circle.radius || 
                new_pos.y < boundary.bottom() + circle.radius {
            new_vel.y *= -1.0;
        }

        model.circles[i].position = new_pos;
        model.circles[i].velocity = new_vel;
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(PINK);
    let draw = app.draw();

    for circle in model.circles.iter() {
        draw.ellipse()
            .xy(circle.position)
            .w_h(circle.radius * 2.0, circle.radius * 2.0)
            .rgba(circle.color.r, circle.color.g, circle.color.b, circle.color.a);

    }

    draw.to_frame(app, &frame).unwrap();
}