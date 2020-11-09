use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("There is an error to start the terminal"),
         }
    }

    fn refresh_screen(&self) -> Result <(), std::io::Error> {
        Terminal::cusrsor_position(0, 0);
        Terminal::cursor_hide();
        if self.should_quit {
            Terminal::clear_screen();
            print!("Goodbye!\r");
        } else {
            self.darw_row_nums();
            Terminal::cusrsor_position(0, 0);
        }
        Terminal::cusrsor_show();
        Terminal::flush()
    }

    fn darw_row_nums(&self) {
        let height = self.terminal.size().height;
        for row in 0 .. height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                println!("Tiny editor -- version {}\r", VERSION)
            } else {
                println!("~\r");
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!(e);
}