use crate::{
    context::{Inputs, MgiContext},
    prelude::{LayerManager, TextureManager},
    MgiResult,
};
use sdl2::{event::Event, pixels::Color, video::GLProfile};
use std::cell::RefCell;

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
}

impl<T: Game> GameBuilder<T> {
    pub fn init(title: &str, width: u32, height: u32) -> MgiResult<Self> {
        Ok(Self {
            title: title.into(),
            width,
            height,
            ctx: MgiContext {
                canvas: None,
                inputs: Inputs {
                    key_down: vec![],
                    key_up: vec![],
                    mouse_pos: (0, 0),
                    left_click: false,
                    right_click: false,
                    middle_click: false,
                },
                clear_color: Color::WHITE,
                texture_manager: None,
                layer_manager: None,
            },
            game_obj: T::setup(),
            state: GameBuilderState {
                texture_manager_added: false,
                layer_manager_added: false,
            },
        })
    }

    pub fn add_texture_manager(mut self, texture_manager: TextureManager) -> MgiResult<Self> {
        if self.state.texture_manager_added == false {
            self.ctx.texture_manager = Some(RefCell::new(texture_manager));
            self.state.texture_manager_added = true;

            return Ok(self);
        }

        return Err("Only one TextureManager can be added to a GameBuilder".into());
    }

    pub fn add_layer_manager(mut self, layer_manager: LayerManager) -> MgiResult<Self> {
        if self.state.layer_manager_added == false {
            self.ctx.layer_manager = Some(layer_manager);
            self.state.layer_manager_added = true;

            return Ok(self);
        }

        return Err("Only one TextureManager can be added to a GameBuilder".into());
    }

    pub fn run(mut self) -> MgiResult<()> {
        // Initalize SDL
        let sdl_ctx = sdl2::init()?;
        let video_sys = sdl_ctx.video()?;
        let gl_attr = video_sys.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core); // Don't use deprecated OpenGL functions
        gl_attr.set_context_version(3, 2); // Set the OpenGL context version (OpenGL 3.2)
        gl_attr.set_multisample_buffers(1); // Enable anti-aliasing
        gl_attr.set_multisample_samples(4);
        let window = video_sys
            .window(&self.title, self.width, self.height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(self.ctx.clear_color);

        // Load all textures
        if let Some(tm) = self.ctx.texture_manager.as_mut() {
            tm.borrow_mut().load_textures(canvas.texture_creator())?;
        }

        // Add canvas to context
        self.ctx.canvas = Some(canvas);

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
                    } => self.ctx.inputs.key_down.push(key),

                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => self.ctx.inputs.key_up.push(key),

                    Event::MouseMotion { x, y, .. } => self.ctx.inputs.mouse_pos = (x, y),

                    Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                        sdl2::mouse::MouseButton::Left => self.ctx.inputs.left_click = true,
                        sdl2::mouse::MouseButton::Middle => self.ctx.inputs.middle_click = true,
                        sdl2::mouse::MouseButton::Right => self.ctx.inputs.right_click = true,
                        _ => (),
                    },

                    Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                        sdl2::mouse::MouseButton::Left => self.ctx.inputs.left_click = false,
                        sdl2::mouse::MouseButton::Middle => self.ctx.inputs.middle_click = false,
                        sdl2::mouse::MouseButton::Right => self.ctx.inputs.right_click = false,
                        _ => (),
                    },

                    // TODO: Add scroll wheel handling
                    _ => (),
                }

                self.game_obj.update(&mut self.ctx)?;

                // Remove keys from inputs once they're handled
                self.ctx.inputs.key_up = vec![];
                self.ctx.inputs.key_down = vec![];
            }

            // Clear the screen
            self.ctx.canvas.as_mut().unwrap().clear();

            // Render the game_obj
            self.game_obj.render(&mut self.ctx)?;

            // Render changes to the window
            self.ctx.canvas.as_mut().unwrap().present();
        }

        Ok(())
    }
}
