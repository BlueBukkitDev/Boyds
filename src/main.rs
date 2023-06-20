use ggez::event;
use ggez::graphics::{self, Color, Canvas};
use ggez::{Context, GameResult};
use ggez::glam::*;

struct MainState {
    //pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s: MainState = MainState{}; //{ pos_x: 0.0};
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from_rgb(0, 70, 85)
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        canvas.draw(&circle, Vec2::new(400.0, 400.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb: ggez::ContextBuilder = ggez::ContextBuilder::new("Boyds", "Blue Dev");
    let (ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new()?;
    event::run(ctx, event_loop, state)
}
