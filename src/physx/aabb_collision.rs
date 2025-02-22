/* Axis Aligned Bounding Box Collision

    pub struct Boundingbox_aa{ ... }

    function(object.rectangle, target.rectangle) -> (f32, f32) {... (x, y)}

    (a.x + a.width) >= b.x      a.rhs v b.lhs
        &&
    a.x <= (b.x + b.width)      a.lhs v b.rhs
        &&
    (a.y + a.height) >= b.y     a.btm v b.top
        &&
    a.y <= (b.y + b.height)     a.top v b.btm


    what do we return to the user
        values must be with respect to the object.rectangle
        : a vector ? - giving the direction of collision
                       with (0, 0) = no collision.

    useage:
        let (x, y) = aabb_collision::function(object.rectangle, target.rectangle);

    Note:
        it maybe wise to detect collisions 1 frame ahead as objects landing
        fully inside the target may get stuck when using delta time animation
*/

#[derive(Default)]
pub struct AABoundingBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl AABoundingBox {
    pub fn new(position_x: f32, position_y: f32, width: f32, height: f32) -> Self {
        AABoundingBox {
            x: position_x,
            y: position_y,
            width,
            height,
        }
    }

    pub fn detect_collision(&self, target: &AABoundingBox) -> bool {
        // variables
        let mut collision = false;

        // detect collision
        if (self.x + self.width) >= target.x
            && self.x <= (target.x + target.width)
            && (self.y + self.height) >= target.y
            && self.y <= (target.y + target.height)
        {
            // collision has occured
            collision = true;
        }
        // return result to user
        collision
    }
}
