use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Font;

use crate::sprite::Sprite;
use crate::{Player, GLASS_SPACE};

pub struct World {
    player: Player,
    remote_player: Option<Player>,
    right_top_box_area: BoxArea,
    right_bottom_box_area: BoxArea,
    left_bottom_box_area: BoxArea,
    left_top_box_area: BoxArea,
    stops: Vec<Point>,
}

/// The world, the player and any item exists within
impl World {
    /// Creates and initializes new playable world.
    pub fn init() -> World {
        World {
            player: Player::init(),
            remote_player: None,
            right_top_box_area: BoxArea::new(BoxAreaPosition::RightTop, BoxAreaContent::EmptyGlass),
            right_bottom_box_area: BoxArea::new(
                BoxAreaPosition::RightBottom,
                BoxAreaContent::HiddenBox,
            ),
            left_bottom_box_area: BoxArea::new(
                BoxAreaPosition::LeftBottom,
                BoxAreaContent::Nothing,
            ),
            left_top_box_area: BoxArea::new(BoxAreaPosition::LeftTop, BoxAreaContent::Nothing),
            stops: vec![
                Point::new(380, 60),
                Point::new(590, 450),
                Point::new(720, 300),
                Point::new(20, 410),
                Point::new(190, 560),
            ],
        }
    }

    pub fn get_player(&mut self, _: String) -> &mut Player {
        &mut self.player
    }

    pub fn playable_rect() -> Rect {
        Rect::new(0, 50, 800, 550)
    }

