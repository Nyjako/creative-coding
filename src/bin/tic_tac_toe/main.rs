use nannou::prelude::*;
use math::round;

const BOARD_SIZE: usize = 10;
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
    current: Vec2,
    finished: bool,
    win_pos: (Vec2, Vec2)
}

fn model(_app: &App) -> Model {

    Model {
        board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
        current_player: CellState::Player1,
        current: pt2(0.0, 0.0),
        finished: false,
        win_pos: (pt2(0.0, 0.0), pt2(0.0, 0.0))
    }
}

fn p_eq(p1: CellState, p2: CellState) -> bool {
    match p1 {
        CellState::Player1 => match p2 {
            CellState::Player1 => true,
            _ => false,
        }
        CellState::Player2 => match p2 {
            CellState::Player2 => true,
            _ => false,
        }
        _ => false
    }
}

fn check_win(board: [[CellState; BOARD_SIZE]; BOARD_SIZE], player: CellState) -> (bool, Vec2, Vec2) {
    let mut counter: [(i32, Vec2); 4] = [(0, pt2(0.0, 0.0));4];

    for i in 0..BOARD_SIZE {
        counter[0].0 = 0;
        counter[1].0 = 0;
        for j in 0..BOARD_SIZE {
            counter[2].0 = 0;
            counter[3].0 = 0;

            if p_eq(board[i][j], player) {
                if counter[0].0 == 0 { counter[0].1 = pt2(i as f32, j as f32); }
                counter[0].0 += 1;
                if counter[0].0 == REQUIRED as i32 {
                    return (true, counter[0].1, pt2(i as f32, j as f32));
                }
            }
            else {
                counter[0].0 = 0;
            }

            if p_eq(board[j][i], player) {
                if counter[1].0 == 0 { counter[1].1 = pt2(j as f32, i as f32); }
                counter[1].0 += 1;
                if counter[1].0 == REQUIRED as i32 {
                    return (true, counter[1].1, pt2(j as f32, i as f32));
                }
            }
            else {
                counter[1].0 = 0;
            }

            for x in 0..REQUIRED {
                if i + x < BOARD_SIZE && j + x < BOARD_SIZE {
                    if p_eq(board[i + x][j + x], player) {
                        if counter[2].0 == 0 { counter[2].1 = pt2((i+x) as f32, (j+x) as f32); }
                        counter[2].0 += 1;
                        if counter[2].0 == REQUIRED as i32 {
                            return (true, counter[2].1, pt2((i+x) as f32, (j+x) as f32));
                        }
                    }
                    else {
                        counter[2].0 = 0;
                    }
                }
                else {
                    counter[2].0 = 0;
                }

                if i + x < BOARD_SIZE && BOARD_SIZE as i32 - (j + x) as i32 - 1 >= 0 {
                    if p_eq(board[i + x][BOARD_SIZE - (j + x) - 1], player) {
                        if counter[3].0 == 0 { counter[2].1 = pt2((i+x) as f32, (BOARD_SIZE - (j + x) - 1) as f32); }
                        counter[3].0 += 1;
                        if counter[3].0 == REQUIRED as i32 {
                            return (true, counter[3].1, pt2((i+x) as f32, (BOARD_SIZE - (j + x) - 1) as f32));
                        }
                    }
                    else {
                        counter[3].0 = 0;
                    }
                }
                else {
                    counter[3].0 = 0;
                }
            }

        }
    }

    return  (false, pt2(0.0, 0.0), pt2(0.0, 0.0));
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.finished {
        ()
    }
    else {
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
            // println!("Pressed left mouse at X:{} Y:{}", app.mouse.x, app.mouse.y);
            match model.board[current_cell.x as usize][current_cell.y as usize] {
                CellState::Empty => {
                    model.board[current_cell.x as usize][current_cell.y as usize] = model.current_player;

                    let output = check_win(model.board, model.current_player);
                    model.finished = output.0;
                    if model.finished {
                        println!("Finished");
                        model.win_pos = (output.1, output.2);
                    }

                    match model.current_player {
                        CellState::Player1 => model.current_player = CellState::Player2,
                        CellState::Player2 => model.current_player = CellState::Player1,
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}

fn draw_board(draw: &Draw, board: [[CellState; BOARD_SIZE]; BOARD_SIZE], current: Vec2, win_pos: Option<(Vec2, Vec2)>) {
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

    if win_pos.is_some() {
        let pos = win_pos.unwrap();
        let temp_point1 = pt2(CELL_WIDTH * pos.0.x, HEIGHT as f32 - CELL_HEIGHT * pos.0.y);
        let temp_point2 = pt2(CELL_WIDTH * pos.1.x, HEIGHT as f32 - CELL_HEIGHT * pos.1.y);

        draw.line()
            .start(temp_point1 + pt2(CELL_WIDTH / 2.0, -(CELL_HEIGHT / 2.0)))
            .end(temp_point2 + pt2(CELL_WIDTH / 2.0, -(CELL_HEIGHT / 2.0)))
            .weight(5.0)
            .color(PINK);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(WHITE);

    let boundary = app.window_rect();
    let draw = app.draw().x_y(boundary.left(), boundary.bottom()); // (0,0) is now left bottom
    
    if model.finished {
        draw_board(&draw, model.board, model.current, Some(model.win_pos));
    }
    else {
        draw_board(&draw, model.board, model.current, None);
    }

    draw.to_frame(app, &frame).unwrap();
}
