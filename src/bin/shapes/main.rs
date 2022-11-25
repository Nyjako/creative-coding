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

    draw.background().color(PURPLE);

    draw.rect().x_y(0.0, 0.0).w_h(200.0, 200.0).color(GRAY);

    draw.line().start( pt2(200.0, 200.0) ).end( pt2(-200.0, -200.0) ).weight(3.0).color(BLUE);

    draw.ellipse().x_y(-200.0, 200.0).w_h(50.0, 50.0).color(PINK);

    draw.arrow().start(pt2(200.0, 200.0)).end(pt2(-200.0, 200.0)).weight(2.0).color(GREEN);

    draw.text("Nyjako was here.").center_justify().x_y(0.0, -200.0).font_size(26).color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}