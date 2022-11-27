use nannou::prelude::*;
use rand::random;

const WIDTH: u32  = 800;
const HEIGHT: u32 = 800;
const COLS: u32   = 8;
const ROWS: u32   = 8;
const STROKE: f32 = 3.0;

const W_SPACING: f32 = (WIDTH / COLS) as f32;
const H_SPACING: f32 = (HEIGHT / ROWS) as f32;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIDTH, HEIGHT)
        .run();
}

#[derive(Copy, Clone)]
struct Cell {
    top:    bool,
    bottom: bool,
    left:   bool,
    right:  bool
}

struct Model {
    maze: [[Cell; ROWS as usize]; COLS as usize],
    walker: Vec2,
    history: Vec<Vec2>
}

fn model(_app: &App) -> Model {
    let temp: [Cell; ROWS as usize] = [Cell {
        top:    true,
        bottom: true,
        left:   true,
        right:  true
    }; ROWS as usize];



    Model {
        maze: [temp; COLS as usize],
        walker: pt2(0.0, 0.0),
        history: Vec::new()
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn draw_maze(maze: [[Cell; ROWS as usize]; COLS as usize], draw: &Draw) {
    // let width  = boundary.left().abs() + boundary.right().abs();
    // let height = boundary.bottom().abs() + boundary.top().abs();

    for j in 0..COLS {
        for i in 0..ROWS {
            let temp = maze[i as usize][j as usize];

            let temp_x = W_SPACING * i as f32;
            let temp_y = H_SPACING * j as f32;

            if temp.top {
                draw.line().start(pt2(temp_x, temp_y)).end(pt2(temp_x + W_SPACING, temp_y)).color(BLACK).weight(STROKE);
            }
            if temp.bottom {
                draw.line().start(pt2(temp_x, temp_y + H_SPACING)).end(pt2(temp_x + W_SPACING, temp_y + H_SPACING)).color(BLACK).weight(STROKE);
            }
            if temp.left {
                draw.line().start(pt2(temp_x, temp_y)).end(pt2(temp_x, temp_y + H_SPACING)).color(BLACK).weight(STROKE);
            }
            if temp.right {
                draw.line().start(pt2(temp_x + W_SPACING, temp_y)).end(pt2(temp_x + W_SPACING, temp_y + H_SPACING)).color(BLACK).weight(STROKE);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let boundary = app.window_rect();
    frame.clear(WHITE);
    let draw = app.draw().x_y(boundary.left(), boundary.bottom()); // (0,0) is now left bottom 

    let spacing = pt2(W_SPACING, H_SPACING);

    draw.rect().xy(model.walker * spacing + spacing * 0.5).wh(spacing).color(LIME);
    draw_maze(model.maze, &draw);

    draw.to_frame(app, &frame).unwrap();
}