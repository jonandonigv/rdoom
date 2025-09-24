use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::map::Map;
use crate::player::Player;

pub struct Renderer {
    tile_size: i32,
}

impl Renderer {
    pub fn new(tile_size: i32) -> Self {
        Renderer { tile_size }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, map: &Map, player: &Player) {
        let (width, height) = map.get_dimensions();
        let tiles = map.get_tiles();

        // Render Map
        for y in 0..height {
            for x in 0..width {
                let color = match tiles[y as usize][x as usize] {
                    'W' => Color::RGB(100, 100, 100), // Gray walls
                    _ => Color::RGB(50, 50, 50),      // Dark gray tiles
                };
                canvas.set_draw_color(color);
                canvas
                    .fill_rect(Rect::new(
                        x * self.tile_size,
                        y * self.tile_size,
                        self.tile_size as u32,
                        self.tile_size as u32,
                    ))
                    .unwrap();
            }
        }

        // Render Player
        let (px, py) = player.get_position();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(todo!())).unwrap();
    }
}
