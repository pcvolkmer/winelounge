use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

pub enum Sprite {
    BottleEmpty,
    BottleFilled,
    BushHorizontal,
    BushVertical,
    Flower,
    GlassEmpty,
    GlassFilled,
    HiddenBox,
    Lounge,
    Nothing,
    Player(PlayerDirection, PlayerFootstep),
    Stone,
}

pub enum PlayerFootstep {
    None = 5,
    Left = 60,
    Right = 115,
}

pub enum PlayerDirection {
    Down,
    Left,
    Right,
    Up,
}

impl Sprite {
    pub fn rect(&self) -> Rect {
        match &self {
            Sprite::BottleEmpty => Rect::new(35, 550, 20, 50),
            Sprite::BottleFilled => Rect::new(5, 550, 20, 50),
            Sprite::BushHorizontal => Rect::new(70, 510, 50, 25),
            Sprite::BushVertical => Rect::new(70, 550, 25, 50),
            Sprite::Flower => Rect::new(130, 550, 25, 25),
            Sprite::GlassEmpty => Rect::new(35, 510, 20, 25),
            Sprite::GlassFilled => Rect::new(5, 510, 20, 25),
            Sprite::HiddenBox => Rect::new(5, 620, 50, 50),
            Sprite::Lounge => Rect::new(5, 700, 150, 95),
            Sprite::Nothing => Rect::new(70, 620, 50, 50),
            Sprite::Stone => Rect::new(130, 510, 25, 25),
            Sprite::Player(pd, fs) => Sprite::player_rect(pd, fs),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.rect().size()
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture, x: i32, y: i32) {
        let render_rect = Rect::new(x, y, self.rect().width(), self.rect().height());
        let _r = canvas.copy(texture, self.rect(), render_rect);
    }

    fn player_rect(player_direction: &PlayerDirection, footstep: &PlayerFootstep) -> Rect {
        let x = match footstep {
            PlayerFootstep::Left => 60,
            PlayerFootstep::Right => 115,
            _ => 5,
        };

        match player_direction {
            PlayerDirection::Down => Rect::new(x, 5, 40, 115),
            PlayerDirection::Left => Rect::new(x, 255, 40, 115),
            PlayerDirection::Right => Rect::new(x, 130, 40, 115),
            PlayerDirection::Up => Rect::new(x, 380, 40, 115),
        }
    }
}
