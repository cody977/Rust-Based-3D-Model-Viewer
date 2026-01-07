use raylib::prelude::*;


pub struct Button  {
    rect: Rectangle,
    pub  text: String,
    pub color: Color,
    pub  text_color: Color,
    pub  hover_color: Color,
    pub  is_hovered: bool,
    pub   is_clicked: bool,
    pub  is_enabled: bool,
}

impl Button {
   pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str) -> Self {
        Button {
            rect: Rectangle::new(x, y, width, height),
            text: text.to_string(),
            color: Color::DARKGRAY,
            hover_color: Color::GRAY,
            text_color: Color::WHITE,
            is_hovered: false,
            is_clicked: false,
            is_enabled: true,

        }
    }

    pub  fn is_hovered(&self, mouse_pos: Vector2) -> bool {
        self.rect.check_collision_point_rec(mouse_pos)
    }

    pub  fn is_clicked(&self, mouse_pos: Vector2, rl: &RaylibHandle) -> bool {
        self.is_hovered(mouse_pos) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle, mouse_pos: Vector2) {
        let color = if self.is_hovered(mouse_pos) {
            self.hover_color
        } else {
            self.color
        };

        rl.draw_rectangle_rec(self.rect, color);
        rl.draw_rectangle_lines_ex(self.rect, 2 as f32, Color::BLACK);

        let text_width = rl.measure_text(&self.text, 20);
        let text_x = self.rect.x + (self.rect.width - text_width as f32) / 2.0;
        let text_y = self.rect.y + (self.rect.height - 20.0) / 2.0;

        rl.draw_text(&self.text, text_x as i32, text_y as i32, 20, self.text_color);
    }
}