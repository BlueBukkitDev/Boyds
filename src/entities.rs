use crate::utils::Point;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct BoidMember {
    pub id: u8,
    pub variety: Variety,
    pub pos_x: f32,
    pub pos_y: f32,
    pub dir: f32,
    pub size: f32
}

impl BoidMember {
    pub fn new(id: u8, variety: Variety, pos_x: f32, pos_y: f32, dir: f32, size: f32) -> BoidMember {
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

    pub fn get_location(&self) -> Point {
        return Point::new(self.pos_x, self.pos_y);
    }

    /*fn equals(&self, member: &BoidMember) -> bool {
        return self.id == member.id;
    }*/

    pub fn transform(&mut self, point: Point) {
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

    pub fn approach(&mut self, target: Point, strength: f32) {
        self.conform(self.get_location().get_dir(target), strength);
    }

    pub fn repel(&mut self, deterrent: Point, strength: f32) {
        let new_dir = deterrent.get_dir(self.get_location());
        if self.dir > new_dir {
            self.dir += strength;
        }else {
            self.dir -= strength;
        }
        self.dir = (self.dir+360.0)%360.0;
    }
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