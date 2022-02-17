use crate::sprite::Sprite;
use crate::{Player, GLASS_SPACE};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Font;

pub struct World {
    pub player: Player,
    pub right_top_box_area: BoxArea,
    pub right_bottom_box_area: BoxArea,
    pub left_bottom_box_area: BoxArea,
    pub left_top_box_area: BoxArea,
    stops: Vec<Point>,
}

impl World {
    pub fn init() -> World {
        World {
            player: Player::init(),
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

    pub fn collides_with_box_area(&mut self) -> Option<BoxAreaPosition> {
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

    pub fn collides_with_lounge(&mut self) -> bool {
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

    pub fn move_up(&mut self) {
        if self.player.position.y > 50 {
            self.player.move_up();
            if self.collides_with_stop() {
                self.player.move_down();
                self.player.face_up();
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.player.position.y < 600 - 110 {
            self.player.move_down();
            if self.collides_with_stop() {
                self.player.move_up();
                self.player.face_down();
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.player.position.x > 0 {
            self.player.move_left();
            if self.collides_with_stop() {
                self.player.move_right();
                self.player.face_left();
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.player.position.x < 800 - 40 {
            self.player.move_right();
            if self.collides_with_stop() {
                self.player.move_left();
                self.player.face_right();
            }
        }
    }

    pub fn stop_player(&mut self) {
        self.player.stop()
    }

    pub fn update_box_areas(&mut self) {
        World::update_box_area(&mut self.right_top_box_area);
        World::update_box_area(&mut self.right_bottom_box_area);
        World::update_box_area(&mut self.left_bottom_box_area);
        World::update_box_area(&mut self.left_top_box_area);
    }

    fn update_box_area(box_area: &mut BoxArea) {
        let now = chrono::Utc::now().timestamp();
        let r: i64 = (rand::random::<i64>() % 10) + 3;

        if box_area.content == BoxAreaContent::Nothing && box_area.last_update + 10 < now {
            box_area.content = BoxAreaContent::HiddenBox;
            box_area.last_update = now;
        } else if box_area.content != BoxAreaContent::Nothing && box_area.last_update + 30 < now - r
        {
            box_area.content = BoxAreaContent::Nothing;
            box_area.last_update = now;
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture: &Texture, font: &Font) {
        canvas.clear();

        canvas.set_draw_color(Color::RGB(160, 90, 44));
        canvas.fill_rect(Rect::new(0, 0, 800, 45));

        canvas.set_draw_color(Color::RGB(206, 182, 115));

        // Points/Glasses
        (1..=GLASS_SPACE).for_each(|i| {
            canvas.set_draw_color(Color::RGB(128, 51, 0));
            canvas.fill_rect(Rect::new(5, 37, GLASS_SPACE as u32 * 25 + 5, 4));

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

        canvas.copy(
            &t2,
            x.rect(),
            Some(Rect::new(790 - x.width() as i32, 8, x.width(), x.height())),
        );
        canvas.set_draw_color(Color::RGB(206, 182, 115));

        canvas.present();
    }
}

#[derive(Debug)]
pub struct BoxArea {
    position: BoxAreaPosition,
    pub content: BoxAreaContent,
    last_update: i64,
}

impl BoxArea {
    fn new(position: BoxAreaPosition, content: BoxAreaContent) -> BoxArea {
        BoxArea {
            position,
            content,
            last_update: chrono::Utc::now().timestamp(),
        }
    }

    pub fn update_content(&mut self, content: BoxAreaContent) {
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

    fn enter_rect(&self) -> Rect {
        match self.position {
            BoxAreaPosition::RightTop => {
                Rect::new(self.bounding_rect().x(), self.bounding_rect().y(), 25, 110)
            }
            BoxAreaPosition::RightBottom => {
                Rect::new(self.bounding_rect().x(), self.bounding_rect().y(), 25, 110)
            }
            BoxAreaPosition::LeftBottom => Rect::new(
                self.bounding_rect().x() + 85,
                self.bounding_rect().y(),
                25,
                110,
            ),
            BoxAreaPosition::LeftTop => Rect::new(
                self.bounding_rect().x() + 85,
                self.bounding_rect().y(),
                25,
                110,
            ),
        }
    }

    fn collides_with(&self, player: &Player) -> bool {
        self.bounding_rect().contains_point(player.center())
    }

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

#[derive(Debug)]
pub enum BoxAreaPosition {
    RightTop,
    RightBottom,
    LeftBottom,
    LeftTop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoxAreaContent {
    Nothing,
    HiddenBox,
    EmptyGlass,
    FilledBottle,
    EmptyBottle,
}

impl BoxAreaContent {
    pub fn random() -> BoxAreaContent {
        match rand::random::<i32>() % 5 {
            1 | 4 => BoxAreaContent::EmptyGlass,
            2 | 3 => BoxAreaContent::FilledBottle,
            _ => BoxAreaContent::Nothing,
        }
    }
}
