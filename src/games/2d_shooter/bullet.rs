use raylib::color::Color;
use raylib::math::Vector2;

pub struct Bullet {
    pub position: Vector2,
    pub direction: Vector2,
    pub speed: f32,
    pub color: Color,
    pub size: f32,
    pub damage: f32,
    pub active: bool,
}

impl Bullet {
    pub fn new(start_pos: Vector2, target_pos: Vector2, speed: f32) -> Bullet {
        //calculate direction from player to mouse
        let dist_x = target_pos.x - start_pos.x;
        let dist_y = target_pos.y - start_pos.y;
        let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

        // Normalize direction (handle zero distance)
        let direction = if distance > 0.0 {
            Vector2::new(dist_x / distance, dist_y / distance)
        } else {
            Vector2::new(0.0, 0.0)
        };

        Bullet {
            position: start_pos,
            direction,
            speed,
            color: Color::BLACK,
            size: 5.0,
            damage: 50.0,
            active: true
        }
    }

    pub fn on_collision(&self, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
        //Find the closest point on rectangle to circle center
        let closest_x = self.position.x.max(rect_x).min(rect_x + rect_width);
        let closest_y = self.position.y.max(rect_y).min(rect_y + rect_height);

        //Calculate distance between bullet center and closet point
        let dist_x = self.position.x - closest_x;
        let dist_y = self.position.y - closest_y;
        let distance_squared = dist_x * dist_x + dist_y * dist_y;

        //Collision if distance is less than bullet radius
        distance_squared < (self.size * self.size)
    }
}