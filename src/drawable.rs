use crate::prelude::Color;
use crate::prelude::MgiContext;
use crate::MgiResult;
use sdl2::rect::Rect;
use std::rc::Rc;

/// Rect Implementation
impl MgiContext {
    pub fn draw_rect(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        mut color: Color,
        alpha: f32,
    ) -> MgiResult<()> {
        color.a = (255. * alpha) as u8;
        let rect = Rect::new(x, y, width, height);

        let mut ctx = self.inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        canvas.set_draw_color(color);
        canvas.draw_rect(rect)?;
        canvas.set_draw_color(self.clear_color);

        Ok(())
    }

    pub fn fill_rect(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        mut color: Color,
        alpha: f32,
    ) -> MgiResult<()> {
        color.a = (255. * alpha) as u8;
        let rect = Rect::new(x, y, width, height);

        let mut ctx = self.inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        canvas.set_draw_color(color);
        canvas.fill_rect(rect)?;
        canvas.set_draw_color(self.clear_color);

        Ok(())
    }
}

/// Line Implementation
impl MgiContext {
    pub fn draw_line(
        &self,
        start: (i32, i32),
        end: (i32, i32),
        mut color: Color,
        alpha: f32,
    ) -> MgiResult<()> {
        color.a = (255. * alpha) as u8;

        let mut ctx = self.inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        canvas.set_draw_color(color);
        canvas.draw_line(start, end)?;
        canvas.set_draw_color(self.clear_color);

        Ok(())
    }
}

impl MgiContext {
    pub fn draw_texture(
        &mut self,
        texture_name: &str,
        src: Option<Rect>,
        dest: Rect,
        rotation: f64,
        alpha: f32,
    ) -> MgiResult<()> {
        let inner = Rc::clone(&self.inner);

        // Get canvas
        let mut ctx = inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        // Get correct texture
        let no_texture_msg = format!("The specified texture (`{}`) does not exist in the TextureManager: Add a texture using TextureManager::add_texture()", texture_name);
        let texture_manager = &mut self.texture_manager.as_mut().ok_or("TextureManager does not exist: Add a texture manager using GameBuilder::add_texture_manager()")?.borrow_mut();
        let raw_texture = texture_manager
            .get_texture_mut(texture_name)
            .ok_or(no_texture_msg)?;

        // Apply alpha to texture
        raw_texture.set_alpha_mod((255. * alpha) as u8);

        // Draw the texture
        canvas.copy_ex(
            raw_texture,
            src,
            Some(dest),
            rotation,
            dest.center(),
            false,
            false,
        )?;

        Ok(())
    }

    #[allow(unused)]
    pub fn draw_texture_simple(&self, texture_name: &str, alpha: f32) -> MgiResult<()> {
        Ok(())
    }
}
