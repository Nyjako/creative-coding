use nannou::prelude::*;

const WIDTH: u32  = 800;
const HEIGHT: u32 = 800;
const COLS: i32   = 8;
const ROWS: i32   = 8;
const STROKE: f32 = 3.0;

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
    maze: [[Cell; ROWS as usize]; COLS as usize]
}

fn model(_app: &App) -> Model {
    let temp: [Cell; ROWS as usize] = [Cell {
        top:    true,
        bottom: true,
        left:   true,
        right:  true
    }; ROWS as usize];



    Model {
        maze: [temp; COLS as usize]
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn draw_maze(maze: [[Cell; ROWS as usize]; COLS as usize], draw: &Draw, boundary: Rect) {
    let width  = boundary.left().abs() + boundary.right().abs();
    let height = boundary.bottom().abs() + boundary.top().abs();

    let w_spacing = width / COLS as f32;
    let h_spacing = height / ROWS as f32;

    for j in 0..COLS {
        for i in 0..ROWS {
            let temp = maze[i as usize][j as usize];

            let temp_x = boundary.left() + w_spacing * i as f32;
            let temp_y = boundary.top()  - h_spacing * (j + 1) as f32;

            if temp.top {
                draw.line().start(pt2(temp_x, temp_y)).end(pt2(temp_x + w_spacing, temp_y)).color(BLACK).weight(STROKE);
            }
            if temp.bottom {
                draw.line().start(pt2(temp_x, temp_y + h_spacing)).end(pt2(temp_x + w_spacing, temp_y + h_spacing)).color(BLACK).weight(STROKE);
            }
            if temp.left {
                draw.line().start(pt2(temp_x, temp_y)).end(pt2(temp_x, temp_y + h_spacing)).color(BLACK).weight(STROKE);
            }
            if temp.right {
                draw.line().start(pt2(temp_x + w_spacing, temp_y)).end(pt2(temp_x + w_spacing, temp_y + h_spacing)).color(BLACK).weight(STROKE);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let boundary = app.window_rect();
    frame.clear(WHITE);
    let draw = app.draw();

    draw_maze(model.maze, &draw, boundary);

    draw.to_frame(app, &frame).unwrap();
}