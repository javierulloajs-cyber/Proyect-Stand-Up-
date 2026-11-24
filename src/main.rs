use macroquad::prelude::*;
use hecs::*;

#[path ="scripts/componentes.rs"]
mod componentes;
use componentes::*;
#[path ="scripts/sistemas.rs"]
mod sistemas;
#[path = "scripts/recursos.rs"]
mod recursos;

const VIRTUAL_WIDTH: i32 = 1180;
const VIRTUAL_HEIGHT: i32 = 620;

fn config() -> Conf {
    Conf {
        window_title: "Stand Up!".to_string(),
        window_width: VIRTUAL_WIDTH,
        window_height: VIRTUAL_HEIGHT,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let render_tar = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_tar.texture.set_filter(FilterMode::Nearest);
    
    let camara2d = Camera2D {
        render_target: Some(render_tar.clone()),
        zoom: vec2(2.0 / VIRTUAL_WIDTH as f32, 2.0 / VIRTUAL_HEIGHT as f32),
        target: vec2(0.0, 0.0),
        ..Default::default()
    };

    let mut world = World::new();
    let recursos = recursos::Recursos::new().await;

    world.spawn((
        Position(vec2(100.0, 100.0)),
        Hitbox(vec2(50.0, 50.0)),
        recursos.player_sheet,
        Jugador
    ));

    loop {
        // Actualizar


        set_camera(&camara2d);
        clear_background(LIGHTGRAY);
        // Dibujar

        sistemas::sis_render(&world);

        set_default_camera();

        // Lienzo donde se dibujará todo el juego
        let scale = (screen_width() / VIRTUAL_WIDTH as f32).min(screen_height() / VIRTUAL_HEIGHT as f32);
        let dest_x = VIRTUAL_WIDTH as f32 * scale;
        let dest_y = VIRTUAL_HEIGHT as f32 * scale;
        let x = (screen_width() - dest_x) / 2.0;
        let y = (screen_height() - dest_y) / 2.0;

        draw_texture_ex(&render_tar.texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(dest_x, dest_y)), flip_y: false, ..Default::default() });

        next_frame().await;
    }
}