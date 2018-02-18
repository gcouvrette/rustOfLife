extern crate sdl2;
extern crate rand;
mod game;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};
use sdl2::rect::Rect;

fn main() {
    let w = 500_usize;
    let h = 500_usize;
    let mut game = game::Game::new((w) as usize,(h) as usize);

    // Randomize initial state
    for y in 0..h {
        for x in 0..w {
            if rand::random::<bool>() {
                game.set_state(game::State::ALIVE, x as usize, y as usize);
            }
        }
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust of Life", (w*2) as u32, (h*2) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for y in 0..h {
            for x in 0..w {
                if game.get_state(x, y) == game::State::ALIVE {
                    canvas.fill_rect(Rect::new((x*2) as i32, (y*2) as i32, 2, 2));
                }
            }
        }
        canvas.present();
        game.step();
        thread::sleep(time::Duration::from_millis(10));
    }
}
