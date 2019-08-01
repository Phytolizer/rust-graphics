use crate::common::{Position, Size};
use sdl2::render::WindowCanvas;
use crate::world::World;
use sdl2::rect::Rect;
use sdl2::video::WindowPos::Positioned;
use crate::tile::{Tile, TILE_SIZE};
use sdl2::event::EventType::TextInput;

pub(crate) struct Viewport {
    pos: Position,
    output_dimensions: Size,
    zoom_factor: f64,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            pos: Position { x: 0, y: 0 },
            output_dimensions: Size { w: 0, h: 0 },
            zoom_factor: 1.0,
        }
    }
    pub fn set_pos(&mut self, pos: Position) {
        self.pos = pos;
    }
    pub fn set_output_dimensions(&mut self, output_dimensions: Size) {
        self.output_dimensions = output_dimensions;
    }
    pub fn set_zoom(&mut self, zoom_factor: f64) {
        self.zoom_factor = zoom_factor;
    }
    pub fn render(&self, canvas: &mut WindowCanvas, world: &World) -> Result<(), String> {
       let world_bounds: Rect = Rect::new(
           self.pos.x - ((self.output_dimensions.w / 2) as f64 / self.zoom_factor / TILE_SIZE as f64) as i32,
           self.pos.y - ((self.output_dimensions.h / 2) as f64 / self.zoom_factor / TILE_SIZE as f64) as i32,
           (self.output_dimensions.w as f64 / self.zoom_factor / TILE_SIZE as f64) as u32,
           (self.output_dimensions.h as f64 / self.zoom_factor / TILE_SIZE as f64) as u32
        );
        for i in bounds.left()..bounds.right() {
            for j in bounds.top()..bounds.bottom() {
                if i < 0 || j < 0 || i > world.width() || j > world.height() {
                    continue;
                }
                let t: &Tile = world.get_tile(i as usize, j as usize).unwrap();
                // TODO t.render(tile_atlases, canvas, Rect::new())
            }
        }
        Ok(())
    }
}