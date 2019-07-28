use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::image::LoadSurface;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use crate::common::Size;

pub struct Sprite<'a> {
    texture: Option<sdl2::render::Texture<'a>>,
    width: u32,
    height: u32,
}

impl<'a> Default for Sprite<'a> {
    fn default() -> Sprite<'a> {
        Sprite {
            texture: None,
            width: 0,
            height: 0,
        }
    }
}

impl<'a> Sprite<'a> {
    pub fn new() -> Sprite<'a> {
        Sprite {
            texture: None,
            width: 0,
            height: 0,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn load_from_file(&mut self, texture_creator: &'a TextureCreator<WindowContext>,
                          path: &str, color_key: Option<Color>) -> Result<(), String> {
        self.texture = None;
        let surface_result = Surface::from_file(path);
        match surface_result {
            Ok(_) => {
                let mut surface: Surface = surface_result.unwrap();
                if let Some(key) = color_key {
                    if let Err(msg) = surface.set_color_key(true, key) {
                        return Err(msg);
                    }
                }
                self.width = surface.width();
                self.height = surface.height();
                self.texture = Some(texture_creator.create_texture_from_surface(surface).unwrap());
                Ok(())
            }
            Err(msg) => Err(msg)
        }
    }
    pub fn set_color_mod(&mut self, color: Color) {
        if let Some(t) = &mut self.texture {
            t.set_color_mod(color.r, color.g, color.b);
        }
    }
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        if let Some(t) = &mut self.texture {
            t.set_blend_mode(mode);
        }
    }
    pub fn set_alpha(&mut self, alpha: u8) {
        if let Some(t) = &mut self.texture {
            t.set_alpha_mod(alpha);
        }
    }
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, size: Option<Size>, clip: Option<Rect>) -> Result<(), String> {
        let mut render_quad: Rect = Rect::new(x, y, self.width, self.height);
        if let Some(c) = clip {
            render_quad.set_width(c.width());
            render_quad.set_height(c.height());
        }
        if let Some(s) = size {
            render_quad.set_width(s.w);
            render_quad.set_height(s.h);
        }
        match &self.texture {
            Some(t) => canvas.copy(t, clip, render_quad),
            None => Ok(())
        }
    }
    pub fn render_to_texture<'b>(&'b self, canvas: &mut WindowCanvas, texture_creator: &'b TextureCreator<WindowContext>,
                                 size: Option<Rect>, clip: Option<Rect>)
                                 -> Result<Texture, String> {
        let mut render_quad: Rect = Rect::new(0, 0, self.width, self.height);
        if let Some(c) = clip {
            render_quad.set_width(c.width());
            render_quad.set_height(c.height());
        }
        let (texture_w, texture_h): (u32, u32);
        match size {
            Some(s) => {
                texture_w = s.width();
                texture_h = s.height();
            }
            None => {
                texture_w = render_quad.width();
                texture_h = render_quad.height();
            }
        }
        let mut texture: Texture = texture_creator.create_texture_target(None, texture_w, texture_h).unwrap();
        let result = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.clear();
            let copy_result = match &self.texture {
                Some(t) => texture_canvas.copy(t, clip, Rect::new(0, 0, texture_w, texture_h)),
                None => Ok(())
            };
            match copy_result {
                Ok(..) => (),
                Err(msg) => {
                    println!("ERROR: Could not copy texture: {}", msg);
                }
            }
        });
        match result {
            Ok(..) => Ok(texture),
            Err(msg) => {
                Err(format!("ERROR: Could not draw to new texture: {}", msg))
            }
        }
    }
}
