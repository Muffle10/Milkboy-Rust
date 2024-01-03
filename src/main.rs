use macroquad::prelude::*;
use macroquad::time::*;
use milkboy_rust::sprite::*;
enum AnimType {
    Enemy,
    Player,
}
fn animate(source_index: &mut i16, anim_speed: f32, frame:i16, anim_type: AnimType){
    match anim_type{
    AnimType::Player => { if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right)) && frame as f32 % anim_speed == 0f32 /*&& get_time() as f32 % anim_speed as f32 >= anim_speed as f32 * ( 0.9f32 - anim_speed as f32 * 0.02*/{
        *source_index += 1;
        if *source_index > 3{
            *source_index = 0;
        }
    }
    if is_key_released(KeyCode::Right) || is_key_released(KeyCode::Left){
        *source_index = 0;
    }
},
AnimType::Enemy => {
    if get_time() as f32 % anim_speed as f32 >= anim_speed as f32 * ( 0.9f32 - anim_speed as f32 * 0.02){
        *source_index += 1;
        if *source_index > 4{
            *source_index = 0;
        }
    }
}
    }
}
fn animate_update(source_rect: &mut Rect, index: i16){
    source_rect.x = 16f32 * index as f32;
}
#[macroquad::main("Milkboy: Rust")]
async fn main() {
    let milkboy_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/milkboy.png").await.unwrap();
    milkboy_texture.set_filter(FilterMode::Nearest);
    let enemy_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/evil_milk.png").await.unwrap();
    enemy_texture.set_filter(FilterMode::Nearest);
    let mut pos = Vec2::new(30f32, 30f32);
    let mut enemy_pos = Vec2::new(30f32, 30f32);
    let mut source_player_index = 0;
    let mut source_enemy_index = 0;
    let mut frame = 0;
    let mut enemy_source_rect = Rect::new(source_player_index as f32 *16f32, 0f32, 16f32, 16f32);
    let mut source_rect = Rect::new(source_player_index as f32 *16f32, 0f32, 16f32, 16f32);
    loop{
        animate(&mut source_player_index, 5f32, frame, AnimType::Player);
        //animate(&mut source_enemy_index, anim_speed, AnimType::Enemy);
        if is_key_down(KeyCode::Right){
            pos.x += 300f32 * get_frame_time() as f32;
        }
        if is_key_down(KeyCode::Left){
            pos.x -= 300f32 * get_frame_time() as f32;
        }
        if is_key_down(KeyCode::Down){
            pos.y += 300f32 * get_frame_time() as f32;
        }
        if is_key_down(KeyCode::Up){
            pos.y -= 300f32 * get_frame_time() as f32;
        }
        frame+=1;
        if frame % 5 == 0 {
            source_enemy_index += 1
        }
        if source_enemy_index > 4 {
            source_enemy_index = 0;
        }
        clear_background(WHITE);
        draw_text(&format!("{}", frame), 400f32, 200f32, 100f32, BLACK);
        draw_text(&format!("{}", get_frame_time() as f32), 400f32, 300f32, 100f32, BLACK);
        draw_texture_ex(&milkboy_texture, pos.x,pos.y, WHITE, DrawTextureParams{dest_size: Some(Vec2 {x:100f32, y:100f32}),source: Some(source_rect), ..Default::default()});
        draw_texture_ex(&enemy_texture, enemy_pos.x,enemy_pos.y, WHITE, DrawTextureParams{dest_size: Some(Vec2 {x:50f32, y:50f32}),source: Some(enemy_source_rect), ..Default::default()});
        animate_update(&mut source_rect, source_player_index);
        animate_update(&mut enemy_source_rect, source_enemy_index);
        //source_rect.x = 16f32 * source_player_index as f32;
        //enemy_source_rect.x = 16f32 * source_enemy_index as f32;
        next_frame().await;
    }
}
