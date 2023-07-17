use rand::Rng;

#[derive(Copy, Clone)]
pub struct Direction {
    pub angle:f32
}

impl Direction {
    pub fn new(angle:f32) -> Direction {
        Direction {
            angle:angle
        }
    }

    pub fn new_random() -> Direction {
        let mut rng = rand::thread_rng();
        Direction {
            angle:rng.gen_range(0, 360) as f32
        }
    }

    /** 
     * Returns the absolute value of the real difference between two directions. 
     * If `angle1` is 300.0 and `angle2` is 15.0, the difference is 75.0. Because of this wrapping, difference returned cannot exceed 180.0. 
    */
    pub fn difference(angle1:Direction, angle2:Direction) -> f32 {
        (angle1.angle-angle2.angle).abs()%180.0
    }

    /**
     * Rolls the `Direction` CCW by "amt" degrees. 
     */
    pub fn subtract(&mut self, amt:f32) {
        self.angle = (self.angle-amt) % 360.0;
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }

    /**
     * Rolls the `Direction` CW by "amt" degrees
     */
    pub fn add(&mut self, amt:f32) {
        self.angle = (self.angle+amt) % 360.0;
    }

    /**
     * Returns whether `self` is clockwise of "dir". 
     * If `is_cw_of(self, dir)` and `is_cw_of(dir, self)` both return false, then the two angles are exactly opposite.
     */
    pub fn is_cw_of(&self, dir:Direction) -> bool {
        if self.angle > 180.0 {//self is left
            if dir.angle > 180.0 {//dir is left
                if self.angle < dir.angle {//self is lower/ccw
                    return false;
                }else if self.angle > dir.angle {//self is higher/cw
                    return true;
                }
            }else if dir.angle < 180.0 {//dir is right
                if self.angle-dir.angle < 180.0 {//angle open to the bottom
                    return true;
                }else if self.angle-dir.angle > 180.0 {//angle open to the top
                    return false;
                }
            }
        }else if self.angle < 180.0 {//self is right
            if dir.angle < 180.0 {//dir is right
                if self.angle < dir.angle {//self is higher/ccw
                    return false;
                }else if self.angle > dir.angle {//self is lower/cw
                    return true;
                }
            }else if dir.angle > 180.0 {//dir is left
                if dir.angle-self.angle < 180.0 {//angle open to the bottom
                    return false;
                }else if dir.angle-self.angle > 180.0 {//angle open to the top
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x:f32,
    pub y:f32
}

impl Point {
    pub fn new(x:f32, y:f32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn distance(&self, point: Point) -> f32 {
        let a = self.x-point.x;
        let b = self.y-point.y;
        return ((a*a)+(b*b)).sqrt();
    }

    /**
    * Returns a `Point` that is "dist" distance away from the "origin" `Point` in `Direction` "dir". 
    */
    pub fn extend_forward(&self, dir: Direction, dist: f32) -> Point {
        let mut pos_x = self.x;
        let mut pos_y = self.y;
    
        if dir.angle < 90.0 {
            pos_x += (dir.angle/90.0)*dist;
            pos_y -= ((90.0-dir.angle)/90.0)*dist;
        }else if dir.angle < 180.0 {
            pos_x += ((180.0-dir.angle)/90.0)*dist;
            pos_y += ((dir.angle-90.0)/90.0)*dist;
        }else if dir.angle < 270.0 {
            pos_x -= ((dir.angle-180.0)/90.0)*dist;
            pos_y += ((270.0-dir.angle)/90.0)*dist;
        }else if dir.angle < 360.0 {
            pos_x -= ((360.0-dir.angle)/90.0)*dist;
            pos_y -= ((dir.angle-270.0)/90.0)*dist;
        }
        return Point::new(pos_x, pos_y);
    }

    pub fn get_dir(&self, target: Point) -> f32 {//cos(theta) = adj/hyp, so //acos(adj/hyp) = theta
        if target.x >  self.x {//right
            if target.y < self.y {//top
                return f32::acos((self.y-target.y)/self.distance(target))//top right
            }//bottom
            return 90.0+f32::acos((target.x-self.x)/self.distance(target))//bottom right
        }//left
        if target.y > self.y {//bottom
            return 180.0+f32::acos(target.y-self.y)/self.distance(target)//bottom left
        }//top
        return 270.0+f32::acos((self.x-target.x)/self.distance(target))//top left
    }
}

#[derive(PartialEq, Eq)]
pub enum Wall {
    Side, Flat
}

#[derive(PartialEq, Eq)]
pub enum VarietyMatcher {
    Introvert, Oblivious, Extrovert
}