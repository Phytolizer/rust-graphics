#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate strum_macros;
extern crate enum_map;
extern crate sdl2;
extern crate strum;

mod animated_sprite;
mod common;
mod sprite;
mod tile;
mod viewport;
mod world;

use animated_sprite::AnimatedSprite;
use enum_map::EnumMap;
use sprite::Sprite;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::image::InitFlag;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;
use crate::common::{Size, Position};
use crate::tile::{TileId, TILE_SIZE};
use crate::world::World;
use crate::viewport::Viewport;

fn main() {
    // init graphics stuff
    let sdl_context: Sdl = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG);
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    // create window
    let window: Window = video_subsystem.window("Game", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let fps = window.display_mode().unwrap().refresh_rate;
    // used for direct screen drawing or drawing to a texture
    let mut canvas = window.into_canvas().target_texture().build().unwrap();
    // send this anywhere textures are needed, textures cannot outlive it!
    let texture_creator = canvas.texture_creator();
    // all tile rendering will use this collection as a reference
    // prevent excessive file I/O and texture copying
    let tile_atlases: EnumMap<tile::TileId, Sprite> = tile::load_tile_atlases(&texture_creator).unwrap();
    // provides events
    let mut event_pump = sdl_context.event_pump().unwrap();
    // window size in pixels
    let mut size: (u32, u32) = (canvas.window().size().0, canvas.window().size().1);
    // fps is the maximum frame rate; each frame will be compared to this value and will hang the
    // render thread if necessary to avoid going over this frame rate
    let wait_time = std::time::Duration::from_nanos((1000000000i32 / fps) as u64);
    // statistics
    let mut total_frame_time: u128 = 0;
    let mut num_frames: u128 = 0;
    // define things here
    let w = (size.0 / TILE_SIZE) as usize;
    let h = (size.1 / TILE_SIZE) as usize;
    dbg!(w);
    let mut my_world = World::new(w, h);
    for i in 0usize..w {
        for j in 0usize..h {
            if j == 3 {
                my_world.get_tile_mut(i, j).unwrap().set_id(TileId::Grass);
            } else if j > 3 {
                my_world.get_tile_mut(i, j).unwrap().set_id(TileId::Dirt);
            }
        }
    }
    my_world.update_cached_neighbors();
    // used for timing and average frame rate calculations
    let program_start = std::time::Instant::now();
    'running: loop {
        let frame_start = std::time::Instant::now();
        // event handling yay!
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::Window { timestamp: _, window_id: _, win_event: e } => {
                    if let WindowEvent::Resized(w, h) = e {
                        size = (w as u32, h as u32);
                    }
                }
                _ => {}
            }
        }
        // background color
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        // Rendering code (each frame)
        for i in 0usize..w {
            for j in 0usize..h {
                my_world.get_tile(i, j).unwrap().render(&tile_atlases, &mut canvas, Rect::new(i as i32 * TILE_SIZE as i32, j as i32 * TILE_SIZE as i32, TILE_SIZE, TILE_SIZE));
            }
        }
        // Finished
        canvas.present();
        // Frame rate stabilization, never go above `fps`
        let frame_time = std::time::Instant::now() - frame_start;
        total_frame_time += frame_time.as_nanos();
        num_frames += 1;
        if frame_time < wait_time {
            std::thread::sleep(wait_time - frame_time);
        } else {
            println!("frame took too long; presenting next ASAP\n\
            ({:?} > {:?})", frame_time, wait_time);
        }
    }
    println!("Average frame time: {} ms (ideal upper bound: {} ms)", (total_frame_time as f64) / (num_frames as f64) / 1000000f64, 1000f64 / fps as f64);
    println!("Average frame rate: {:?} (ideal {})", (num_frames as f64) / (std::time::Instant::now() - program_start).as_nanos() as f64 * 1000000000f64, fps);
}
