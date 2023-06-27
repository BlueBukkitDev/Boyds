use ggez::event;
use ggez::graphics::{self, Color, Canvas};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::{self, Rng};

struct BoidMember {
    pos_x: f32,
    pos_y: f32,
    dir: u16
}

impl BoidMember {
    fn new(pos_x:f32, pos_y:f32, dir:u16) -> BoidMember {
        let m: BoidMember = BoidMember{
            pos_x : pos_x,
            pos_y : pos_y, 
            dir : dir
        };
        return m;
    }

    fn transform(&mut self, x:f32, y:f32) {
        self.pos_x = x;
        self.pos_y = y;
    }

    fn reflect(&mut self, wall:Wall) {
        if wall == Wall::Top {
            //we're -90 to +90 here
        }
        self.dir += 180;
        if self.dir >= 360{
            self.dir -= 360;
        }
    }
}

#[derive(PartialEq, Eq)]//this allows us to compare enums with ==
enum Wall {
    Top, Right, Bottom, Left
}

struct Point {
    x:f32,
    y:f32
}

impl Point {
    fn new(x:f32, y:f32) -> Point {
        let p = Point {
            x: x,
            y: y,
        };
        return p;
    }
}

struct MainState {
    boid:Vec<BoidMember>
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s: MainState = MainState{boid:Vec::new()}; //{ pos_x: 0.0};
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

//fn get_random_int(range:u16) -> u16 {
//    let mut rng = rand::thread_rng();
//
//    rng.gen_range(0, range)
//}

fn random_dir() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 360)
}

fn move_member(member:&mut BoidMember) -> Point {
    let mut _dir = member.dir as f32;
    let mut pos_x = member.pos_x;

    let mut pos_y = member.pos_y;
    if _dir/90.0 < 1.0 {
        //x = dir/90*step_normal   --  so if we are facing right, dir = 89. 89/90 = 0.9888 = 0.9888. Move right 0.9888. 
        //y = 1-(dir/90)*step_normal   --  so if we are facing right, dir = 89. 1-(89/90) = 0.0111. Move up 0.0111.
        //x = dir/90*step_normal   --  So if we are facing up, dir = 1. 1/90 = 0.0111 = 0.0111. Move right 0.0111.
        //y = 1-(dir/90)*step_normal   --  So if we are facing up, dir = 1. 1-(1/90) = 0.9888. Move up 0.9888. 
        pos_x += _dir/90.0;
        pos_y -= 1.0-(_dir/90.0);
        if pos_x > 800.0 {
            pos_x -= _dir/90.0;
            member.reflect(Wall::Right);
        }
        if pos_y < 0.0 {
            pos_y += 1.0-(_dir/90.0);
            member.reflect(Wall::Top);
        }
    }else if _dir/90.0 < 2.0 {
        _dir %= 90.0;
        pos_x += _dir/90.0;
        pos_y += 1.0-(_dir/90.0);
        if pos_x > 800.0 {
            pos_x -= _dir/90.0;
            member.reflect(Wall::Right);
        }
        if pos_y > 600.0 {
            pos_y -= 1.0-(_dir/90.0);
            member.reflect(Wall::Bottom);
        }
    }else if _dir/90.0 < 3.0 {
        _dir %= 90.0;
        pos_x -= _dir/90.0;
        pos_y += 1.0-(_dir/90.0);
        if pos_x < 0.0 {
            pos_x += _dir/90.0;
            member.reflect(Wall::Left);
        }
        if pos_y > 600.0 {
            pos_y -= 1.0-(_dir/90.0);
            member.reflect(Wall::Bottom);
        }
    }else if _dir/90.0 < 4.0 {
        _dir %= 90.0;
        pos_x -= _dir/90.0;
        pos_y -= 1.0-(_dir/90.0);
        if pos_x < 0.0 {
            pos_x += _dir/90.0;
            member.reflect(Wall::Left);
        }
        if pos_y < 0.0 {
            pos_y += 1.0-(_dir/90.0);
            member.reflect(Wall::Top);
        }
    }
    return Point::new(pos_x, pos_y);
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        if self.boid.len() < 50 {
            MainState::add_boid_member(self, BoidMember::new(get_random_float(800), get_random_float(600), random_dir()));
        }
        let mut i = 0;
        while i < self.boid.len() {
            let mut member = &mut self.boid[i];
            let new_loc: Point = move_member(&mut member);
            self.boid[i].transform(new_loc.x, new_loc.y);
            i+=1;
        }

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
