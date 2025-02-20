

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

