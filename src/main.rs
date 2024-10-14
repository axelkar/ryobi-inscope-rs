use std::{
    io::{stdin, stdout, Read, Write},
    os::fd::AsRawFd,
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    time::Duration,
};

use owo_colors::{OwoColorize, XtermColors};
use extraproto::query_battery_level_sync;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

pub mod extraproto;

struct TerminalManager {
    original_termios: Termios,
}
impl TerminalManager {
    fn new() -> std::io::Result<Self> {
        let stdin = stdin().as_raw_fd();
        let original_termios = Termios::from_fd(stdin)?;

        let mut new_termios = original_termios;
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(stdin, TCSANOW, &new_termios).unwrap();

        Ok(Self { original_termios })
    }
}
impl Drop for TerminalManager {
    fn drop(&mut self) {
        tcsetattr(stdin().as_raw_fd(), TCSANOW, &self.original_termios).unwrap();
    }
}

struct ChildProcessDropper {
    child: Child,
}
impl Drop for ChildProcessDropper {
    fn drop(&mut self) {
        self.child.kill().ok();
    }
}

#[derive(Clone, Copy)]
struct State {
    battery_level: u8,
    light_level: u8,
}

fn main() -> std::io::Result<()> {
    let _terminal_manager = TerminalManager::new()?;
    let _child_manager = ChildProcessDropper {
        child: Command::new("ffplay")
            .args([
                "-fflags",
                "nobuffer",
                "-flags",
                "low_delay",
                "-rtsp_transport",
                "tcp",
                "rtsp://192.168.0.1",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    };
    let state = Arc::new(Mutex::new(State {
        battery_level: query_battery_level_sync()?,
        light_level: 0,
    }));
    extraproto::set_light_level_sync(0)?;
    render(*state.lock().unwrap());

    let new_state = state.clone();
    std::thread::spawn(move || {
        loop {
        std::thread::sleep(Duration::from_secs(10));
        let mut state = new_state.lock().unwrap();
        let old_state = *state;

        state.battery_level = query_battery_level_sync().unwrap();
        if state.battery_level != old_state.battery_level {
            print!("\x1B[4A");
            stdout().flush().unwrap();
            render(*state);
            stdout().flush().unwrap();
        }
    }
    });

    loop {
        let mut buf = [0];
        stdin().read_exact(&mut buf)?;
        let ch = buf[0];
        let mut state = state.lock().unwrap();
        let old_state = *state;

        match ch {
            b'q' => {
                return Ok(())
            },
            b'0' => state.light_level = 0,
            b'1' => state.light_level = 1,
            b'2' => state.light_level = 2,
            b'3' => state.light_level = 3,
            b'\x1b' => {
                let mut buf = [0; 2];
                stdin().read_exact(&mut buf)?;
                match &buf {
                    b"[A" => {
                        // Arrow up
                        state.light_level = (state.light_level + 1).clamp(0, 3);
                    }
                    b"[B" => {
                        // Arrow down
                        state.light_level = (state.light_level.saturating_sub(1)).clamp(0, 3);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if state.light_level != old_state.light_level {
            extraproto::set_light_level_sync(state.light_level)?;
            print!("\x1B[4A");
            stdout().flush().unwrap();
            render(*state);
            stdout().flush().unwrap();
        }
    }
}

fn render(state: State) {
    let light_color = |min_level| {
        if state.light_level >= min_level {
            XtermColors::White
        } else {
            XtermColors::DarkGray
        }
    };
    println!(
        "{}\n{}\n{}",
        "🭦██🭛".color(light_color(3)),
        " 🭖🭡".color(light_color(2)),
        " 🭦🭛".color(light_color(1))
    );
    println!(
        "{}",
        format_args!(
            "Battery: {} (officially {}%)",
            state.battery_level,
            match state.battery_level {
                100.. => 0.0,
                45.. => 99.99,
                22.. => 66.66,
                10.. => 33.33,
                _ => 0.0,
            }
        )
        .color(match state.battery_level {
            30.. => XtermColors::BrightGreen,
            22.. => XtermColors::Yellow,
            10.. => XtermColors::FlushOrange,
            _ => XtermColors::Red,
        })
    );
}
