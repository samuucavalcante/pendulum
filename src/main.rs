use speedy2d::color::Color;
use vector::Vector;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{ Graphics2D, Window };

fn main() {
    let window: Window = Window::new_centered("Pendulum", (800, 400)).unwrap();

    let win_handler = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
    };

    window.run_loop(win_handler);
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        self.p.update();
        self.p.draw(graphics);

        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector,
    position: Vector,

    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32,
    m: f32,
    g: f32
}

impl Pendulum {
    fn new(x: f32, y: f32, r:f32) -> Pendulum {
        Pendulum { 
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r: r,
            m: 1.0,
            g: 1.5 
        }
    }

    fn update(&mut self) {
        self.angular_acceleration = -self.g * self.angle.sin() / self.r;
    
        self.angular_velocity += self.angular_acceleration;
    
        self.angle += self.angular_velocity;
    
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());
        
        self.position.add(&self.origin); // Passando uma referência para origin
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line((self.origin.x, self.origin.y), (self.position.x, self.position.y), 3.0, Color::RED);

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLACK);
    }
}

mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector {x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &mut Vector {
            self.x += other.x;  
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector {
            self.x = x;
            self.y = y;
            self
        }
    }
}