    /// Handles key events for player move.
    ///
    /// This checks if player collides with any stop item or will move out of world.
    /// If player can move, move him and turn him to the correct side.
    pub fn handle_event(&mut self, event: Event) {
        let player_id = self.player.id.clone();
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Up) | Some(Keycode::W),
                ..
            } => {
                self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Up));
                if self.collides_with_stop() || !self.player.within_rect(&Self::playable_rect()) {
                    self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Down));
                    self.execute_command(Command::FacePlayer(player_id.clone(), Direction::Up));
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down) | Some(Keycode::S),
                ..
            } => {
                self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Down));
                if self.collides_with_stop() || !self.player.within_rect(&Self::playable_rect()) {
                    self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Up));
                    self.execute_command(Command::FacePlayer(player_id.clone(), Direction::Down));
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left) | Some(Keycode::A),
                ..
            } => {
                self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Left));
                if self.collides_with_stop() || !self.player.within_rect(&Self::playable_rect()) {
                    self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Right));
                    self.execute_command(Command::FacePlayer(player_id.clone(), Direction::Left));
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right) | Some(Keycode::D),
                ..
            } => {
                self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Right));
                if self.collides_with_stop() || !self.player.within_rect(&Self::playable_rect()) {
                    self.execute_command(Command::MovePlayer(player_id.clone(), Direction::Left));
                    self.execute_command(Command::FacePlayer(player_id.clone(), Direction::Right));
                }
            }
            Event::KeyUp { .. } => self.execute_command(Command::StopPlayer(player_id.clone())),
            _ => {}
        }
    }

    /// Executes a command for world update.
    pub fn execute_command(&mut self, command: Command) {
        debug!("{}", command);

        match command {
            Command::SpawnPlayer(player_id, x, y) => &mut {
                // TBD
            },
            Command::RemovePlayer(player_id) => &mut {
                // TBD
            },
            Command::FacePlayer(player_id, Direction::Down) => {
                &mut self.get_player(player_id).face_down()
            },
            Command::FacePlayer(player_id, Direction::Up) => {
                &mut self.get_player(player_id).face_up()
            },
            Command::FacePlayer(player_id, Direction::Left) => {
                &mut self.get_player(player_id).face_left()
            },
            Command::FacePlayer(player_id, Direction::Right) => {
                &mut self.get_player(player_id).face_right()
            },
            Command::MovePlayer(player_id, Direction::Down) => {
                &mut self.get_player(player_id).move_down()
            },
            Command::MovePlayer(player_id, Direction::Up) => {
                &mut self.get_player(player_id).move_up()
            },
            Command::MovePlayer(player_id, Direction::Left) => {
                &mut self.get_player(player_id).move_left()
            },
            Command::MovePlayer(player_id, Direction::Right) => {
                &mut self.get_player(player_id).move_right()
            },
            Command::StopPlayer(player_id) => {
                &mut self.get_player(player_id).stop()
            },
            Command::UpdateBoxArea(position, content) => &mut {
                match position {
                    BoxAreaPosition::RightTop => {
                        self.right_top_box_area.update_content(content);
                        self.right_top_box_area.last_update = chrono::Utc::now().timestamp();
                    }
                    BoxAreaPosition::RightBottom => {
                        self.right_bottom_box_area.update_content(content);
                        self.right_top_box_area.last_update = chrono::Utc::now().timestamp();
                    }
                    BoxAreaPosition::LeftBottom => {
                        self.left_bottom_box_area.update_content(content);
                        self.right_top_box_area.last_update = chrono::Utc::now().timestamp();
                    }
                    BoxAreaPosition::LeftTop => {
                        self.left_top_box_area.update_content(content);
                        self.right_top_box_area.last_update = chrono::Utc::now().timestamp();
                    }
                };
            }
        };
    }

    /// Updates box areas to provide new boxes and remove items after some time
    pub fn update_box_areas(&mut self) {
        self.update_box_area(BoxAreaPosition::RightTop);
        self.update_box_area(BoxAreaPosition::RightBottom);
        self.update_box_area(BoxAreaPosition::LeftBottom);
        self.update_box_area(BoxAreaPosition::LeftTop);
    }

    /// Handles both, collisions with lounge and any box area
    pub fn handle_collisions(&mut self) {
        self.handle_lounge_collisions();
        self.handle_boxarea_collisions();
    }

    /// Renders world using given canvas, texture and font
    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture, font: &Font) {
        canvas.clear();

        canvas.set_draw_color(Color::RGB(160, 90, 44));
        let _r = canvas.fill_rect(Rect::new(0, 0, 800, 45));

        canvas.set_draw_color(Color::RGB(206, 182, 115));

        // Points/Glasses
        (1..=GLASS_SPACE).for_each(|i| {
            canvas.set_draw_color(Color::RGB(128, 51, 0));
            let _r = canvas.fill_rect(Rect::new(5, 37, GLASS_SPACE as u32 * 25 + 5, 4));

            if self.player.filled_glasses + self.player.empty_glasses >= i {
                Sprite::GlassEmpty.render(canvas, texture, (i as i32) * 25 - 15, 10);
            }
            if self.player.filled_glasses >= i {
                Sprite::GlassFilled.render(canvas, texture, (i as i32) * 25 - 15, 10);
            }
        });

        // Lounge
        Sprite::Lounge.render(canvas, texture, 325, 260);

        // Box Areas
        self.right_top_box_area.render(canvas, texture);
        self.right_bottom_box_area.render(canvas, texture);
        self.left_bottom_box_area.render(canvas, texture);
        self.left_top_box_area.render(canvas, texture);

        // Decoration
        Sprite::Flower.render(canvas, texture, 235, 130);
        Sprite::Flower.render(canvas, texture, 120, 210);
        Sprite::Flower.render(canvas, texture, 535, 150);
        Sprite::Flower.render(canvas, texture, 435, 370);
        Sprite::Flower.render(canvas, texture, 235, 470);
        Sprite::Flower.render(canvas, texture, 555, 510);

        // Stops
        for s in &self.stops {
            Sprite::Stone.render(canvas, texture, s.x(), s.y())
        }

        // Player
        self.player.render(canvas, texture);

        // Points
        let x = font
            .render(format!("Score: {:#04}", self.player.points).as_str())
            .blended(Color::RGBA(246, 222, 155, 255))
            .unwrap();
        let t2 = canvas.texture_creator();
        let t2 = t2.create_texture_from_surface(&x).unwrap();

        let _r = canvas.copy(
            &t2,
            x.rect(),
            Some(Rect::new(790 - x.width() as i32, 16, x.width(), x.height())),
        );
        canvas.set_draw_color(Color::RGB(206, 182, 115));

        canvas.present();
    }

    fn update_box_area(&mut self, box_area_position: BoxAreaPosition) {
        let box_area = match box_area_position {
            BoxAreaPosition::RightTop => &self.right_top_box_area,
            BoxAreaPosition::RightBottom => &self.right_bottom_box_area,
            BoxAreaPosition::LeftBottom => &self.left_bottom_box_area,
            BoxAreaPosition::LeftTop => &self.left_top_box_area,
        };

        let now = chrono::Utc::now().timestamp();
        let r: i64 = (rand::random::<i64>() % 10) + 3;

        if box_area.content == BoxAreaContent::Nothing && box_area.last_update + 10 < now {
            self.execute_command(Command::UpdateBoxArea(
                box_area_position,
                BoxAreaContent::HiddenBox,
            ));
        } else if box_area.content != BoxAreaContent::Nothing && box_area.last_update + 30 < now - r
        {
            self.execute_command(Command::UpdateBoxArea(
                box_area_position,
                BoxAreaContent::Nothing,
            ));
        }
    }

    fn has_player_collision(&mut self) -> Collision {
        if let Some(ba) = self.collides_with_box_area() {
            return Collision::BoxArea(ba);
        } else if self.collides_with_lounge() {
            return Collision::Lounge;
        } else if self.collides_with_stop() {
            return Collision::Stopper;
        }

        Collision::None
    }

    fn collides_with_box_area(&mut self) -> Option<BoxAreaPosition> {
        if self.right_top_box_area.collides_with(&self.player) {
            return Some(BoxAreaPosition::RightTop);
        } else if self.right_bottom_box_area.collides_with(&self.player) {
            return Some(BoxAreaPosition::RightBottom);
        } else if self.left_bottom_box_area.collides_with(&self.player) {
            return Some(BoxAreaPosition::LeftBottom);
        } else if self.left_top_box_area.collides_with(&self.player) {
            return Some(BoxAreaPosition::LeftTop);
        }

        None
    }

    fn collides_with_lounge(&mut self) -> bool {
        let lounge_rect = Rect::new(325, 260, 150, 95);
        lounge_rect.contains_point(self.player.center())
    }

    fn collides_with_stop(&mut self) -> bool {
        for s in &self.stops {
            let x = s.x() + 12;
            let y = s.y() + 12;
            if self.player.bounding_rect().contains_point(Point::new(x, y)) {
                return true;
            }
        }
        false
    }

    fn handle_lounge_collisions(&mut self) {
        if Collision::Lounge == self.has_player_collision() && self.player.can_drink_glass() {
            self.player.drink_glass()
        }
    }

    // TODO Commands
    fn handle_boxarea_collisions(&mut self) {
        if let Collision::BoxArea(bap) = self.has_player_collision() {
            let ba = match bap {
                BoxAreaPosition::RightTop => &mut self.right_top_box_area,
                BoxAreaPosition::RightBottom => &mut self.right_bottom_box_area,
                BoxAreaPosition::LeftBottom => &mut self.left_bottom_box_area,
                BoxAreaPosition::LeftTop => &mut self.left_top_box_area,
            };

            let content = match &ba.content {
                BoxAreaContent::HiddenBox => BoxAreaContent::random(),
                BoxAreaContent::EmptyGlass => BoxAreaContent::EmptyGlass,
                BoxAreaContent::FilledBottle => BoxAreaContent::FilledBottle,
                _ => BoxAreaContent::Nothing,
            };

            if content == BoxAreaContent::EmptyGlass && self.player.can_pick_glass() {
                self.execute_command(Command::UpdateBoxArea(bap, BoxAreaContent::Nothing));
                self.player.pick_glass();
            } else if content == BoxAreaContent::EmptyGlass && !self.player.can_pick_glass() {
                self.execute_command(Command::UpdateBoxArea(bap, BoxAreaContent::EmptyGlass));
            } else if content == BoxAreaContent::FilledBottle && self.player.can_fill_glass() {
                self.execute_command(Command::UpdateBoxArea(bap, BoxAreaContent::EmptyBottle));
                self.player.fill_glass();
            } else if content == BoxAreaContent::FilledBottle && !self.player.can_fill_glass() {
                self.execute_command(Command::UpdateBoxArea(bap, BoxAreaContent::FilledBottle));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    SpawnPlayer(String, u32, u32),
    RemovePlayer(String),
    FacePlayer(String, Direction),
    MovePlayer(String, Direction),
    StopPlayer(String),
    UpdateBoxArea(BoxAreaPosition, BoxAreaContent),
}

#[derive(Debug, PartialEq, Eq)]
enum Collision {
    BoxArea(BoxAreaPosition),
    Lounge,
    Stopper,
    None,
}

#[derive(Debug)]
struct BoxArea {
    position: BoxAreaPosition,
    pub content: BoxAreaContent,
    last_update: i64,
}

impl BoxArea {
    /// Creates a new BoxArea
    fn new(position: BoxAreaPosition, content: BoxAreaContent) -> BoxArea {
        BoxArea {
            position,
            content,
            last_update: chrono::Utc::now().timestamp(),
        }
    }

    fn update_content(&mut self, content: BoxAreaContent) {
        self.content = content;
        self.last_update = chrono::Utc::now().timestamp();
    }

    fn bounding_rect(&self) -> Rect {
        let x_offset = match self.position {
            BoxAreaPosition::RightTop => 685,
            BoxAreaPosition::RightBottom => 685,
            BoxAreaPosition::LeftBottom => 5,
            BoxAreaPosition::LeftTop => 5,
        };
        let y_offset = match self.position {
            BoxAreaPosition::RightTop => 50,
            BoxAreaPosition::RightBottom => 480,
            BoxAreaPosition::LeftBottom => 480,
            BoxAreaPosition::LeftTop => 50,
        };

        Rect::new(x_offset, y_offset, 110, 110)
    }

    /// Checks if player collides with this BoxArea
    fn collides_with(&self, player: &Player) -> bool {
        self.bounding_rect().contains_point(player.center())
    }

    /// Renders BoxArea using goven Canvas and Texture
    fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        let x_offset = self.bounding_rect().x();
        let y_offset = self.bounding_rect().y();

        // Border
        Sprite::BushHorizontal.render(canvas, texture, x_offset + 30, y_offset);
        Sprite::BushHorizontal.render(canvas, texture, x_offset + 30, y_offset + 85);
        let (dst_x, dst_y) = match self.position {
            BoxAreaPosition::RightTop => (x_offset + 85, y_offset + 30),
            BoxAreaPosition::RightBottom => (x_offset + 85, y_offset + 30),
            BoxAreaPosition::LeftBottom => (x_offset, y_offset + 30),
            BoxAreaPosition::LeftTop => (x_offset, y_offset + 30),
        };
        Sprite::BushVertical.render(canvas, texture, dst_x, dst_y);

        // Box
        let box_sprite = match &self.content {
            BoxAreaContent::HiddenBox => Sprite::HiddenBox,
            BoxAreaContent::FilledBottle => Sprite::BottleFilled,
            BoxAreaContent::EmptyBottle => Sprite::BottleEmpty,
            BoxAreaContent::EmptyGlass => Sprite::GlassEmpty,
            BoxAreaContent::Nothing => Sprite::Nothing,
        };
        let (box_width, box_height) = box_sprite.size();
        box_sprite.render(
            canvas,
            texture,
            x_offset + 30 + (50 - box_width as i32) / 2,
            y_offset + 30 + (50 - box_height as i32) / 2,
        );
    }
}

/// Position of a BoxArea.
/// There are only four possible values for each vertex of the world.
#[derive(Debug, PartialEq, Eq)]
pub enum BoxAreaPosition {
    RightTop,
    RightBottom,
    LeftBottom,
    LeftTop,
}

/// Content of a BoxArea
#[derive(Debug, PartialEq, Eq)]
pub enum BoxAreaContent {
    Nothing,
    HiddenBox,
    EmptyGlass,
    FilledBottle,
    EmptyBottle,
}

impl BoxAreaContent {
    /// Selects new random BoxAreaContent
    fn random() -> BoxAreaContent {
        match rand::random::<i32>() % 5 {
            1 | 4 => BoxAreaContent::EmptyGlass,
            2 | 3 => BoxAreaContent::FilledBottle,
            _ => BoxAreaContent::Nothing,
        }
    }
}
