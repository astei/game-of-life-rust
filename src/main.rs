extern crate sdl2;

mod board;

use board::ConwayBoard;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use std::time::Duration;
use std::time::Instant;

static SIXTY_FPS_BUDGET: Duration = Duration::new(0, 1_000_000_000u32 / 60);

fn draw_board(board: &ConwayBoard, canvas: &mut Canvas<Window>, scalar: usize) {
    let scalar_u32 = scalar as u32;

    let mut live_cells = Vec::new();
    let mut dead_cells = Vec::new();

    for x in 0..board.width {
        for y in 0..board.height {
            let cell_rect = Rect::new(
                (x * scalar) as i32,
                (y * scalar) as i32,
                scalar_u32,
                scalar_u32
            );

            if board.get(x, y) {
                live_cells.push(cell_rect);
            } else {
                dead_cells.push(cell_rect);
            }
        }
    }

    canvas.set_draw_color(Color::RGB(0, 128, 0));
    canvas.fill_rects(&live_cells[..]).unwrap();

    canvas.set_draw_color(Color::RGB(128, 0, 0));
    canvas.fill_rects(&&dead_cells[..]).unwrap();

    // draw a grid
    if scalar > 1 {
        canvas.set_draw_color(Color::RGB(128, 128, 128));
        for x in 0..board.width {
            canvas.draw_line(
                Point::new((x * scalar) as i32, 0),
                Point::new((x * scalar) as i32, (board.height * scalar) as i32)
            ).unwrap();
        }

        for y in 0..board.height {
            canvas.draw_line(Point::new(0, (y * scalar) as i32), Point::new((board.width * scalar) as i32, (y * scalar) as i32));
        }
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let board_width = 200;
    let board_height = 100;
    let scalar = 6;

    let mut board = ConwayBoard::new_drylife(board_width, board_height);
    board.randomize();

    let window = video_subsystem.window(
        "game of life", (board_width * scalar).try_into().unwrap(), (board_height * scalar).try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rounds = 0;
    let mut frames = 0;
    let mut paused = true;

    let mut prev_times = vec![Instant::now(); 60];

    'running: loop {
        let now = Instant::now();

        canvas.clear();
        draw_board(&board, &mut canvas, scalar);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    board.randomize();
                    paused = true;
                    rounds = 0;
                },
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    let board_x = (x / scalar as i32).unsigned_abs().try_into().unwrap();
                    let board_y = (y / scalar as i32).unsigned_abs().try_into().unwrap();
                    let new_value = !board.get(board_x, board_y);
                    board.set(board_x, board_y, new_value);
                    println!("x: {} y: {}", x, y);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        if !paused {
            board.simulate();
            rounds += 1;
        }

        canvas.present();

        let ideal_fps = prev_times.len();
        let now2 = Instant::now();
        let elapsed = now2 - now;
        let fps = ideal_fps as f64 / (now2 - prev_times[frames % ideal_fps]).as_secs_f64();

        canvas.window_mut().set_title(format!("game of life (rounds: {}, paused: {}, fps: {:.2})", rounds, paused, fps).as_str());
        prev_times[frames % ideal_fps] = now2;
        frames += 1;

        println!("{}", fps);

        if elapsed < SIXTY_FPS_BUDGET {
            // We have some time to spare, go to sleep
            ::std::thread::sleep(SIXTY_FPS_BUDGET - elapsed);
        }
    }
}