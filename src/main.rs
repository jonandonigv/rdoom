// Declares the external sdl2 library
extern crate sdl2;
use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

mod assets;
mod combat;
mod enemy;
mod map;
mod player;
mod renderer;

fn main() {
    // Initializes the sdl2 library, returning a context object that manages
    // sdl2 resources. unwrap assumes success and panics on failure
    let sdl_context = sdl2::init().unwrap();
    // Initializes the video subsytem for graphics and window management
    let video_subsystem = sdl_context.video().unwrap();

    // Creates a window with the title 'rdoom' and dimensions 800x600 pixels
    // .position_centered() Centers the window on the screen
    // .build() Construcst the window
    let window = video_subsystem
        .window("rdoom", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    // Converts the window into a Canvas, which is used for rendering graphics.
    // into_canvas() Creates a hardware-accelerated rendering context.
    // build() Finalizes the canvas creation
    let mut canvas = window.into_canvas().build().unwrap();

    // set_draw_color(Color::RGB(0, 255, 255)) Sets the drawing color to cyan
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    // Fills the entire canvas with the current draw color
    canvas.clear();
    // Updates the window to display the cleared (cyan) canvas
    canvas.present();
    // Creates an event pump to handle input and system events (e.g. keyboards, mouse, window
    // events)
    let mut event_pump = sdl_context.event_pump().unwrap();
    // Initializes a counter i to cylce through color values
    let mut i = 0;
    // Defines a labeled loop named running that continues until broken
    'running: loop {
        // Increments i and wraps it around at 255 (to stay within RGB values: 0-255)
        i = (i + 1) % 255;
        // Sets the canvas color to an RGB value where:
        // red channel: i (increases from 0 to 254, then wraps to 0)
        // green channel: Fixed at 64
        // blue channel: 255 - i (decreases from 255 to 1, then wraps to 255)
        // this creates a smooth color transition effect
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // fills the canvas with the new color
        canvas.clear();
        // Retrieves all pending events
        for event in event_pump.poll_iter() {
            // Check each event
            match event {
                // Triggered when the user closes the window (e.g. clicking the close button)
                Event::Quit { .. }
                // Triggered when the Escape key is pressed
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                        // If either occurs, break 'running exits the running loop, ending the
                        // program
                } => break 'running,
                // Ignores all other events
                _ => {}
            }
        }
        // Updates the window to show the final frame
        canvas.present();
        // Pauses the program for approx 1/60th of a second, simulating a 60fps framerate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
