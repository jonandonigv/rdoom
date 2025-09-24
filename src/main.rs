// Declares the external sdl2 library
extern crate sdl2;
use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::{map::Map, player::Player, renderer::Renderer};

mod assets;
mod combat;
mod enemy;
mod map;
mod player;
mod renderer;

const W_NAME: &str = "rdoom";
const W_HEIGHT: u32 = 800;
const W_WIDTH: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(W_NAME, W_HEIGHT, W_WIDTH)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let map = Map::new(20, 15);
    let mut player = Player::new(2.0, 2.0);
    let renderer = Renderer::new(32);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => player.handle_event(&event),
            }
        }
        player.update(&map, 1.0 / 60.0);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        renderer.render(&mut canvas, &map, &player);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
