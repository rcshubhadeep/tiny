use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;


pub fn start_main_loop(){
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes(){
        match b {
            Ok(b) => {
                let c = b as char;
                if crate::ioutils::is_it_a_control_char(c){
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if crate::ioutils::is_it_ctrl_q(b){
                    break;
                }
            }
            Err(err) => crate::errors::die(err),
        }
    }
}