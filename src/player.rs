use crate::sprite::Sprite;
use crate::{sprite, GLASS_SPACE};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

pub struct Player {
    pub position: Point,
    direction: PlayerDirection,
    footstep: u8,
    pub empty_glasses: u8,
    pub filled_glasses: u8,
    pub points: u32,
}

enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Player {
    pub fn init() -> Player {
        Player {
            position: Point::new(380, 250),
            direction: PlayerDirection::Down,
            footstep: 0,
            empty_glasses: 0,
            filled_glasses: 0,
            points: 0,
        }
    }

    pub fn can_pick_glass(&self) -> bool {
        self.empty_glasses + self.filled_glasses < GLASS_SPACE
    }

    pub fn pick_glass(&mut self) {
        self.empty_glasses += 1;
        self.points += 2
    }

    pub fn can_fill_glass(&self) -> bool {
        self.empty_glasses > 0
    }

    pub fn fill_glass(&mut self) {
        self.empty_glasses -= 1;
        self.filled_glasses += 1;
        self.points += 3
    }

    pub fn can_drink_glass(&self) -> bool {
        self.filled_glasses > 0
    }

    pub fn drink_glass(&mut self) {
        self.filled_glasses -= 1;
        self.points += 5
    }

    pub fn center(&self) -> Point {
        Point::new(self.position.x() + 19, self.position.y() + 56)
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect::new(self.position.x(), self.position.y(), 40, 115)
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture) {
        let direction = match self.direction {
            PlayerDirection::Down => sprite::PlayerDirection::Down,
            PlayerDirection::Left => sprite::PlayerDirection::Left,
            PlayerDirection::Right => sprite::PlayerDirection::Right,
            PlayerDirection::Up => sprite::PlayerDirection::Up,
        };

        let footstep = match self.footstep {
            1 => sprite::PlayerFootstep::Left,
            2 => sprite::PlayerFootstep::Right,
            _ => sprite::PlayerFootstep::None,
        };

        Sprite::Player(direction, footstep).render(
            canvas,
            texture,
            self.position.x(),
            self.position.y(),
        );
    }

    pub fn face_up(&mut self) {
        self.direction = PlayerDirection::Up;
    }

    pub fn face_down(&mut self) {
        self.direction = PlayerDirection::Down;
    }

    pub fn face_left(&mut self) {
        self.direction = PlayerDirection::Left;
    }

    pub fn face_right(&mut self) {
        self.direction = PlayerDirection::Right;
    }

    pub fn move_up(&mut self) {
        self.face_up();
        self.footstep = &self.footstep % 2 + 1;
        self.position.y -= 15
    }

    pub fn move_down(&mut self) {
        self.face_down();
        self.footstep = &self.footstep % 2 + 1;
        self.position.y += 15
    }

    pub fn move_left(&mut self) {
        self.face_left();
        self.footstep = &self.footstep % 2 + 1;
        self.position.x -= 15
    }

    pub fn move_right(&mut self) {
        self.face_right();
        self.footstep = &self.footstep % 2 + 1;
        self.position.x += 15
    }

    pub fn stop(&mut self) {
        self.footstep = 0;
    }
}
