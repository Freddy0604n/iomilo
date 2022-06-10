use sdl2::pixels::Color;

pub struct Rectangle {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub color: Color,
    pub rescale: bool, // set the widht and height relative to the window width and heigth (promilles)
}

impl Rectangle {
    pub fn new(
        position: (i32, i32),
        size: (u32, u32),
        color: (u8, u8, u8),
        rescale: bool,
    ) -> Rectangle {
        Rectangle {
            position,
            size,
            color: Color::RGB(color.0, color.1, color.2),
            rescale,
        }
    }

    pub fn update_color(&mut self, new_color: (u8, u8, u8)) {
        self.color = Color::RGB(new_color.0, new_color.1, new_color.2);
    }

    pub fn update_position(&mut self, new_position: (i32, i32)) {
        self.position = new_position.clone();
    }

    pub fn update_size(&mut self, new_size: (u32, u32)) {
        self.size = new_size.clone();
    }

    pub fn update_all(
        &mut self,
        new_position: (i32, i32),
        new_size: (u32, u32),
        new_color: (u8, u8, u8),
    ) {
        self.position = new_position.clone();
        self.size = new_size.clone();
        self.color = Color::RGB(new_color.0, new_color.1, new_color.2);
    }
}

pub struct Window {
    pub title: String,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub bg_color: Color,
    pub bar_color: Color,
    pub rectangles: Vec<Rectangle>,
    pub clicks: Vec<ClickArea>,
}

impl Window {
    pub fn new(
        title: String,
        position: (i32, i32),
        size: (u32, u32),
        bg_color: (u8, u8, u8),
        bar_color: (u8, u8, u8),
    ) -> Window {
        Window {
            title,
            position,
            size,
            bg_color: Color::RGB(bg_color.0, bg_color.1, bg_color.2),
            bar_color: Color::RGB(bar_color.0, bar_color.1, bar_color.2),
            rectangles: Vec::new(),
            clicks: Vec::new(),
        }
    }
}

pub struct Sprite {
    pub buffer: Vec<(u8, u8, u8)>,
    pub position: (i32, i32),
    pub size: (i32, i32),
}

impl Sprite {
    pub fn new(buffer: Vec<(u8, u8, u8)>, position: (i32, i32), size: (u32, u32)) -> Sprite {
        Sprite {
            buffer,
            position,
            size: (size.0 as i32, size.1 as i32), // to prevent negative number input but allow sdl2 to understand it
        }
    }
}

pub struct ClickArea {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub clickaction_id: u32, // the number that is returned on click
}

impl ClickArea {
    pub fn new(position: (i32, i32), size: (u32, u32), clickaction_id: u32) -> ClickArea {
        ClickArea {
            position,
            size,
            clickaction_id,
        }
    }
}
