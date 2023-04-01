use mouse_rs::{types::keys::Keys, Mouse};
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

extern crate stopwatch;

use ctrlc;
use stopwatch::{Stopwatch};
use mouse_rs::types::Point;


#[derive(PartialEq, Eq)]
enum Phase {
    WaitForMove = 1,
    IdleAfterMove,
    CircleAnimation,
}

struct ProgramState {
    mm: DeviceState,
    mouse: Mouse,
    state: Phase,
    last_mouse_pos: (i32, i32),
    wait_before_click: i32,
    idle_time: i32,
    timer_click: Stopwatch,
    timer_idle: Stopwatch,

}

trait Updatable {
    fn update(&mut self);
}

fn clear()
{
    if cfg!(windows) {
        std::process::Command::new("cls").status().unwrap();
        println!("this is windows");
    } else if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    }
}

impl Updatable for ProgramState {
    fn update(&mut self) {
// println!("hello there!");
        let pos = self.mm.get_mouse().coords;
        println!("{} - {}", pos.0,pos.1);

        if !(pos.0 == self.last_mouse_pos.0 && pos.1 == self.last_mouse_pos.1)
        {
            println!("hello there! 2");
            self.timer_idle.restart();
            self.last_mouse_pos = pos;
            self.state = Phase::IdleAfterMove;
        } else {
            if self.state == Phase::IdleAfterMove
            {
                println!("a");
                if self.timer_idle.elapsed_ms() > self.idle_time as i64
                {
                     println!("b");
                    self.timer_click.restart();
                    self.state = Phase::CircleAnimation;
                }
            } else if self.state == Phase::CircleAnimation
            {
                println!("c");
                if self.timer_click.elapsed_ms() > self.wait_before_click as i64
                {
                    clear();
                     println!("d");
                    self.state = Phase::WaitForMove;
// println!("hello there!");
                    // mouse.move_to(500, 500).expect("Unable to move mouse");
                    self.mouse.press(&Keys::LEFT).expect("Unable to press button");
                    self.mouse.release(&Keys::LEFT).expect("Unable to release button");
                     println!("2");
                }
            }
        }
    }
}


fn main() {
    let mut run = true;


    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        run = false;
    })
        .expect("Error setting Ctrl-C handler");
    println!("hello there!");
    let device_state = DeviceState::new();
    let mut config = ProgramState {
        mm: device_state,
        state: Phase::WaitForMove,
        wait_before_click: 1200,
        idle_time: 200,
        last_mouse_pos: (0, 0),
        mouse: Mouse::new(),
        timer_click: Stopwatch::new(),
        timer_idle: Stopwatch::new(),

    };
    config.timer_idle.restart();
    config.timer_click.restart();
    while run {
        config.update();
    }
}
