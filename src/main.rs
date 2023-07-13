use ggez::event;
use ggez::graphics::{self, Color, Canvas};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::{self, Rng};
use ggez::conf;

static WIDTH: f32 = 1800.0;
static HEIGHT: f32 = 900.0;
static MEMBER_SIZE: f32 = 5.0;

#[derive(Copy, Clone)]
struct BoidMember {
    id: u8,
    variety: Variety,
    pos_x: f32,
    pos_y: f32,
    dir: f32
}

impl BoidMember {
    fn new(id: u8, variety: Variety, pos_x: f32, pos_y: f32, dir: f32) -> BoidMember {
        let m: BoidMember = BoidMember{
            id : id,
            variety: variety,
            pos_x : pos_x,
            pos_y : pos_y, 
            dir : dir
        };
        return m;
    }

    fn get_location(&self) -> Point {
        return Point::new(self.pos_x, self.pos_y);
    }

    fn equals(&self, member: &BoidMember) -> bool {
        return self.id == member.id;
    }

    fn transform(&mut self, point: Point) {
        self.pos_x = point.x;
        self.pos_y = point.y;
    }

    fn reflect(&mut self, wall:Wall) {
        if wall == Wall::Flat {
            if self.dir < 180.0 {//moving up and right
                self.dir = 180.0-self.dir;
            }else if self.dir > 180.0 {//Moving up and left
                self.dir = 540.0-self.dir;
            }
        }else if wall == Wall::Side {
            self.dir = 360.0-self.dir;
        }
    }

    fn steer(&mut self, dir: f32, strength: f32) -> bool {
        if dir < self.dir {
            if self.dir-dir < strength {
                self.dir = dir;
                return true;
            }
            self.dir -= strength;
        }else {
            if dir-self.dir < strength {
                self.dir = dir;
                return true;
            }
            self.dir += strength;
        }
        return false;
    }
}

#[derive(PartialEq, Eq)]//this allows us to compare enums with ==
enum Wall {
    Side, Flat
}

#[derive(Copy, Clone, PartialEq)]
struct Variety {
    r:u8,
    g:u8,
    b:u8
}

impl Variety {
    fn red() -> Variety {
        Variety {
            r: 180, 
            g: 50, 
            b: 30
        }
    }
    fn green() -> Variety {
        Variety {
            r: 50, 
            g: 180, 
            b: 90
        }
    }
    fn blue() -> Variety {
        Variety {
            r: 40, 
            g: 90, 
            b: 180
        }
    }
    fn random() -> Variety {
        let rand = rand::thread_rng().gen_range(0, 4);
        if rand == 0 {
            Self::red()
        }else if rand == 1 {
            Self::green()
        }else {
            Self::blue()
        }
    }
}

fn wrap(position: &mut Point, wall: Wall){
    if wall == Wall::Flat {
        if position.y < 2.0 {//top
            position.y = HEIGHT;
        }else {//bottom
            position.y = 0.0;
        }
    }else if wall == Wall::Side {
        if position.x < 2.0 {//left
            position.x = WIDTH;
        }else {//right
            position.x = 0.0;
        }
    }
}

struct Point {
    x:f32,
    y:f32
}

impl Point {
    fn new(x:f32, y:f32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    fn distance(&self, point: Point) -> f32 {
        let a = self.x-point.x;
        let b = self.y-point.y;
        return ((a*a)+(b*b)).sqrt();
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

fn get_random_float(range:f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0, range) as f32
}

//fn get_random_int(range:u16) -> u16 {
//    let mut rng = rand::thread_rng();
//
//    rng.gen_range(0, range)
//}

fn random_dir() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 360) as f32
}

/**
 * Speed should be 1 for the default step distance. 
 */
