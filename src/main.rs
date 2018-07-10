extern crate piston_window;

use piston_window::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");

    let mut window: PistonWindow = WindowSettings::new("Hello World!", [800, 400])
        .exit_on_esc(true).build().unwrap();

    let mut s1 = Ship::new(50., 50., 10., 100.);
    let mut s2 = Ship::new(250., 250., 40., 100.);
    let shots: Arc<Mutex<Vec<Shot>>> = Arc::new(Mutex::new(Vec::new()));

    s1.controls.insert(Key::W, Command::JumpF);
    s1.controls.insert(Key::A, Command::JumpL);
    s1.controls.insert(Key::S, Command::JumpB);
    s1.controls.insert(Key::D, Command::JumpR);
    s1.controls.insert(Key::Q, Command::RotL);
    s1.controls.insert(Key::E, Command::RotR);
    s1.controls.insert(Key::Space, Command::Shoot);

    s2.controls.insert(Key::I, Command::JumpF);
    s2.controls.insert(Key::J, Command::JumpL);
    s2.controls.insert(Key::K, Command::JumpB);
    s2.controls.insert(Key::L, Command::JumpR);
    s2.controls.insert(Key::U, Command::RotL);
    s2.controls.insert(Key::O, Command::RotR);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.1, 0.2, 0.1, 0.5], g);
            rectangle([1., 0., 0., 1.], [0., 0., 40., 40.], c.transform.trans(s1.x, s1.y).rot_deg(s1.rot).trans(-20., -20.), g);
            rectangle([0., 0., 1., 1.], [0., 0., 40., 40.], c.transform.trans(s2.x, s2.y).rot_deg(s2.rot).trans(-20., -20.), g);
            line([0., 1., 1., 1.], 3., [s1.x, s1.y, s1.x + 50.*s1.rot.to_radians().cos(), s1.y + 50.*s1.rot.to_radians().sin()], c.transform, g);
            line([0., 1., 1., 1.], 3., [s2.x, s2.y, s2.x + 50.*s2.rot.to_radians().cos(), s2.y + 50.*s2.rot.to_radians().sin()], c.transform, g);

            for shot in shots.lock().unwrap().iter_mut() {
                rectangle([0.6, 0.6, 0.6, 1.], [0., 0., 5., 5.], c.transform.trans(shot.x, shot.y), g);
                shot.x += shot.dx;
                shot.y += shot.dy;
            }
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("{:?}", key);
            if let Some(command) = s1.parse_key(key){
                s1.command(command);
            }

            if let Some(command) = s2.parse_key(key){
                s2.command(command);
            }

            if s1.shooting {
                s1.shooting = false;
                let dx = 2.*s1.rot.to_radians().cos();
                let dy = 2.*s1.rot.to_radians().sin();
                shots.lock().unwrap().push( Shot{ x: s1.x + 10.*dx, y: s1.y + 10.*dy, dx, dy });
            }
        }
    }
}

struct Ship{
    x: f64,
    y: f64,
    rot: f64,
    health: f64,
    controls: HashMap<Key, Command>,
    shooting: bool,
}

#[derive(Clone)]
struct Shot{
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

#[derive(Clone)]
enum Command{
    JumpF,
    JumpR,
    JumpL,
    JumpB,
    RotR,
    RotL,
    Shoot,
}

impl Ship{
    fn new(x: f64, y: f64, rot: f64, health: f64) -> Self{
        let controls = HashMap::new();
        Self{ x, y, rot, health, controls, shooting: false}
    }

    fn parse_key(&self, key: Key) -> Option<Command> {
        if let Some(c) = self.controls.get(&key){
            return Some(c.to_owned());
        }
        None
    }

    fn command(&mut self, command: Command){
        match command {
            Command::JumpF => self.jump(0.),
            Command::JumpB => self.jump(-180.),
            Command::JumpR => self.jump(90.),
            Command::JumpL => self.jump(-90.),
            Command::RotR => self.rot(13.),
            Command::RotL => self.rot(-13.),
            Command::Shoot => self.shoot(),
        }
    }

    fn rot(&mut self, deg: f64){
        self.rot += deg;
    }

    fn jump(&mut self, temprot: f64){
        let rot = self.rot + temprot;
        self.x += 25.*rot.to_radians().cos();
        self.y += 25.*rot.to_radians().sin();

        let clamped = self.x.max(0.).min(800.);
        self.x = clamped;
        let clamped = self.y.max(0.).min(400.);
        self.y = clamped;
    }

    fn shoot(&mut self){
        self.shooting = true;
    }
}

