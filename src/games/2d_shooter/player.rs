use raylib::prelude::*;
use crate::engine::screen::*;

pub struct Player {
    pub position: Vector2,
    pub speed: f32,
    pub color: Color,
    pub size: f32,
    pub health: f32,
    pub can_shoot: bool,
    pub shoot_timer: f32,
    pub shoot_delay: f32,
    pub mouse_position: Vector2,
    pub enemies_killed: i32,
    pub time_alive: f32,
    pub experience_points_cur: f32,
    pub experience_points_needed: f32,
    pub rank: i32,
    pub difficulty_multiplier: f32,
}

impl Default for Player {
    fn default() -> Player {
        let screen = Screen::default();
        let position = Vector2::new(screen.width as f32 / 2.0, screen.height as f32 / 2.0);
        let speed = 10.0;
        let color = Color::MAROON;
        let size = 25.0;
        let health = 100.0;
        let can_shoot = true;
        let shoot_timer = 0.0;
        let shoot_delay = 0.5;
        let mouse_position = Vector2::new(0.0, 0.0);
        let enemies_killed = 0;
        let time_alive = 0.0;
        let experience_points_cur = 0.0;
        let experience_points_needed = 5.0;
        let rank = 1;
        let difficulty_multiplier = 1.0;


        Player { position, speed, color, size, health, can_shoot, shoot_timer, shoot_delay,
            mouse_position, enemies_killed, time_alive, experience_points_cur, experience_points_needed,
            rank, difficulty_multiplier }
    }
}

impl Player {
    pub fn update(&mut self, dt: f32) {
        if !self.can_shoot {
            self.shoot_timer += dt;
            if self.shoot_timer >= self.shoot_delay {
                self.can_shoot = true;
                self.shoot_timer = 0.0;
            }
        }
    }

    pub fn shoot(&mut self) {
        if self.can_shoot {
            self.can_shoot = false;
        }
    }

    pub fn check_rank(&mut self, screen: &mut Screen) -> bool {
        if self.experience_points_cur >= self.experience_points_needed {
            self.rank += 1; //increase rank
            self.experience_points_cur = 0.0; // reset current exp points
            self.experience_points_needed += 50.00; //increase exp points
            screen.is_paused = true;


            //Increase difficulty multiplier by 20%
            self.difficulty_multiplier += 0.2;
            return true; //ranked up
        }
        false
    }
}