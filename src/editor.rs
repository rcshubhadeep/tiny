use std::io::{self, stdout};
use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor{

}

impl Editor {

    pub fn run(&self){
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                },
                Err(err) => crate::errors::die(err),
            }
        }
    }

    pub fn default() -> Self {
        Self{}
    }
}


pub fn start_main_loop(){
    let _stdout: RawTerminal<io::Stdout> = stdout().into_raw_mode().unwrap();
    let editor: Editor = Editor::default();
    editor.run();
}