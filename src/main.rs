use std::path::{Path, PathBuf};
use raylib::prelude::*;
use rfd::FileDialog;

mod engine;

use engine::screen::*;
use crate::engine::input::keyboard::*;

fn main() {
    let mut screen = Screen::default();
    let mut input = InputState::new();
    let mut should_reload_model = false;

    let (mut rl, thread) = raylib::init()
        .size(screen.width, screen.height)
        .title("Inspect3D- 3D Viewer Powered By Raylib Using Rust")
        .vsync()
        .build();
    rl.set_target_fps(60);

    let editor_camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let mut model_path = PathBuf::from("./src/assets/funko/funko.obj");
    let mut file_name = model_path.file_name().unwrap().to_str().unwrap().to_string();
    println!("Loading model: {}", file_name);

    let mut model = rl.load_model(&thread, model_path.to_str().unwrap()).expect("Couldn't load model");
    let mut model_rotation = 0.0;
    let mut model_scale = Vector3::new(1.0, 1.0, 1.0);

    // LOAD FONT ONCE, OUTSIDE THE LOOP
    let custom_font = rl.load_font(&thread, "./src/engine/fonts/JetBrainsMono.ttf").unwrap();

    while !rl.window_should_close() {
        input.update();
        let mouse_pos = rl.get_mouse_position();
        let is_held = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

        {
            let mut win = rl.begin_drawing(&thread);
            win.clear_background(Color {
                r: 10,
                g: 10,
                b: 10,
                a: 255,
            });

            // 3D drawing
            {
                let mut d3 = win.begin_mode3D(editor_camera);
                d3.draw_grid(15, 1.0);

                d3.draw_model_ex(&model,
                                 Vector3::new(0.0, 0.0, 0.0),  // Center at origin
                                 Vector3::new(0.0, 1.0, 0.0),
                                 model_rotation,
                                 model_scale,
                                 Color::WHITE,
                );
            }

            // 2D drawing
            let text_label_file_name = format!("File: {}", file_name);
            let text_label_rotate = "A/D: Rotate".to_string();
            let text_label_scale = "W/S: Scale".to_string();

            win.draw_text_ex(&custom_font, text_label_file_name.as_str(), Vector2 { x: 12.0, y: 10.0 }, 24.0, 0.5, Color::WHITE);
            win.draw_text_ex(&custom_font, text_label_rotate.as_str(), Vector2 { x: 12.0, y: 70.0 }, 24.0, 0.5, Color::WHITE);
            win.draw_text_ex(&custom_font, text_label_scale.as_str(), Vector2 { x: 12.0, y: 100.0 }, 24.0, 0.5, Color::WHITE);

            win.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 24);

            let button_rotate_left = Rectangle {
                x: 350.0,
                y: 800.0,
                width: 200.0,
                height: 50.0
            };

            let button_rotate_right = Rectangle {
                x: 650.0,
                y: 800.0,
                width: 200.0,
                height: 50.0
            };

            let button_size_increase = Rectangle {
                x: 900.0,
                y: 800.0,
                width: 50.0,
                height: 50.0
            };

            let button_size_decrease = Rectangle {
                x: 250.0,
                y: 800.0,
                width: 50.0,
                height: 50.0
            };

            let button_select_file = Rectangle {
                x: 100.0,
                y: 800.0,
                width: 100.0,
                height: 50.0
            };

            if win.gui_button(button_rotate_left, "Rotate Left")
                || (button_rotate_left.check_collision_point_rec(mouse_pos) && is_held) {
                model_rotation -= 1.5;
            }

            if win.gui_button(button_rotate_right, "Rotate Right")
                || (button_rotate_right.check_collision_point_rec(mouse_pos) && is_held) {
                model_rotation += 1.5;
            }

            if win.gui_button(button_size_increase, "+")
                || (button_size_increase.check_collision_point_rec(mouse_pos) && is_held) {
                model_scale += 0.01;
            }

            if win.gui_button(button_size_decrease, "-")
                || (button_size_decrease.check_collision_point_rec(mouse_pos) && is_held) {
                model_scale -= 0.01;
            }

            //if win.gui_button(button_select_file, "Select File") {
             //   should_reload_model = true;
           // }
        } // Drawing scope ends here, `win` is dropped and `rl` is released

        // Handle model reloading OUTSIDE the drawing scope
        if should_reload_model {
            if let Some(new_path) = select_file() {
                println!("Selected path: {:?}", new_path);
                println!("Path exists: {}", new_path.exists());

                match rl.load_model(&thread, new_path.to_str().unwrap()) {
                    Ok(loaded_model) => {
                        model = loaded_model;
                        model_path = new_path;

                        if let Some(name) = model_path.file_name() {
                            if let Some(name_str) = name.to_str() {
                                file_name = name_str.to_string();
                                println!("Successfully loaded: {}", file_name);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to load model: {}", e);
                        println!("The OBJ file may have polygonal faces with >3 vertices.");
                        println!("Try triangulating it in Blender before loading.");
                    }
                }
            }
            should_reload_model = false;
        }

        // Input handling (outside drawing scope)
        if input.is_down(VK_W) {
            model_scale += Vector3::new(0.01, 0.01, 0.01);
        }

        if input.is_down(VK_A) {
            model_rotation -= 1.5;
        }

        if input.is_down(VK_S) {
            model_scale -= Vector3::new(0.01, 0.01, 0.01);
        }

        if input.is_down(VK_D) {
            model_rotation += 1.5;
        }

        if input.is_down(VK_P) {
            screen.is_paused = true;
            println!("{}", screen.is_paused);
        }
    }
}

//currently crashes if .obj is too big
fn select_file() -> Option<PathBuf> {
    let file_path = FileDialog::new()
        .set_title("Select File")
        .add_filter("obj", &["obj"])
        .set_directory("/")
        .pick_file();

    if let Some(path) = file_path {
        println!("Selected file: {:?}", path);
        Some(path)
    } else {
        println!("No file selected");
        None
    }
}