use macroquad::prelude::*;

pub struct Position(pub Vec2);
pub struct Hitbox(pub Vec2);

// Componentes complejos
pub struct AnimacionDatos {
    pub texture: Texture2D,
    pub colas: f32,
    pub filas: f32,
    pub tile_w: f32,
    pub tile_h: f32
}

// Major tags
pub struct Jugador;