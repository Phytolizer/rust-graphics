use crate::common::{Position, Size};

struct Viewport {
    pos: Position,
    output_dimensions: Size,
    zoom_factor: f32,
}

impl Viewport {
    fn new() -> Viewport {
        Viewport {
            pos: Position { x: 0, y: 0 },
            output_dimensions: Size { w: 0, h: 0 },
            zoom_factor: 1.0,
        }
    }
}