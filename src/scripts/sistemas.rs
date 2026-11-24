use crate::{componentes::*, recursos::AnimacionDatos, *};

pub fn sis_render(world: &World) {
    for (position, hitbox, animacion_datos) in &mut world.query::<(&Position, &Hitbox, &AnimacionDatos)>() {
        draw_rectangle(position.0.x, position.0.y, hitbox.0.x, hitbox.0.y, BLACK);

        let rect = Rect {
            x: 0.0, y: 0.0,
            w: 32.0, h: 32.0
        };

        draw_texture_ex(
            &animacion_datos.texture,
            position.0.x, position.0.y, WHITE,
            DrawTextureParams { dest_size: Some(vec2(100.0, 100.0)), source: Some(rect), ..Default::default() }
        );
    }
}