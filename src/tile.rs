use sdl2::video::WindowContext;
use sdl2::render::{TextureCreator, Canvas, WindowCanvas};
use enum_map::EnumMap;
use crate::sprite::Sprite;
use strum::IntoEnumIterator;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use enum_map::Enum;
use crate::common::Size;
use sdl2::event::EventType::TextInput;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Display)]
#[derive(Debug)]
#[derive(Enum)]
#[derive(EnumIter)]
#[derive(PartialEq)]
pub enum TileId {
    Nothing,
    Dirt,
    Stone,
    Grass,
}

pub const TILE_SIZE: u32 = 8;
lazy_static! {
    pub static ref TILE_QUAD: Rect = Rect::new(0, 0, TILE_SIZE as u32, TILE_SIZE as u32);
}
#[derive(Debug)]
pub struct Tile {
    id: TileId,
    neighbors: [Option<TileId>; 4],
    frame: u32,
    solid: bool,
    neighbor_bf: u32,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            id: TileId::Nothing,
            neighbors: [None; 4],
            frame: 0,
            solid: false,
            neighbor_bf: 0,
        }
    }
    pub fn set_id(&mut self, id: TileId) {
        self.id = id;
    }
    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }
    pub fn set_solid(&mut self, solid: bool) {
        self.solid = solid;
    }
    pub fn set_neighbors(&mut self, neighbors: [Option<TileId>; 4]) {
        self.neighbor_bf = 0;
        for i in 0u8..4 {
            let neighbor_counts: bool;
            if let Some(n) = neighbors[usize::from(i)] {
                neighbor_counts = n != TileId::Nothing;
            } else {
                neighbor_counts = true;
            }
            if neighbor_counts {
                self.neighbor_bf += 2u32.pow(u32::from(i));
            }
        }
        self.neighbors = neighbors;
    }
    pub fn get_id(&self) -> TileId {
        self.id
    }
    pub fn get_frame(&self) -> u32 {
        self.frame
    }
    pub fn get_neighbor_bf(&self) -> u32 {
        self.neighbor_bf
    }
    pub fn render(&self, atlases: &EnumMap<TileId, Sprite>, canvas: &mut WindowCanvas, dest: Rect) -> Result<(), String> {
        let tile_sprite = &atlases[self.id];
        let clip = Rect::new((TILE_SIZE * self.frame) as i32, (TILE_SIZE * self.neighbor_bf) as i32, TILE_SIZE, TILE_SIZE);
        tile_sprite.render(canvas, dest.x(), dest.y(), Some(Size{w: dest.width(), h: dest.height()}), Some(clip))
    }
}

pub fn load_tile_atlases<'tc>(texture_creator: &'tc TextureCreator<WindowContext>)
                              -> Result<EnumMap<TileId, Sprite<'tc>>, String> {
    let magenta = Color { r: 255, g: 0, b: 255, a: 255 };
    let mut tile_atlases = EnumMap::<TileId, Sprite<'tc>>::new();
    for id in TileId::iter() {
        if id == TileId::Nothing {
            continue;
        }
        tile_atlases[id] = Sprite::new();
        if let Err(msg) = tile_atlases[id].load_from_file(&texture_creator,
                                                          &format!("sprites/Tile_{}.png", id.to_string()),
                                                          Some(magenta),
        ) {
            println!("{}", msg);
        }
    }
    Ok(tile_atlases)
}