fn get_next_pos(position: &mut Point, direction: f32, speed: f32) -> Point {
    let dir = direction;
    let mut pos_x = position.x;

    let mut pos_y = position.y;
    if dir < 90.0 {
        //x = dir/90*step_normal   --  so if we are facing right, dir = 89. 89/90 = 0.9888 = 0.9888. Move right 0.9888. 
        //y = 1-(dir/90)*step_normal   --  so if we are facing right, dir = 89. 1-(89/90) = 0.0111. Move up 0.0111.
        //x = dir/90*step_normal   --  So if we are facing up, dir = 1. 1/90 = 0.0111 = 0.0111. Move right 0.0111.
        //y = 1-(dir/90)*step_normal   --  So if we are facing up, dir = 1. 1-(1/90) = 0.9888. Move up 0.9888. 
        if pos_x > WIDTH {
            wrap(position, Wall::Side);
            pos_x = position.x;
        }
        if pos_y < 0.0 {
            wrap(position, Wall::Flat);
            pos_y = position.y;
        }
        pos_x += (dir/90.0)*speed;
        pos_y -= ((90.0-dir)/90.0)*speed;
    }else if dir < 180.0 {
        if pos_x > WIDTH {
            wrap(position, Wall::Side);
            pos_x = position.x;
        }
        if pos_y > HEIGHT {
            wrap(position, Wall::Flat);
            pos_y = position.y;
        }
        pos_x += ((180.0-dir)/90.0)*speed;
        pos_y += ((dir-90.0)/90.0)*speed;
    }else if dir < 270.0 {
        if pos_x < 0.0 {
            wrap(position, Wall::Side);
            pos_x = position.x;
        }
        if pos_y > HEIGHT {
            wrap(position, Wall::Flat);
            pos_y = position.y;
        }
        pos_x -= ((dir-180.0)/90.0)*speed;
        pos_y += ((270.0-dir)/90.0)*speed;
    }else if dir < 360.0 {
        if pos_x < 0.0 {
            wrap(position, Wall::Side);
            pos_x = position.x;
        }
        if pos_y < 0.0 {
            wrap(position, Wall::Flat);
            pos_y = position.y;
        }
        pos_x -= ((360.0-dir)/90.0)*speed;
        pos_y -= ((dir-270.0)/90.0)*speed;
    }
    return Point::new(pos_x, pos_y);
}

fn average_directions(members: Vec<BoidMember>) -> f32 {
    let mut dir: f32 = 0.0;
    let length: f32 = members.len() as f32;
    for member in members {
        dir += member.dir;
    }
    return dir/length;
}

fn get_nearby_members(variety: Variety, location: Point, boid: &Vec<BoidMember>) -> Vec<BoidMember> {
    let mut nearby: Vec<BoidMember> = Vec::new();
    let mut count = 0;
    while count < boid.len() {
        let each: &BoidMember = &boid[count];
        if location.distance(each.get_location()) < 100.0 {
            if each.variety == variety {
                nearby.insert(0, BoidMember::new(each.id, each.variety, each.pos_x, each.pos_y, each.dir));
            }
        }
        count += 1;
    }
    nearby
}

fn get_crowded_members(variety: Variety, location: Point, boid: &Vec<BoidMember>) -> Vec<BoidMember>{
    let mut nearby: Vec<BoidMember> = Vec::new();
    let mut count = 0;
    while count < boid.len() {
        let each: &BoidMember = &boid[count];
        if location.distance(each.get_location()) < 10.0 {
                nearby.insert(0, BoidMember::new(each.id, each.variety, each.pos_x, each.pos_y, each.dir));
        }
        count += 1;
    }
    nearby
}

fn apply_cohesion(member: &mut BoidMember, members: &Vec<BoidMember>) {
    member.steer(average_directions(get_nearby_members(member.variety, member.get_location(), &members)), 0.2);
}

fn apply_repulsion(member: &mut BoidMember, members: &Vec<BoidMember>) {
    for nearby in members {
        if nearby.equals(member) {
            continue;
        }
        //do repulsion here
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.boid.len() < 100 {
            let count = self.boid.len() as u8;
            MainState::add_boid_member(self, BoidMember::new(count+1, Variety::random(), get_random_float(WIDTH), get_random_float(HEIGHT), random_dir()));
        }
        let mut i = 0;
        while i < self.boid.len() {
            let boid = &mut self.boid;
            let members = boid.clone();
            let mut member: &mut BoidMember = boid.get_mut(i).unwrap();
            let new_loc: Point = get_next_pos(&mut member.get_location(), member.dir, 1.0);
            member.transform(new_loc);

            apply_cohesion(member, &members);
            apply_repulsion(member, &members);
            
            i+=1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from_rgb(0, 70, 85)
        );
        for mut _member in &self.boid {
            let body = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(_member.pos_x, _member.pos_y),
                MEMBER_SIZE,
                0.1,
                Color::from_rgb(_member.variety.r, _member.variety.g, _member.variety.b),
            )?;
            let loc: Point = get_next_pos(&mut _member.get_location(), _member.dir, 7.0);
            let head = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(loc.x, loc.y),
                MEMBER_SIZE/1.7,
                0.1,
                Color::from_rgb(_member.variety.r, _member.variety.g, _member.variety.b),
            )?;
            canvas.draw(&body, Vec2::new(0.0, 0.0));
            canvas.draw(&head, Vec2::new(0.0, 0.0));
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mut mode = conf::WindowMode::default();
    mode.width = WIDTH;
    mode.height = HEIGHT;
    let cb: ggez::ContextBuilder = ggez::ContextBuilder::new("Boyds", "Blue Dev").window_mode(mode);

    let (ctx, event_loop) = cb.build()?;
    let state: MainState = MainState::new()?;
    event::run(ctx, event_loop, state)
}
