use mgi::{Drawable, Event, EventHandler, Game, Keycode};

pub(crate) struct TestGame<'a> {
    pub(crate) renderer: &'a mut dyn Drawable,
    pub(crate) event_handler: &'a mut dyn EventHandler,
    running: bool,
}

impl<'a> TestGame<'a> {
    pub(crate) fn new(
        renderer: &'a mut dyn Drawable,
        event_handler: &'a mut dyn EventHandler,
    ) -> Self {
        Self {
            renderer,
            event_handler,
            running: true,
        }
    }
}

impl<'a> Drawable for TestGame<'a> {
    fn update(&mut self) {
        self.renderer.update()
    }

    fn render(&mut self, canvas: &mut mgi::Canvas<mgi::Window>) {
        self.renderer.render(canvas)
    }

    fn setup(&mut self, canvas: &mut mgi::Canvas<mgi::Window>) {
        self.renderer.setup(canvas)
    }
}

impl<'a> EventHandler for TestGame<'a> {
    fn handle_events(&mut self, event: mgi::Event) {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running = false,
            _ => self.event_handler.handle_events(event),
        }
    }
}

impl<'a> Game for TestGame<'a> {
    fn is_running(&self) -> bool {
        self.running
    }
}
