use nannou::prelude::*;
use rand::seq::SliceRandom;

const WIDTH: u32  = 800;
const HEIGHT: u32 = 800;
const COLS: u32   = 50;
const ROWS: u32   = 50;
const STROKE: f32 = 1.0;

const W_SPACING: f32 = (WIDTH / COLS) as f32;
const H_SPACING: f32 = (HEIGHT / ROWS) as f32;

const BG_COL:      rgb::Srgb<u8> = WHITE;
const WALLS_COL:   rgb::Srgb<u8> = BLACK;
const WALKER_COL:  rgb::Srgb<u8> = LIME;
const HISTORY_COL: rgb::Srgb<u8> = PINK;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WIDTH, HEIGHT)
        .run();
}

#[derive(Copy, Clone)]
struct Cell {
    top:     bool,
    bottom:  bool,
    left:    bool,
    right:   bool,
    visited: bool
}

struct Model {
    maze: [[Cell; ROWS as usize]; COLS as usize],
    walker: Vec2,
    history: Vec<Vec2>
}

fn model(_app: &App) -> Model {
    let temp: [Cell; ROWS as usize] = [Cell {
        top:     true,
        bottom:  true,
        left:    true,
        right:   true,
        visited: false
    }; ROWS as usize];



    Model {
        maze: [temp; COLS as usize],
        walker: pt2(0.0, 0.0),
        history: Vec::new()
    }
}

fn visited(pos: Vec2, maze: &[[Cell; ROWS as usize]; COLS as usize]) -> bool {
    maze[pos.x as usize][pos.y as usize].visited
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.maze[model.walker.x as usize][model.walker.y as usize].visited = true;
    
    let mut posible_moves: Vec<Vec2> = Vec::new();

    if model.walker.y > 0.0 && !visited(pt2(model.walker.x, model.walker.y - 1.0), &model.maze) {
        posible_moves.push(pt2(model.walker.x, model.walker.y - 1.0));
    }
    if model.walker.y < (ROWS - 1) as f32 && !visited(pt2(model.walker.x, model.walker.y + 1.0), &model.maze) {
        posible_moves.push(pt2(model.walker.x, model.walker.y + 1.0));
    }
    if model.walker.x > 0.0 && !visited(pt2(model.walker.x - 1.0, model.walker.y), &model.maze) {
        posible_moves.push(pt2(model.walker.x - 1.0, model.walker.y));
    }
    if model.walker.x < (COLS - 1) as f32 && !visited(pt2(model.walker.x + 1.0, model.walker.y), &model.maze) {
        posible_moves.push(pt2(model.walker.x + 1.0, model.walker.y));
    }

    if posible_moves.len() > 0 {
        let random_move = posible_moves.choose(&mut rand::thread_rng()).unwrap();
        let temp_diff = *random_move - model.walker;
        let x = temp_diff.x;
        let y = temp_diff.y;
        let walker_x = model.walker.x as usize;
        let walker_y = model.walker.y as usize;

        model.history.push(model.walker);

        if x > 0.0 {
            model.maze[walker_x][walker_y].right = false;
            model.maze[walker_x + 1][walker_y].left = false;
        } else if x < 0.0 {
            model.maze[walker_x][walker_y].left = false;
            model.maze[walker_x - 1][walker_y].right = false;
        } else if y < 0.0 {
            model.maze[walker_x][walker_y].bottom = false;
            model.maze[walker_x][walker_y - 1].top = false;
        } else if y > 0.0 {
            model.maze[walker_x][walker_y].top = false;
            model.maze[walker_x][walker_y + 1].bottom = false;
        }

        model.walker = *random_move;
    }
    else if model.history.len() > 0 {
        model.walker = model.history.pop().unwrap();
    }
}

fn draw_maze(maze: [[Cell; ROWS as usize]; COLS as usize], draw: &Draw) {

    for j in 0..COLS {
        for i in 0..ROWS {
            let temp = maze[i as usize][j as usize];

            let temp_x = W_SPACING * i as f32;
            let temp_y = H_SPACING * j as f32;

            if temp.bottom && j != 0 {
                if !maze[i as usize][j as usize - 1].top {
                    draw.line()
                        .start(pt2(temp_x, temp_y))
                        .end(pt2(temp_x + W_SPACING, temp_y))
                        .color(WALLS_COL).weight(STROKE);
                }
            }
            if temp.top && j != COLS - 1 {
                    draw.line()
                        .start(pt2(temp_x, temp_y + H_SPACING))
                        .end(pt2(temp_x + W_SPACING, temp_y + H_SPACING))
                        .color(WALLS_COL).weight(STROKE);
            }
            if temp.left && i != 0 {
                if !maze[i as usize - 1][j as usize].right {
                    draw.line()
                        .start(pt2(temp_x, temp_y))
                        .end(pt2(temp_x, temp_y + H_SPACING))
                        .color(WALLS_COL).weight(STROKE);
                }
            }
            if temp.right && i != ROWS - 1 {
                draw.line()
                    .start(pt2(temp_x + W_SPACING, temp_y))
                    .end(pt2(temp_x + W_SPACING, temp_y + H_SPACING))
                    .color(WALLS_COL).weight(STROKE);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let boundary = app.window_rect();
    frame.clear(BG_COL);
    let draw = app.draw().x_y(boundary.left(), boundary.bottom()); // (0,0) is now left bottom 

    let spacing = pt2(W_SPACING, H_SPACING);

    for i in model.history.iter() {
        draw.rect().xy(*i * spacing + spacing * 0.5).wh(spacing).color(HISTORY_COL);
    }

    draw.rect().xy(model.walker * spacing + spacing * 0.5).wh(spacing).color(WALKER_COL);
    draw_maze(model.maze, &draw);

    draw.to_frame(app, &frame).unwrap();
}