use geometry_2d::geometry::{Direction, Position};
use rand::Rng;

#[derive(Copy, Clone)]
pub struct BoidMember {
    pub id: u8,
    pub variety: Variety,
    pub pos_x: f32,
    pub pos_y: f32,
    pub dir: Direction,
    pub size: f32
}

impl BoidMember {
    pub fn new(id: u8, variety: Variety, pos_x: f32, pos_y: f32, dir: Direction, size: f32) -> BoidMember {
        let m: BoidMember = BoidMember{
            id : id,
            variety: variety,
            pos_x : pos_x,
            pos_y : pos_y, 
            dir : dir,
            size: size
        };
        return m;
    }

    pub fn get_location(&self) -> Position {
        return Position::new(self.pos_x, self.pos_y);
    }

    /*fn equals(&self, member: &BoidMember) -> bool {
        return self.id == member.id;
    }*/

    pub fn transform(&mut self, point: Position) {
        self.pos_x = point.x;
        self.pos_y = point.y;
    }

    /*pub fn reflect(&mut self, wall:Wall) {
        if wall == Wall::Flat {
            if self.dir < 180.0 {//moving up and right
                self.dir = 180.0-self.dir;
            }else if self.dir > 180.0 {//Moving up and left
                self.dir = 540.0-self.dir;
            }
        }else if wall == Wall::Side {
            self.dir = 360.0-self.dir;
        }
    }*/

    pub fn conform(&mut self, dir: f32, strength: f32) -> bool {
        if dir < self.dir.angle {
            if self.dir.angle-dir < strength {
                self.dir.angle = dir;
                return true;
            }
            self.dir.angle -= strength;
        }else {
            if dir-self.dir.angle < strength {
                self.dir.angle = dir;
                return true;
            }
            self.dir.angle += strength;
        }
        return false;
    }

    pub fn approach(&mut self, target: Position, strength: f32) {
        self.conform(self.get_location().get_dir(target), strength);
    }

    pub fn repel(&mut self, deterrent: Position, strength: f32) {//////////////////There will be a rollover issue here
        let new_dir = deterrent.get_dir(self.get_location());
        if self.dir.angle > new_dir {
            self.dir.angle += strength;
        }else {
            self.dir.angle -= strength;
        }
        self.dir.angle = (self.dir.angle+360.0)%360.0;
    }

    pub fn collide(&mut self, obstacle: &mut BoidMember) {
        let diff = Direction::difference(self.dir, obstacle.dir);
        //println!("[dirs] 1:{}, 2:{}", dir1.angle, dir2.angle);
        if self.dir.is_cw_of(obstacle.dir) {
            self.dir.add(diff/2.0);
            obstacle.dir.subtract(diff/2.0);
        }else{
            self.dir.subtract(diff/2.0);
            obstacle.dir.add(diff/2.0);
        }
    }
}

pub fn average_directions(members: Vec<BoidMember>) -> f32 {
    let length: f32 = members.len() as f32;
    if length <= 0.0 {
        return 0.0;
    }
    let mut dir: f32 = members[0].dir.angle/length;
    let mut count = 1;
    while count < members.len() {//Need to use iterative averaging instead of batch averaging. 
        if (dir - members[count].dir.angle).abs() > 180.0 {
            dir += members[count].dir.angle/length;
            dir += 180.0;//invert
            dir = (360.0+dir)%360.0;
        }else{
            dir += members[count].dir.angle/length;
            dir = (360.0+dir)%360.0;
        }
        count += 1;
    }
    dir
}

#[derive(PartialEq, Eq)]
pub enum VarietyMatcher {
    Introvert, Oblivious, Extrovert
}

#[derive(Copy, Clone, PartialEq)]
pub struct Variety {
    pub r:u8,
    pub g:u8,
    pub b:u8
}

impl Variety {
    pub fn red() -> Variety {
        Variety {
            r: 180, 
            g: 50, 
            b: 30
        }
    }
    pub fn green() -> Variety {
        Variety {
            r: 50, 
            g: 180, 
            b: 90
        }
    }
    pub fn blue() -> Variety {
        Variety {
            r: 40, 
            g: 90, 
            b: 180
        }
    }
    pub fn random() -> Variety {
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