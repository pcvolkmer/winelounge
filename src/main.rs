use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::player::Player;
use crate::world::World;

mod player;
mod sprite;
mod world;
mod net;

const GLASS_SPACE: u8 = 5;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Wine Lounge", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(44, 48, 63));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./assets/sprite.png").unwrap();

    let font = sdl2::ttf::init().unwrap();
    let font = font.load_font("./assets/Retro.ttf", 16).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut world = World::init();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                e => world.handle_event(e),
            }
        }

        if chrono::Utc::now().timestamp_millis() % 1000 > 950 {
            world.update_box_areas();
        }

        world.render(&mut canvas, &texture, &font);

        ::std::thread::sleep(Duration::from_millis(25));
    }
}
