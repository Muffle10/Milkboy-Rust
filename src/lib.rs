pub mod sprite {
    use macroquad::prelude::*;
    pub struct Sprite {
        pub rect: Rect,
    }
    pub fn new(x: f32,y: f32,w: f32,h: f32) -> Sprite {
        Sprite {
            rect: Rect::new(x,y,w,h)
        }
    }
}