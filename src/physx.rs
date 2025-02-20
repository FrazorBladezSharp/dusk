pub mod aabb_collision;



/* object inside box collision
        : where the box can be the application window

    function(object, target) -> (f32, f32) {... (x, y)}


    left side detected x = -1       object.x <= target.x
    right side detected x = 1       object.x >= target.x
    top side detected y = -1        object.y <= target.y
    btm side detected y = 1         object.y >= target.y

    (0, 0) = no collision

    useage:
        let (x, y) = physx::function(object.rectangle, target.rectangle);

    Note:
        it maybe wise to detect collisions 1 frame ahead as objects landing
        fully outside the target may get stuck when using delta time animation

*/

