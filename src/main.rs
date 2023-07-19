mod entities;

use ggez::event;
use ggez::graphics::{self, Color, Canvas};
use ggez::{Context, GameResult};
use ggez::glam::*;
use rand::{self, Rng};
use ggez::conf;
use entities::{BoidMember, Variety, VarietyMatcher, average_directions};
use geometry_2d::geometry::{Direction, Position, Axis};

static WIDTH: f32 = 1800.0;
static HEIGHT: f32 = 900.0;
static MEMBER_SIZE: f32 = 5.0;
//static MEMBER_SPEED: f32 = 1.3;

fn wrap(position: &mut Position, axis: Axis){
    if axis == Axis::Vertical {
        if position.y < 2.0 {//top
            position.y = HEIGHT;
        }else {//bottom
            position.y = 0.0;
        }
    }else if axis == Axis::Horizontal {
        if position.x < 2.0 {//left
            position.x = WIDTH;
        }else {//right
            position.x = 0.0;
        }
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

/**
 * Speed should be 1 for the default step distance. 
 */
fn get_next_pos(position: &mut Position, direction: f32, speed: f32) -> Position {
    let dir = direction;
    let mut pos_x = position.x;

    let mut pos_y = position.y;
    if dir < 90.0 {
        if pos_x > WIDTH {
            wrap(position, Axis::Horizontal);
            pos_x = position.x;
        }
        if pos_y < 0.0 {
            wrap(position, Axis::Vertical);
            pos_y = position.y;
        }
        pos_x += (dir/90.0)*speed;
        pos_y -= ((90.0-dir)/90.0)*speed;
    }else if dir < 180.0 {
        if pos_x > WIDTH {
            wrap(position, Axis::Horizontal);
            pos_x = position.x;
        }
        if pos_y > HEIGHT {
            wrap(position, Axis::Vertical);
            pos_y = position.y;
        }
        pos_x += ((180.0-dir)/90.0)*speed;
        pos_y += ((dir-90.0)/90.0)*speed;
    }else if dir < 270.0 {
        if pos_x < 0.0 {
            wrap(position, Axis::Horizontal);
            pos_x = position.x;
        }
        if pos_y > HEIGHT {
            wrap(position, Axis::Vertical);
            pos_y = position.y;
        }
        pos_x -= ((dir-180.0)/90.0)*speed;
        pos_y += ((270.0-dir)/90.0)*speed;
    }else if dir < 360.0 {
        if pos_x < 0.0 {
            wrap(position, Axis::Horizontal);
            pos_x = position.x;
        }
        if pos_y < 0.0 {
            wrap(position, Axis::Vertical);
            pos_y = position.y;
        }
        pos_x -= ((360.0-dir)/90.0)*speed;
        pos_y -= ((dir-270.0)/90.0)*speed;
    }
    Position::new(pos_x, pos_y)
}

fn average_locations(members: Vec<BoidMember>) -> Position {
    let mut pos_x: f32 = 0.0;
    let mut pos_y: f32 = 0.0;
    let length: f32 = members.len() as f32;
    for member in members {
        pos_x += member.get_location().x;
        pos_y += member.get_location().y;
    }
    Position::new(pos_x/length, pos_y/length)
}

fn get_nearby_members(variety: Variety, location: Position, boid: &Vec<BoidMember>, matcher: VarietyMatcher, dist: f32) -> Vec<BoidMember> {
    let mut nearby: Vec<BoidMember> = Vec::new();
    let mut count = 0;
    while count < boid.len() {
        let each: &BoidMember = &boid[count];
        if location.distance(each.get_location()) <= dist {
            if matcher == VarietyMatcher::Introvert {
                if each.variety == variety {
                    nearby.insert(0, BoidMember::new(each.id, each.variety, each.pos_x, each.pos_y, each.dir, each.size));
                }
            }else if matcher == VarietyMatcher::Extrovert {
                if each.variety != variety {
                    nearby.insert(0, BoidMember::new(each.id, each.variety, each.pos_x, each.pos_y, each.dir, each.size));
                }
            }else if matcher == VarietyMatcher::Oblivious {
                nearby.insert(0, BoidMember::new(each.id, each.variety, each.pos_x, each.pos_y, each.dir, each.size));
            }
        }
        count += 1;
    }
    nearby
}

fn apply_attraction(member: &mut BoidMember, members: &Vec<BoidMember>) {
    member.approach(average_locations(get_nearby_members(member.variety, member.get_location(), &members, VarietyMatcher::Introvert, 200.0)), 1.0);
}

fn apply_cohesion(member: &mut BoidMember, members: &Vec<BoidMember>) {
    member.conform(average_directions(get_nearby_members(member.variety, member.get_location(), &members, VarietyMatcher::Introvert, 100.0)), 1.0);
}

fn apply_repulsion(member: &mut BoidMember, members: &Vec<BoidMember>) {
    member.repel(average_locations(get_nearby_members(member.variety, member.get_location(), &members, VarietyMatcher::Extrovert, 50.0)), 1.0);
}

fn apply_collision(member: &mut BoidMember, members: &Vec<BoidMember>) {
    let near: Vec<BoidMember> = get_nearby_members(member.variety, member.get_location(), &members, VarietyMatcher::Oblivious, 5.0);
    if near.len() > 0 {
        member.collide(&mut get_nearby_members(member.variety, member.get_location(), &members, VarietyMatcher::Oblivious, 5.0)[0]);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.boid.len() < 100 {
            let count = self.boid.len() as u8;
            MainState::add_boid_member(self, BoidMember::new(count+1, Variety::random(), get_random_float(WIDTH), get_random_float(HEIGHT), Direction::new_random(), MEMBER_SIZE+get_random_float(2.0)));
        }
        let mut i = 0;
        while i < self.boid.len() {
            let boid = &mut self.boid;
            let members = boid.clone();
            let member: &mut BoidMember = boid.get_mut(i).unwrap();
            let new_loc: Position = get_next_pos(&mut member.get_location(), member.dir.angle, 2.0 - (get_nearby_members(member.variety, member.get_location(), &members, VarietyMatcher::Extrovert, 50.0).len() as f32 / 100.0));
            member.transform(new_loc);

            apply_attraction(member, &members);
            apply_cohesion(member, &members);
            apply_repulsion(member, &members);
            apply_collision(member, &members);
            
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
                _member.size,
                0.1,
                Color::from_rgb(_member.variety.r, _member.variety.g, _member.variety.b),
            )?;
            let loc: Position = _member.get_location().extend_forward(_member.dir, _member.size+2.0);
            let head = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(loc.x, loc.y),
                _member.size/1.7,
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
