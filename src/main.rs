mod player;
mod world;

use std::time::Duration;

use crate::player::player::Player;
use crate::world::world::{BoxAreaContent, BoxAreaPosition, World};
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Font;

const GLASS_SPACE: u8 = 5;

fn main() {
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
    let font = font.load_font("./assets/font.ttf", 20).unwrap();

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
                Event::KeyDown {
                    keycode: Some(Keycode::Up) | Some(Keycode::W),
                    ..
                } => {
                    world.move_up();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down) | Some(Keycode::S),
                    ..
                } => {
                    world.move_down();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left) | Some(Keycode::A),
                    ..
                } => {
                    world.move_left();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right) | Some(Keycode::D),
                    ..
                } => {
                    world.move_right();
                }
                Event::KeyUp { .. } => {
                    world.stop_player();
                }
                _ => {}
            }
        }

        if world.collides_with_lounge() && world.player.can_drink_glass() {
            world.player.drink_glass()
        }

        let colliding_box_area = match world.collides_with_box_area() {
            Some(BoxAreaPosition::RightTop) => Option::Some(&mut world.right_top_box_area),
            Some(BoxAreaPosition::RightBottom) => Option::Some(&mut world.right_bottom_box_area),
            Some(BoxAreaPosition::LeftBottom) => Option::Some(&mut world.left_bottom_box_area),
            Some(BoxAreaPosition::LeftTop) => Option::Some(&mut world.left_top_box_area),
            None => Option::None,
        };

        match colliding_box_area {
            Some(ba) => {
                let content = match &ba.content {
                    BoxAreaContent::HiddenBox => BoxAreaContent::random(),
                    BoxAreaContent::EmptyGlass => BoxAreaContent::EmptyGlass,
                    BoxAreaContent::FilledBottle => BoxAreaContent::FilledBottle,
                    _ => BoxAreaContent::Nothing,
                };

                if content == BoxAreaContent::EmptyGlass && world.player.can_pick_glass() {
                    ba.update_content(BoxAreaContent::Nothing);
                    world.player.pick_glass();
                } else if content == BoxAreaContent::EmptyGlass && !world.player.can_pick_glass() {
                    ba.update_content(BoxAreaContent::EmptyGlass);
                } else if content == BoxAreaContent::FilledBottle && world.player.can_fill_glass() {
                    ba.update_content(BoxAreaContent::EmptyBottle);
                    world.player.fill_glass();
                } else if content == BoxAreaContent::FilledBottle && !world.player.can_fill_glass()
                {
                    ba.update_content(BoxAreaContent::FilledBottle);
                }
            }
            None => {}
        }

        if chrono::Utc::now().timestamp_millis() % 1000 > 950 {
            world.update_box_areas();
        }
        world.render(&mut canvas, &texture, &font);

        ::std::thread::sleep(Duration::from_millis(25));
    }
}
