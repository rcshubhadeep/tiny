use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
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
            terminal: Terminal::default().expect("There is an error to initialize Tiny"),
            cursor_position: Position {x: 0, y: 0},
         }
    }

    fn refresh_screen(&self) -> Result <(), std::io::Error> {
        Terminal::cusrsor_position(&Position{x: 0, y: 0});
        Terminal::cursor_hide();
        if self.should_quit {
            Terminal::clear_screen();
            print!("Goodbye!\r");
        } else {
            self.darw_row_nums();
            Terminal::cusrsor_position(&self.cursor_position);
        }
        Terminal::cusrsor_show();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut message = format!("Tiny Editor Version - {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{}{}", spaces, message);
        message.truncate(width);
        println!("{}\r", message);
    }

    fn darw_row_nums(&self) {
        let height = self.terminal.size().height;
        for row in 0 .. height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
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