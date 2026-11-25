use macroquad::experimental::animation::AnimatedSprite;

use crate::*;

pub fn sis_actualizar(world: &mut World) {
    for (position, hitbox, animated_sprite) in &mut world.query::<(&mut Position, &mut Hitbox, &mut AnimatedSprite)>() {
        animated_sprite.update();
    }
}

pub fn sis_render(world: &World) {
    for (position, hitbox, texture, animated_sprite) in &mut world.query::<(&Position, &Hitbox, &Texture2D, &AnimatedSprite)>() {

        draw_rectangle(position.0.x, position.0.y, hitbox.0.x, hitbox.0.y, BLACK);

        let rect = animated_sprite.frame().dest_size;
        draw_texture_ex(
            &texture,
            position.0.x, position.0.y, WHITE,
            DrawTextureParams {
                source: Some(animated_sprite.frame().source_rect),
                dest_size: Some(vec2(rect.x * 2.0, rect.y * 2.0)),
                ..Default::default() 
            }
        );
    }
}