use nannou::prelude::*;
use rand::random;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    position: Point2,
    velocity: Point2
}
const SPEED: f32 = 6.0;
const RADIUS: f32 = 25.0;

fn model(_app: &App) -> Model {
    Model {
        position: pt2(0.0, 0.0),
        velocity: pt2(random::<f32>(), random::<f32>())
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();

    let mut new_pos = model.position + model.velocity * SPEED;

    if new_pos.x + RADIUS > boundary.right() {
        new_pos.x = boundary.right() - RADIUS;
        model.velocity.x *= -1.0;
    }
    else if new_pos.x - RADIUS < boundary.left() {
        new_pos.x = boundary.left() + RADIUS;
        model.velocity.x *= -1.0;
    }

    if new_pos.y + RADIUS > boundary.top() {
        new_pos.y = boundary.top() - RADIUS;
        model.velocity.y *= -1.0;
    }
    else if new_pos.y - RADIUS < boundary.bottom() {
        new_pos.y = boundary.bottom() + RADIUS;
        model.velocity.y *= -1.0;
    }

    model.position = new_pos;
    
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(PINK);
    let draw = app.draw();

    draw.ellipse().xy(model.position).w_h(RADIUS * 2.0, RADIUS * 2.0).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}