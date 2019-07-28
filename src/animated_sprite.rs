use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::image::LoadSurface;
use crate::common::Size;

pub struct AnimatedSprite<'a> {
    sprite_sheet: Option<sdl2::render::Texture<'a>>,
    frame_width: usize,
    frame_height: usize,
    num_frames: (usize, usize),
    custom_frame_sequence: Option<Vec<(usize, usize)>>,
    frame_sequence_length: usize,
}

impl<'a> AnimatedSprite<'a> {
    pub fn new() -> AnimatedSprite<'a> {
        AnimatedSprite {
            sprite_sheet: None,
            frame_width: 0,
            frame_height: 0,
            num_frames: (0, 0),
            custom_frame_sequence: None,
            frame_sequence_length: 0,
        }
    }
    pub fn set_num_frames(&mut self, n: (usize, usize)) {
        self.num_frames = n;
    }
    pub fn set_custom_frame_sequence(&mut self, seq: Option<Vec<(usize, usize)>>) {
        if let Some(s) = &seq {
            self.frame_sequence_length = s.len();
        } else {
            self.frame_sequence_length = 0;
        }
        self.custom_frame_sequence = seq;
    }
    pub fn frame_width(&self) -> usize {
        self.frame_width
    }
    pub fn frame_height(&self) -> usize {
        self.frame_height
    }
    pub fn num_frames(&self) -> (usize, usize) {
        self.num_frames
    }
    pub fn load_from_file(&mut self, texture_creator: &'a TextureCreator<WindowContext>,
                          path: &str, color_key: Option<Color>) -> Result<(), String> {
        if self.num_frames.0 == 0 || self.num_frames.1 == 0 {
            panic!("ERROR: Cannot load to an AnimatedSprite with {} horizontal frames and \
                    {} vertical frames", self.num_frames.0, self.num_frames.1);
        }
        self.sprite_sheet = None;
        let mut surface = sdl2::surface::Surface::from_file(path).unwrap();
        if let Some(key) = color_key {
            if let Err(msg) = surface.set_color_key(true, key) {
                return Err(msg);
            }
        }
        self.frame_width = surface.width() as usize / self.num_frames.0;
        self.frame_height = surface.height() as usize / self.num_frames.1;
        self.sprite_sheet = Some(texture_creator.create_texture_from_surface(surface).unwrap());
        Ok(())
    }
    pub fn render(&self, canvas: &mut WindowCanvas, x: i32, y: i32, size: Option<Size>, frame: (usize, usize)) -> Result<(), String> {
        let mut render_quad = Rect::new(x, y, self.frame_width as u32, self.frame_height as u32);
        if let Some(s) = size {
            render_quad.set_width(s.w);
            render_quad.set_height(s.h);
        }
        let frame_clip: Rect = match &self.custom_frame_sequence {
            Some(cfs) => {
                let f: &(usize, usize) = cfs.get(frame.0).unwrap();
                Rect::new((f.0 * self.frame_width) as i32, (f.1 * self.frame_height) as i32,
                          self.frame_width as u32, self.frame_height as u32)
            }
            None => Rect::new((frame.0 * self.frame_width) as i32, (frame.1 * self.frame_height) as i32,
                              self.frame_width as u32, self.frame_height as u32)
        };
        if let Some(ss) = &self.sprite_sheet {
            if let Err(msg) = canvas.copy(&ss, frame_clip, render_quad) {
                return Err(msg);
            }
        }
        Ok(())
    }
}
