use std::{borrow::Borrow, cell::RefMut, error::Error};

use raylib::{
    prelude::{Color, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    RaylibHandle, RaylibThread,
};

use crate::{
    game_builder::Drawable,
    layers::TextureLayer,
    prelude::{TextureManagerRef, TileMapRef},
    utils::{RenderContext, RenderThread},
};

pub struct Renderer<'l> {
    rl: RenderContext,
    rt: RenderThread,
    // TODO: Make this a generic layers vector
    pub(crate) texture_layers: Vec<TextureLayer<'l>>,
}

impl<'l> Renderer<'l> {
    pub(crate) fn new(rl: RenderContext, rt: RenderThread) -> Self {
        Self {
            rl,
            rt,
            texture_layers: Vec::new(),
        }
    }

    pub fn rl(&self) -> RefMut<RaylibHandle> {
        self.rl.borrow_mut()
    }

    pub fn rt(&self) -> &RaylibThread {
        self.rt.borrow()
    }

    // TODO: Get rid of this -> renderer will only every draw layers!
    pub fn draw_texture_layers(
        &self,
        texture_manager: &TextureManagerRef,
    ) -> Result<(), Box<dyn Error>> {
        let (mut rl, rt) = (self.rl(), self.rt());
        let mut d = rl.begin_drawing(rt);

        for layer in &self.texture_layers {
            if !layer.visible {
                continue;
            }

            let srcs = &layer.object_srcs;
            let dests = &layer.object_dests;
            let rots = &layer.object_rotations;

            for (i, texture_name) in layer.objects.iter().enumerate() {
                let texture = texture_manager
                    .get_texture(texture_name)
                    .ok_or(format!(
                        "Texture `{}` does not exist in the texture manager. Ensure that it is added before trying to render it.",
                    texture_name))?.raw_texture().ok_or("[FATAL ERROR] Texture has not been loaded")?;

                let src = if let Some(src) = srcs[i] {
                    src
                } else {
                    Rectangle::new(0., 0., texture.width as f32, texture.height as f32)
                };

                d.draw_texture_pro(
                    texture,
                    src,
                    dests[i],
                    Vector2::new(0., 0.),
                    rots[i],
                    Color::WHITE,
                )
            }
        }

        Ok(())
    }

    // TODO: Make sure tilemap exists first
    pub fn draw_tilemap(
        &self,
        tilemap: &mut Option<TileMapRef>,
        texture_manager: &Option<TextureManagerRef>,
    ) {
        // TODO: Proper error handlin
        std::borrow::BorrowMut::borrow_mut(&mut tilemap.as_mut().unwrap().0).render(
            self,
            texture_manager,
            &mut None,
        )
    }
}
