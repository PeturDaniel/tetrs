use macroquad::prelude::*;
use macroquad::rand::gen_range;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

type Board = Vec<[u8; WIDTH]>;

#[derive(Clone)]
struct Piece {
    x: i32,
    y: i32,
    kind: usize,
    rot: usize,
}

const TETROMINOES: [[[u8; 4]; 4]; 7] = [
    [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
];

fn block_size() -> f32 {
    let block_w = screen_width() / WIDTH as f32;
    let block_h = screen_height() / HEIGHT as f32;
    block_w.min(block_h)
}

fn board_offset() -> (f32, f32) {
    let b = block_size();
    let ox = (screen_width() - WIDTH as f32 * b) / 2.0;
    let oy = (screen_height() - HEIGHT as f32 * b) / 2.0;
    (ox, oy)
}

fn rotate(shape: [[u8; 4]; 4], r: usize) -> [[u8; 4]; 4] {
    let mut s = shape;
    for _ in 0..r {
        let mut t = [[0; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                t[x][3 - y] = s[y][x];
            }
        }
        s = t;
    }
    s
}

fn collides(board: &Board, p: &Piece) -> bool {
    let s = rotate(TETROMINOES[p.kind], p.rot);
    for y in 0..4 {
        for x in 0..4 {
            if s[y][x] == 0 {
                continue;
            }
            let nx = p.x + x as i32;
            let ny = p.y + y as i32;
            if nx < 0 || nx >= WIDTH as i32 || ny >= HEIGHT as i32 {
                return true;
            }
            if ny >= 0 && board[ny as usize][nx as usize] != 0 {
                return true;
            }
        }
    }
    false
}

fn merge(board: &mut Board, p: &Piece) {
    let s = rotate(TETROMINOES[p.kind], p.rot);
    for y in 0..4 {
        for x in 0..4 {
            if s[y][x] == 1 {
                let nx = p.x + x as i32;
                let ny = p.y + y as i32;
                if ny >= 0 {
                    board[ny as usize][nx as usize] = (p.kind + 1) as u8;
                }
            }
        }
    }
}

fn clear_lines(board: &mut Board) {
    board.retain(|r| r.iter().any(|&v| v == 0));
    while board.len() < HEIGHT {
        board.insert(0, [0; WIDTH]);
    }
}

fn draw_board_background() {
    let b = block_size();
    let (ox, oy) = board_offset();

    draw_rectangle(
        ox,
        oy,
        WIDTH as f32 * b,
        HEIGHT as f32 * b,
        Color::new(0.2, 0.15, 0.1, 1.0),
    );
}

fn draw_board(board: &Board) {
    let b = block_size();
    let (ox, oy) = board_offset();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[y][x] != 0 {
                draw_rectangle(
                    ox + x as f32 * b,
                    oy + y as f32 * b,
                    b - 1.0,
                    b - 1.0,
                    WHITE,
                );
            }
        }
    }
}

fn draw_piece(p: &Piece) {
    let s = rotate(TETROMINOES[p.kind], p.rot);
    let b = block_size();
    let (ox, oy) = board_offset();

    for y in 0..4 {
        for x in 0..4 {
            if s[y][x] == 1 {
                let px = p.x + x as i32;
                let py = p.y + y as i32;
                if py >= 0 {
                    draw_rectangle(
                        ox + px as f32 * b,
                        oy + py as f32 * b,
                        b - 1.0,
                        b - 1.0,
                        GRAY,
                    );
                }
            }
        }
    }
}

fn new_piece() -> Piece {
    Piece {
        x: 3,
        y: -1,
        kind: gen_range(0, 7),
        rot: 0,
    }
}

#[macroquad::main("Tetrs")]
async fn main() {
    let mut board: Board = vec![[0; WIDTH]; HEIGHT];
    let mut piece = new_piece();
    let mut timer = 0.0;
    let mut game_over = false;

    loop {
        clear_background(BLACK);

        draw_board_background();

        timer += get_frame_time();

        if !game_over {
            if is_key_pressed(KeyCode::Left) {
                let mut p = piece.clone();
                p.x -= 1;
                if !collides(&board, &p) {
                    piece = p;
                }
            }
            if is_key_pressed(KeyCode::Right) {
                let mut p = piece.clone();
                p.x += 1;
                if !collides(&board, &p) {
                    piece = p;
                }
            }
            if is_key_pressed(KeyCode::Up) {
                let mut p = piece.clone();
                p.rot = (p.rot + 1) % 4;
                if !collides(&board, &p) {
                    piece = p;
                }
            }
            if is_key_down(KeyCode::Down) {
                timer += 0.05;
            }

            if timer > 0.5 {
                timer = 0.0;
                let mut p = piece.clone();
                p.y += 1;
                if collides(&board, &p) {
                    if piece.y < 0 {
                        game_over = true;
                    } else {
                        merge(&mut board, &piece);
                        clear_lines(&mut board);
                        piece = new_piece();
                    }
                } else {
                    piece = p;
                }
            }
        }

        draw_board(&board);
        draw_piece(&piece);

        if game_over {
            let font_size = block_size() * 2.0;
            let text = "GAME OVER";
            let text_dims = measure_text(text, None, font_size as u16, 1.0);
            draw_text(
                text,
                (screen_width() - text_dims.width) / 2.0,
                screen_height() / 2.0,
                font_size,
                RED,
            );
        }

        next_frame().await;
    }
}
