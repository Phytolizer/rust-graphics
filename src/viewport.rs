use crate::common::{Position, Size};

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
}