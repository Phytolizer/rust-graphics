use crate::common::{Position, Size};
use sdl2::render::WindowCanvas;
use crate::world::World;
use sdl2::rect::Rect;
use sdl2::video::WindowPos::Positioned;

pub(crate) struct Viewport {
    pos: Position,
    output_dimensions: Size,
    zoom_factor: f32,
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
    pub fn set_zoom(&mut self, zoom_factor: f32) {
        self.zoom_factor = zoom_factor;
    }
    pub fn render(&self, canvas: &mut WindowCanvas, world: &World) -> Result<(), String> {
        let bounds: Rect = Rect::new(self.pos.x - (self.output_dimensions.w / 2 * self.zoom_factor) as i32,
                                     self.pos.y - (self.output_dimensions.h / 2 * self.zoom_factor) as i32,
                                     self.output_dimensions.w * self.zoom_factor,
                                     self.output_dimensions.h * self.zoom_factor
        );
    }
}