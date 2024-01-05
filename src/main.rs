use macroquad::prelude::*;
use macroquad::time::*;
enum EntityType {
    Enemy,
    Player,
}
struct AnimatedSprite {
    source_rect: Rect,
    source_index: i16,
    entity_rect: Rect,
    texture: Texture2D,
    entity_type: EntityType,
}
impl AnimatedSprite {
    fn new(entity_rect: Rect, texture: Texture2D, entity_type:EntityType) -> Self{
        Self {
            source_rect: Rect::new(0f32, 0f32, 16f32, 16f32),
            source_index: 0,
            entity_rect,
            texture,
            entity_type,
        }

    }
    fn draw(&self) {
        draw_texture_ex(&self.texture, self.entity_rect.x,self.entity_rect.y, WHITE, DrawTextureParams{dest_size: Some(Vec2::new(self.entity_rect.w, self.entity_rect.h)),source: Some(self.source_rect), ..Default::default()});
    }
    fn animate(&mut self, anim_speed: f32, frame:i16){
        match self.entity_type{
        EntityType::Player => { if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right)) && frame as f32 % anim_speed == 0f32 /*&& get_time() as f32 % anim_speed as f32 >= anim_speed as f32 * ( 0.9f32 - anim_speed as f32 * 0.02*/{
            self.source_index += 1;
            if self.source_index > 3{
                self.source_index = 0;
            }
        }
        if is_key_released(KeyCode::Right) || is_key_released(KeyCode::Left){
            self.source_index = 0;
        }
    },
    EntityType::Enemy => {
        if frame as f32 % anim_speed == 0f32{
            self.source_index += 1;
            if self.source_index > 4{
                self.source_index = 0;
            }
        }
    }
        }
        self.source_rect.x = 16f32 * self.source_index as f32;
    }
    fn update(&mut self){
        match self.entity_type {
            EntityType::Player => {
                if is_key_down(KeyCode::Right){
                    self.entity_rect.x += 300f32 * get_frame_time() as f32;
                }
                if is_key_down(KeyCode::Left){
                    self.entity_rect.x -= 300f32 * get_frame_time() as f32;
                }
                if is_key_down(KeyCode::Down){
                    self.entity_rect.y += 300f32 * get_frame_time() as f32;
                }
                if is_key_down(KeyCode::Up){
                    self.entity_rect.y -= 300f32 * get_frame_time() as f32;
                }
            }
            EntityType::Enemy => {
                self.entity_rect.x += 1
            }
        }
    }
}
#[macroquad::main("Milkboy: Rust")]
async fn main() {
    let mut frame: i16 = 0;
    let milkboy_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/milkboy.png").await.unwrap();
    milkboy_texture.set_filter(FilterMode::Nearest);
    let mut player_sprite = AnimatedSprite::new(Rect::new(10f32, 10f32, 100f32, 100f32), milkboy_texture, EntityType::Player);
    let enemy_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/evil_milk.png").await.unwrap();
    enemy_texture.set_filter(FilterMode::Nearest);
    let mut enemy_sprite = AnimatedSprite::new(Rect::new(10f32, 10f32, 100f32, 100f32), enemy_texture, EntityType::Enemy);
    loop{

        frame+=1;
        clear_background(WHITE);
        //draw_texture_ex(&milkboy_texture, pos.x,pos.y, WHITE, DrawTextureParams{dest_size: Some(Vec2 {x:100f32, y:100f32}),source: Some(source_rect), ..Default::default()});
        player_sprite.draw();
        player_sprite.animate(5f32, frame);
        player_sprite.update();
        enemy_sprite.draw();
        enemy_sprite.animate(5f32, frame);
        //source_rect.x = 16f32 * source_player_index as f32;
        //enemy_source_rect.x = 16f32 * source_enemy_index as f32;
        next_frame().await;
    }
}
