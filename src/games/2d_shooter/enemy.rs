use raylib::color::Color;
use raylib::math::Vector2;
use crate::engine::screen::Screen;

#[derive(Clone, Copy)] // Allows copying the enum
pub enum EnemyType {
    Normal,
    Fast,
    Tank
}

pub struct Enemy {
    pub position: Vector2,
    pub speed: f32,
    pub color: Color,
    pub size: Vector2,
    pub health: f32,
    pub max_health: f32,
    pub alive: bool,
    pub enemy_type: EnemyType,
    pub exp_points: f32
}

impl Default for Enemy {
    fn default() -> Enemy {
        Enemy::new(EnemyType::Normal, 1.0)  // Default to Normal type with x1 difficulty multiplier
    }
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, difficulty_multiplier: f32) -> Enemy {
        let screen = Screen::default();

        let (speed, color, size, base_health, exp_points) = match enemy_type {
            EnemyType::Normal => (50.0, Color::BLUE, Vector2::new(40.0, 40.0), 100.0, 10.0),
            EnemyType::Fast => (100.0, Color::GREEN, Vector2::new(20.0, 20.0), 25.0, 5.0),
            EnemyType::Tank => (25.0, Color::RED, Vector2::new(60.0, 80.0), 200.0, 15.0),
        };

        //Scale health by difficulty multiplier
        let health = base_health * difficulty_multiplier;

        Enemy {
            position: Vector2::new (screen.width as f32 / 2.0, screen.height as f32 / 2.0),
            speed,
            color,
            size,
            health,
            max_health: health,
            alive: true,
            enemy_type,
            exp_points,
        }
    }
}

impl Enemy {
    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health <= 0.0 {
            self.alive = false;
        }
    }
}