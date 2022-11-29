use nannou::prelude::*;

const BOARD_SIZE: usize = 3;
const REQUIRED: usize   = 3;
const WIDTH: u32        = 800;
const HEIGHT: u32       = 800;

#[derive(Copy, Clone)]
enum CellState {
    Player1,
    Player2,
    Empty
}

fn main() {
    nannou::app(model)
        .size(WIDTH, HEIGHT)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    board: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: CellState
}

fn model(_app: &App) -> Model {

    Model {
        board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
        current_player: CellState::Player1
    }
}

fn update(app: &App, _model: &mut Model, _update: Update) {

    let button = app.mouse.buttons.left();

    if button.is_down() {
        println!("Pressed left mouse at X:{} Y:{}", app.mouse.x, app.mouse.y);
    }
}

fn view(app: &App, _model: &Model, frame: Frame){
    frame.clear(WHITE);
    let draw = app.draw();
}