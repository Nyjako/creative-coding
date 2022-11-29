use nannou::prelude::*;
use math::round;

const BOARD_SIZE: usize = 3;
const REQUIRED: usize   = 3;
const WIDTH: u32        = 600;
const HEIGHT: u32       = 600;
const CELL_WIDTH: f32   = WIDTH as f32 / BOARD_SIZE as f32;
const CELL_HEIGHT: f32  = HEIGHT as f32 / BOARD_SIZE as f32;

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
    current_player: CellState,
    current: Vec2
}

fn model(app: &App) -> Model {

    Model {
        board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
        current_player: CellState::Player1,
        current: pt2(0.0, 0.0)
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.board[1][1] = CellState::Player1;
    model.board[2][2] = CellState::Player2;
    let button = app.mouse.buttons.left();

    let bound = app.window_rect();
    let mouse_pos = app.mouse.position() + pt2(bound.right(), bound.top());

    let current_cell = pt2(
        round::floor((mouse_pos.x / CELL_WIDTH) as f64, 0) as f32,
        BOARD_SIZE as f32 - 1.0 - round::floor((mouse_pos.y / CELL_HEIGHT) as f64, 0) as f32 
        // I need to reverse this because I start drawing from bottom left
    );

    model.current = current_cell;

    if button.is_down() {
        println!("Pressed left mouse at X:{} Y:{}", app.mouse.x, app.mouse.y);
    }
}

fn draw_board(draw: &Draw, board: [[CellState; BOARD_SIZE]; BOARD_SIZE], current: Vec2) {
    // println!("{} {}", CELL_HEIGHT, CELL_WIDTH);

    for j in 0..BOARD_SIZE {

        // Draw board
        if j != 0 && j != BOARD_SIZE {
            draw.line()
                .start(pt2(CELL_WIDTH * j as f32, 0.0))
                .end(pt2(CELL_WIDTH * j as f32, HEIGHT as f32))
                .weight(3.0).color(BLACK);
        }
        if j != 0 && j != BOARD_SIZE {
            draw.line()
                .start(pt2(0.0, CELL_HEIGHT * j as f32))
                .end(pt2(WIDTH as f32, CELL_HEIGHT * j as f32))
                .weight(3.0).color(BLACK);
        }

        for i in 0..BOARD_SIZE {

            // Fill board
            let temp_point = pt2(CELL_WIDTH * i as f32, HEIGHT as f32 - CELL_HEIGHT * j as f32);
            match board[i][j] {
                CellState::Player1 => {
                    draw.ellipse()
                        .xy(temp_point + pt2(CELL_WIDTH / 2.0, -(CELL_HEIGHT / 2.0)))
                        .w_h(CELL_WIDTH / 2.0, CELL_HEIGHT / 2.0)
                        .stroke_weight(3.0).stroke(BLACK);
                },
                CellState::Player2 => {
                    let center = temp_point + pt2(CELL_WIDTH / 2.0, -(CELL_HEIGHT / 2.0));

                    draw.line()
                        .start(center - pt2(CELL_WIDTH / 4.0, -(CELL_HEIGHT / 4.0)))
                        .end(center + pt2(CELL_WIDTH / 4.0, -(CELL_HEIGHT / 4.0)))
                        .weight(3.0).color(BLACK);

                    draw.line()
                        .start(center + pt2(CELL_WIDTH / 4.0, CELL_HEIGHT / 4.0))
                        .end(center - pt2(CELL_WIDTH / 4.0, CELL_HEIGHT / 4.0))
                        .weight(3.0).color(BLACK);
                },
                _ => {
                    if current.x == i as f32 && current.y == j as f32 {
                        draw.rect()
                            .xy(temp_point + pt2(CELL_WIDTH / 2.0, -(CELL_HEIGHT / 2.0)))
                            .w_h(CELL_WIDTH, CELL_HEIGHT)
                            .stroke_weight(3.0).stroke(BLACK).color(LIME);
                    }
                }
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(WHITE);

    let boundary = app.window_rect();
    let draw = app.draw().x_y(boundary.left(), boundary.bottom()); // (0,0) is now left bottom 
    draw_board(&draw, model.board, model.current);

    draw.to_frame(app, &frame).unwrap();
}
