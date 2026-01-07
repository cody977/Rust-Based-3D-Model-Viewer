/*
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use rand::Rng;


mod games;
mod engine;


use engine::screen::*;
use crate::games::2d_shooter::bullet::*;
use crate::games::enemy::*;
use crate::games::player::*;
#[derive(Clone, Copy)]
enum Upgrades {
    Health,
    Damage,
    AttackSpeed
}


fn main() {
    let mut screen = Screen::default();
    let (mut rl, thread) = raylib::init()
        .size(screen.width, screen.height)
        .title("Window")
        .vsync()
        .build();
    rl.set_target_fps(60);


    let dt = rl.get_frame_time();
    let mut enemy_spawn_timer = 0.0;
    let enemy_spawn_delay = 1.5;
    let mut enemies: Vec<Enemy> = vec![Enemy::default()];
    let mut player = Player::default();
    let mut spawned_bullets: Vec<Bullet> = Vec::new();



    while !rl.window_should_close() {  //Update every frame
        let dt = rl.get_frame_time();
        player.update(dt);
        player.check_rank(&mut screen);
        player.time_alive += dt;
        player.mouse_position = rl.get_mouse_position();
        enemy_spawn_timer += dt;



        //Spawn enemies
        if enemy_spawn_timer >= enemy_spawn_delay {
            //Randomly choose enemy type
            let random_type = rl.get_random_value(0..3);
            let enemy_type = match random_type {
                0 => EnemyType::Normal,
                1 => EnemyType::Fast,
                _ => EnemyType::Tank,
            };

            let mut new_enemy = Enemy::new(enemy_type, player.difficulty_multiplier);

            // Spawn at random edge position
            let mut rng = rand::rng();
            let side = rl.get_random_value(0..4);
            match side {
                0 => new_enemy.position = Vector2::new(rng.random_range(0.0..screen.width as f32), 0.0), // Top
                1 => new_enemy.position = Vector2::new(screen.width as f32, rng.random_range(0.0..screen.height as f32)), // Right
                2 => new_enemy.position = Vector2::new(rng.random_range(0.0..screen.width as f32), screen.height as f32), // Bottom
                _ => new_enemy.position = Vector2::new(0.0, rng.random_range(0.0..screen.height as f32)), // Left
            }
            enemies.push(new_enemy);
            enemy_spawn_timer = 0.0;
        }


        //Player input
        if rl.is_key_down(KEY_A) {
            player.position.x -= player.speed;
        }
        if rl.is_key_down(KEY_D) {
            player.position.x += player.speed;
        }
        if rl.is_key_down(KEY_W) {
            player.position.y -= player.speed;
        }
        if rl.is_key_down(KEY_S) {
            player.position.y += player.speed;
        }

        if rl.is_key_down(KEY_SPACE) {
            player.rank += 1;
            player.difficulty_multiplier += 0.2;
        }

        if rl.is_key_pressed(KEY_SPACE) {
            screen.is_paused = true;
            println!("{}", screen.is_paused);
        }

        if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) && player.can_shoot {
            let new_bullet = Bullet::new(player.position, player.mouse_position, 10.0); //spawn bullet
            spawned_bullets.push(new_bullet); //add to vector
            player.shoot();
        }

        if (!screen.is_paused) {

            //move bullets once spawned
            for bullet in spawned_bullets.iter_mut() {
                bullet.position.x += bullet.direction.x * bullet.speed;
                bullet.position.y += bullet.direction.y * bullet.speed;

                //check bullet and enemy collision
                for enemy in enemies.iter_mut() {
                    if bullet.active && enemy.alive {
                        if bullet.on_collision(
                            enemy.position.x,
                            enemy.position.y,
                            enemy.size.x,
                            enemy.size.y,
                        ) {
                            enemy.take_damage(bullet.damage);
                            bullet.active = false;

                            if !enemy.alive {
                                player.enemies_killed += 1;
                                player.experience_points_cur += enemy.exp_points;
                            }
                        }
                    }
                }
            }

            // Remove bullets that are off-screen
            spawned_bullets.retain(|bullet| {
                bullet.active &&
                    bullet.position.x >= 0.0 && bullet.position.x <= screen.width as f32 &&
                    bullet.position.y >= 0.0 && bullet.position.y <= screen.height as f32
            });

            //Remove dead enemies
            enemies.retain(|enemy| enemy.alive);




            //Draw
            let mut win = rl.begin_drawing(&thread);
            let enemy_killed = format!("Enemies Killed: {}", player.enemies_killed);
            let time_alive = format!("Time Alive: {}", player.time_alive);
            let player_rank = format!("Rank: {}", player.rank);
            let player_exp = format!("Exp: {} / {}", player.experience_points_cur, player.experience_points_needed);



            win.clear_background(Color::WHITE);
            win.draw_text(enemy_killed.as_str(), 12, 10, 20, Color::BLACK);
            win.draw_text(time_alive.as_str(), 12, 40, 20, Color::BLACK);
            win.draw_text(player_rank.as_str(), 12, 70, 20, Color::BLACK);
            win.draw_text(player_exp.as_str(), 12, 100, 20, Color::BLACK);
            win.draw_circle_v(player.position, player.size, player.color);




            //Draw enemies
            for enemy in enemies.iter_mut() {
                win.draw_rectangle_v(enemy.position, enemy.size, enemy.color);

                //Chase player
                let dx = player.position.x - enemy.position.x;
                let dy = player.position.y - enemy.position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance > 0.0 {
                    enemy.position.x += (dx / distance) * enemy.speed * dt;
                    enemy.position.y += (dy / distance) * enemy.speed * dt;
                }

                //Draw health bar
                let health_percentage = enemy.health / enemy.max_health;
                win.draw_rectangle(
                    enemy.position.x as i32,
                    (enemy.position.y - 10.0) as i32,
                    (enemy.size.x * health_percentage) as i32,
                    5,
                    enemy.color,
                );
            }

            for bullet in &spawned_bullets {
                win.draw_circle_v(bullet.position, bullet.size, bullet.color);
            }


        }
    }



}
*/

