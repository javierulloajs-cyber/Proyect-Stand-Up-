use futures::future::join_all;
use macroquad::audio::{Sound, load_sound};
use macroquad::experimental::animation::{self, AnimatedSprite, Animation};

use crate::*;

pub struct Recursos {
    pub player_sheet: Texture2D,
    pub player_animatedsprite: animation::AnimatedSprite
}

impl Recursos {
    pub async fn new() -> Self {
        let mut rutas_sprites = Vec::new();
        rutas_sprites.push("assets/sprites/Player_Sheet.png");

        let sprites_cargados = Self::cargar_texturas(rutas_sprites).await;

        Self {
            player_sheet: sprites_cargados[0].clone(),
            player_animatedsprite: AnimatedSprite::new(
                32, 32,
                &[
                    Animation {
                        name: "idle".to_string(),
                        row: 0,
                        frames: 6,
                        fps: 12
                    }
                ],
                true // Loop
            )
        }
    }
    pub async fn cargar_texturas(rutas: Vec<&str>) -> Vec<Texture2D> {
        let imgs = rutas.iter()
        .map(|x| { load_image(x) });

        let resultados = join_all(imgs).await;

        resultados
            .iter()
            .map(|x| {
                if let Ok(y) = x {
                    let t = Texture2D::from_image(y);
                    t.set_filter(FilterMode::Nearest);
                    t
                }
                else { panic!("Error al intentar cargar la imagen") }
            })
            .collect()
    }
    pub async fn cargar_audios(rutas: Vec<&str>) -> Vec<Sound> {
        let sonidos = rutas.iter()
        .map(|x| { load_sound(x) });

        let resultados = join_all(sonidos).await;

        resultados
            .into_iter()
            .map(|x| {
                x.unwrap_or_else(|e| { panic!("No se pudo importar el sonido: {}", e) })
            })
            .collect()
    }
    
}