use std::io;
use std::path::Path;

use crate::cmd_function;
use crate::cmd_interface::{createonly, read_write};

pub fn receive_input_command(input_test: &str) ->io::Result<()>{
    Ok(())

}

fn match_command<'a>(wd:&'a str,input:Vec<&'a str>)->Vec<&'a str>{
    input
}

fn input_handling<E: std::fmt::Debug>(return_result:Result<(), E>){
    println!("{:?}",return_result)
}
