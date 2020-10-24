pub fn is_it_a_control_char(c: char)-> bool {
    return c.is_control();
}

pub fn is_it_ctrl_q(b: u8) -> bool{
    if b == 17{
        return true;
    } 
    return false;
}