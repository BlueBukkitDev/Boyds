

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
    * Returns a `Point` that is "dist" distance away from the "origin" `Point` in direction "dir". 
    */
    pub fn get_dir_increment(&self, dir: f32, dist: f32) -> Point {
        let mut pos_x = self.x;
        let mut pos_y = self.y;
    
        if dir < 90.0 {
            pos_x += (dir/90.0)*dist;
            pos_y -= ((90.0-dir)/90.0)*dist;
        }else if dir < 180.0 {
            pos_x += ((180.0-dir)/90.0)*dist;
            pos_y += ((dir-90.0)/90.0)*dist;
        }else if dir < 270.0 {
            pos_x -= ((dir-180.0)/90.0)*dist;
            pos_y += ((270.0-dir)/90.0)*dist;
        }else if dir < 360.0 {
            pos_x -= ((360.0-dir)/90.0)*dist;
            pos_y -= ((dir-270.0)/90.0)*dist;
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