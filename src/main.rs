use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
    let draw = app.draw();
    draw.background().color(BLUE);
    draw.rect().x_y(0.0, 0.0).w_h(200.0, 200.0).color(GRAY);
    draw.to_frame(app, &frame).unwrap();
}