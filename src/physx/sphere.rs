


#[derive(Default)]
pub struct Sphere {
    x: f32,
    y: f32,
    r: f32,
}

impl Sphere {
    pub fn new(position_x: f32, position_y: f32, radius: f32) -> Self {
        Sphere {
            x: position_x,
            y: position_y,
            r: radius,
        }
    }

    pub fn sphere_detect_collision(&self, target: &Sphere) -> f32 {
        // variables
        let mut collision: f32 = 1.0;

        let dist_squared = (self.x - target.x).powi(2) + 
            (self.y - target.y).powi(2);

        // detect collision
        if (self.r + target.r).powi(2) <= dist_squared {
            // collision has occured
            collision = -1.0;
        }


        // return result to user
        collision
    }
}
