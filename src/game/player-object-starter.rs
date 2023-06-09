pub trait Drawable {
    fn draw(&mut self, draw: &mut Draw, position: (f32, f32));
}

struct Player {
    x: f32,
    y: f32,

    width: f32,
    height: f32,

    color: Color,

    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,

            width: 0.0,
            height: 0.0,

            color: Color::BLUE,

            speed: 4.0,
        }
    }

    pub fn with_values(x: f32, y: f32, width: f32, height: f32, color: Color, speed: f32) -> Self {
        Self {
            x,
            y,

            width,
            height,

            color,

            speed,
        }
    }

    pub fn x(&mut self) -> f32 { self.x }
    pub fn y(&mut self) -> f32 { self.y }

    pub fn width(&mut self) -> f32 { self.width }
    pub fn height(&mut self) -> f32 { self.height }

    pub fn color(&mut self) -> Color { self.color }

    pub fn speed(&mut self) -> f32 { self.height }
}