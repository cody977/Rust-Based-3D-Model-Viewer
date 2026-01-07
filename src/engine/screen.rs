

pub struct Screen {
    pub width: i32,
    pub height: i32,
    pub is_paused: bool,
    pub title: String,
}

impl Screen {
    pub fn default() -> Screen {
        let width = 1200;
        let height = 900;
        let is_paused = false;
        let title = String::from("Window");

        Screen { width, height, is_paused, title }
    }


    pub fn new(width: i32, height: i32, title: String) -> Screen {
        Screen {
            width,
            height,
            is_paused: false,
            title
        }
    }
}