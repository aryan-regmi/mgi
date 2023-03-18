use sdl2::{event::Event, pixels::Color};
use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    context::{Inputs, MgiContext, MgiInnerContext},
    LayerManager, MgiResult, TextureManager,
};

pub trait Game {
    fn setup() -> Self;
    fn is_running(&self) -> bool;
    fn update(&mut self, ctx: &mut MgiContext) -> MgiResult<()>;
    fn render(&mut self, ctx: &mut MgiContext) -> MgiResult<()>;
}

struct GameBuilderState {
    texture_manager_added: bool,
    layer_manager_added: bool,
}

pub struct GameBuilder<T: Game> {
    title: String,
    width: u32,
    height: u32,

    ctx: MgiContext,
    game_obj: T,
    state: GameBuilderState,

    fps: u32,
}

impl<T: Game> GameBuilder<T> {
    pub fn init(title: &str, width: u32, height: u32) -> MgiResult<Self> {
        Ok(Self {
            title: title.into(),
            width,
            height,
            ctx: MgiContext {
                inner: Rc::new(RefCell::new(MgiInnerContext {
                    canvas: None,
                    texture_manager: None,
                    layer_manager: None,
                    inputs: Inputs {
                        key_down: vec![],
                        key_up: vec![],
                        mouse_pos: (0, 0),
                        left_click: false,
                        right_click: false,
                        middle_click: false,
                    },
                })),
                clear_color: Color::WHITE,
            },
            game_obj: T::setup(),
            state: GameBuilderState {
                texture_manager_added: false,
                layer_manager_added: false,
            },
            fps: 60,
        })
    }

    pub fn set_fps(mut self, fps: u32) {
        self.fps = fps;
    }

    pub fn add_texture_manager(mut self, texture_manager: TextureManager) -> MgiResult<Self> {
        if self.state.texture_manager_added == false {
            self.ctx.inner.borrow_mut().texture_manager = Some(texture_manager);
            self.state.texture_manager_added = true;

            return Ok(self);
        }

        return Err("Only one TextureManager can be added to a GameBuilder".into());
    }

    pub fn add_layer_manager(mut self, layer_manager: LayerManager) -> MgiResult<Self> {
        if self.state.layer_manager_added == false {
            self.ctx.inner.borrow_mut().layer_manager = Some(layer_manager);
            self.state.layer_manager_added = true;

            return Ok(self);
        }

        return Err("Only one TextureManager can be added to a GameBuilder".into());
    }

    pub fn run(mut self) -> MgiResult<()> {
        // Initalize SDL
        let sdl_ctx = sdl2::init()?;
        let video_sys = sdl_ctx.video()?;
        let window = video_sys
            .window(&self.title, self.width, self.height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(self.ctx.clear_color);

        // Add canvas to context
        self.ctx.inner.borrow_mut().canvas = Some(canvas);

        // Create event pump
        let mut event_pump = sdl_ctx.event_pump()?;

        // Game loop
        while self.game_obj.is_running() {
            for event in event_pump.poll_iter() {
                if let Event::Quit { .. } = &event {
                    break;
                }

                match event {
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => self.ctx.inner.borrow_mut().inputs.key_down.push(key),

                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => self.ctx.inner.borrow_mut().inputs.key_up.push(key),

                    Event::MouseMotion { x, y, .. } => {
                        self.ctx.inner.borrow_mut().inputs.mouse_pos = (x, y)
                    }

                    Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            self.ctx.inner.borrow_mut().inputs.left_click = true
                        }
                        sdl2::mouse::MouseButton::Middle => {
                            self.ctx.inner.borrow_mut().inputs.middle_click = true
                        }
                        sdl2::mouse::MouseButton::Right => {
                            self.ctx.inner.borrow_mut().inputs.right_click = true
                        }
                        _ => (),
                    },

                    Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            self.ctx.inner.borrow_mut().inputs.left_click = false
                        }
                        sdl2::mouse::MouseButton::Middle => {
                            self.ctx.inner.borrow_mut().inputs.middle_click = false
                        }
                        sdl2::mouse::MouseButton::Right => {
                            self.ctx.inner.borrow_mut().inputs.right_click = false
                        }
                        _ => (),
                    },

                    // TODO: Add scroll wheel handling
                    _ => (),
                }

                self.game_obj.update(&mut self.ctx)?;

                // Remove keys from inputs once they're handled
                self.ctx.inner.borrow_mut().inputs.key_up = vec![];
                self.ctx.inner.borrow_mut().inputs.key_down = vec![];
            }

            // Clear the screen
            self.ctx.inner.borrow_mut().canvas.as_mut().unwrap().clear();

            // Render the game_obj
            self.game_obj.render(&mut self.ctx)?;

            // Display changes to the window
            self.ctx
                .inner
                .borrow_mut()
                .canvas
                .as_mut()
                .unwrap()
                .present();

            // Sleep to maintain fps
            std::thread::sleep(Duration::new(0, 1_000_000_000 / self.fps));
        }

        Ok(())
    }
}
