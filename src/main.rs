use macroquad::prelude::*;
enum EntityType {
    Enemy,
    Player,
}
#[derive(PartialEq)]
enum GameState{
    Dialogue,
    Active,
    Menu
}

fn move_player(entity_rect: &mut Rect){
    if is_key_down(KeyCode::Right){
        entity_rect.x += 300f32 * get_frame_time() as f32;
    }
    if is_key_down(KeyCode::Left){
        entity_rect.x -= 300f32 * get_frame_time() as f32;
    }
    if is_key_down(KeyCode::Down){
        entity_rect.y += 300f32 * get_frame_time() as f32;
    }
    if is_key_down(KeyCode::Up){
        entity_rect.y -= 300f32 * get_frame_time() as f32;
    }
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
        EntityType::Player => { if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::Up) || is_key_down(KeyCode::Down)) && frame as f32 % anim_speed == 0f32 /*&& get_time() as f32 % anim_speed as f32 >= anim_speed as f32 * ( 0.9f32 - anim_speed as f32 * 0.02*/{
            self.source_index += 1;
            if self.source_index > 3{
                self.source_index = 0;
            }
        }
        if is_key_released(KeyCode::Right) || is_key_released(KeyCode::Left)|| is_key_released(KeyCode::Up)|| is_key_released(KeyCode::Down){
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
    _ => {todo!()}
        }
        self.source_rect.x = 16f32 * self.source_index as f32;
    }
    async fn update_map(&mut self, location: &mut Texture2D, index: &mut Vec2){
        if self.entity_rect.x > screen_width() {
            self.entity_rect.x = 0f32 - self.entity_rect.w;
            index.x += 1f32;
            
        }
        if self.entity_rect.x < 0f32 - self.entity_rect.w {
            self.entity_rect.x = screen_width();
            index.x -= 1f32;
        }
        if self.entity_rect.y > screen_height() {
            self.entity_rect.y = 0f32 - self.entity_rect.h;
            index.y -= 1f32;
        }
        if self.entity_rect.y < 0f32 - self.entity_rect.h {
            self.entity_rect.y = screen_height();
            index.y += 1f32;
        }
        if index.x == 0f32 {
            index.x = 1f32;
        }
        if index.y == 2f32 && self.entity_rect.y < 500f32{
            self.entity_rect.y = 500f32;
        }
        *location = load_texture(format!("/Users/family/milkboy-rust/src/assets/map{}-{}.png", index.x, index.y).as_str()).await.unwrap();
            location.set_filter(FilterMode::Nearest);
    }
    fn update(&mut self){
        match self.entity_type {
            EntityType::Player => {
                move_player(&mut self.entity_rect);
            }
            EntityType::Enemy => {
                self.entity_rect.x += 10f32
            }
        }
    }
}
fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => {
            if is_key_down(KeyCode::Space){
                draw_text("poop", 200f32, 200f32, 30f32, BLACK);
            }
            intersection
        }
        ,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}
/* todo! */
/*async fn setup_animation<const C: usize>(paths: &[&str; C]) -> [Texture2D; C]{
    let mut output: Vec<Texture2D> = Vec::new();
    let mut index: i16 = 0;
    while index < paths.len() as i16{
        output.push(load_texture(paths[index as usize]).await.unwrap());
        output[index as usize].set_filter(FilterMode::Nearest)
    }
    return output
}*/
#[macroquad::main("Milkboy: Rust")]
async fn main() {
    request_new_screen_size(800f32, 640f32);
    let mut frame: i16 = 0;
    let mut game_state: GameState = GameState::Active;
    let mut map_index = Vec2::new(1f32,1f32);
    let mut index = 0;
    let mut indexer = 1;
    let text = "Wow, this grass is green!";
    let mut message:String = "".to_string();
    //let [milkboy_texture, enemy_texture] = setup_animation(&["/Users/family/milkboy-rust/src/assets/milkboy.png","/Users/family/milkboy-rust/src/assets/evil_milk.png"]).await;
    let milkboy_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/milkboy.png").await.unwrap();
    milkboy_texture.set_filter(FilterMode::Nearest);
    let dialogue_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/dialogue-box.png").await.unwrap();
    let mut dialogue_pos: Vec2 = Vec2::new(100f32,250f32);
    dialogue_texture.set_filter(FilterMode::Nearest);
    let mut player_sprite = AnimatedSprite::new(Rect::new(10f32, 10f32, 100f32, 100f32), milkboy_texture, EntityType::Player);
    let enemy_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/evil_milk.png").await.unwrap();
    enemy_texture.set_filter(FilterMode::Nearest);
    let rock_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/rock.png").await.unwrap();
    rock_texture.set_filter(FilterMode::Nearest);
    let mut enemy_sprite = AnimatedSprite::new(Rect::new(10f32, 10f32, 100f32, 100f32), enemy_texture, EntityType::Enemy);
    let mut rock_sprite = AnimatedSprite::new(Rect::new(10f32, 10f32, 100f32, 100f32), rock_texture, EntityType::Enemy);
    let mut map_texture: Texture2D = load_texture("/Users/family/milkboy-rust/src/assets/map1-1.png").await.unwrap();
    map_texture.set_filter(FilterMode::Nearest);
    let mut switch = false;
    loop{
        frame+=1;
        if game_state == GameState::Active{
        player_sprite.animate(5f32, frame);
        player_sprite.update();
        enemy_sprite.animate(5f32, frame);
        enemy_sprite.update();
        }
        clear_background(WHITE);
        draw_texture_ex(&map_texture, 0f32, 0f32, WHITE, DrawTextureParams{dest_size: Some(Vec2 {x:800f32, y:640f32}), ..Default::default()});
        player_sprite.draw();
        player_sprite.update_map(&mut map_texture, &mut map_index).await;
        enemy_sprite.draw();
        rock_sprite.draw();
        if game_state == GameState::Dialogue{
            draw_texture_ex(&dialogue_texture,  dialogue_pos.x, dialogue_pos.y,WHITE, DrawTextureParams{dest_size: Some(Vec2 {x:600f32, y:480f32}), ..Default::default()});
        if index == text.len(){
            indexer = 0;
            index = 0;
        }
        if frame % 5 == 0{
            if !(indexer == 0){
            message = format!("{}{}", message,text.chars().collect::<Vec<_>>()[index]);
            }
            index+=indexer;
        }

        if !(index == text.len()) {
            draw_text(&message, dialogue_pos.x +100f32, dialogue_pos.y+ 200f32, 30f32, WHITE);
        } 
    }
        if switch {
            game_state = GameState::Dialogue;
        } else {
            game_state = GameState::Active;
        }
        if is_key_pressed(KeyCode::Space){
            switch = !switch;
            index = 0;
            indexer = 1;
            message = "".to_string();
        } 
        //source_rect.x = 16f32 * source_player_index as f32;
        //enemy_source_rect.x = 16f32 * source_enemy_index as f32;
        next_frame().await;
    }
}
