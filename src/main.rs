use ggez::event;
use ggez::graphics::{self, Color, Canvas};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::{self, Rng};

struct BoidMember {
    pos_x: f32,
    pos_y: f32,
    dir: Vec<f32>
}

impl BoidMember {
    fn new(pos_x:f32, pos_y:f32, dir:Vec<f32>) -> BoidMember {
        let m: BoidMember = BoidMember{
            pos_x : pos_x,
            pos_y : pos_y, 
            dir : dir
        };
        return m;
    }
}

struct MainState {
    boid:Vec<BoidMember>,
    timer:i32
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s: MainState = MainState{boid:Vec::new(), timer:0}; //{ pos_x: 0.0};
        Ok(s)
    }

    fn add_boid_member(&mut self, member:BoidMember) {
        self.boid.insert(0, member);
    }
}

fn get_random_float(range:i32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0, range) as f32
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        if self.timer % 60 == 0 && self.boid.len() < 200 {
            MainState::add_boid_member(self, BoidMember::new(get_random_float(800), get_random_float(600), vec![10.0]));
            println!("{} members", self.boid.len());
        }
        self.timer += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from_rgb(0, 70, 85)
        );
        for _member in &self.boid {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(_member.pos_x, _member.pos_y),
                5.0,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&circle, Vec2::new(0.0, 0.0));
        }

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
