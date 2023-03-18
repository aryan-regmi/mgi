use mgi::prelude::*;

struct TestGame {
    running: bool,
    box_pos: (i32, i32),
    box_color: Color,
}

impl Game for TestGame {
    fn setup() -> Self {
        Self {
            running: true,
            box_pos: (300, 300),
            box_color: Color::BLUE,
        }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn update(&mut self, ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        if ctx.key_down(Keycode::Escape) {
            self.running = false;
        }

        let (mouse_x, mouse_y) = ctx.mouse_pos();

        let (x, y) = self.box_pos;

        // If mouse is within the box, change its color
        if mouse_x < x + 200 && mouse_x > x && mouse_y < y + 200 && mouse_y > y {
            self.box_color = Color::BLACK;
        } else {
            self.box_color = Color::BLUE;
        }

        if ctx.left_click() {
            let (x, y) = ctx.mouse_pos();
            let (cx, cy) = (x - 100, y - 100);
            self.box_pos = (cx, cy);
        }

        Ok(())
    }

    fn render(&mut self, ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        ctx.set_clear_color(Color::RGB(100, 100, 100));

        ctx.draw_rect(200, 200, 400, 400, Color::RED, 1.0)?;

        ctx.fill_rect(
            self.box_pos.0,
            self.box_pos.1,
            200,
            200,
            self.box_color,
            1.0,
        )?;

        // ctx.fill_rect(300, 300, 200, 200, Color::BLUE, 0.5)?;

        ctx.draw_line((0, 0), (800, 800), Color::GREEN, 1.0)?;

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    GameBuilder::<TestGame>::init("Hello World", 800, 800)?.run()?;

    Ok(())
}
