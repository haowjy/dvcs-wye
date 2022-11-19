use std::io;
use std::path::Path;

use crate::cmd_function;
use crate::cmd_interface::{createonly, read_write};

//type input=fn()->String;
struct Command<T>{
    path: T,
    command_input:&'static str
}
struct UserInterface<T>{
    commands: Vec<Command<T>>
}

impl<T: Clone> UserInterface<T> {
    fn new()-> Self{
        Self{commands:vec![]}
    }
    pub fn receive_input_command(&mut self,input_test: T) ->io::Result<()>{
        self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        Self::match_command(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        Ok(())
    }

    pub fn match_command(input:Command<T>)->String{
        //input.path
        let mut res:Result<&str,&str>=Err("1");
        match input.command_input{
            "add" => {res=crate::cmd_interface::read_write::add("input.path");}//1
            "remove" => {res=crate::cmd_interface::read_write::remove("input.path");}//2
            "commit" => {res=crate::cmd_interface::read_write::commit("input.path");}//3
            "merge" => {res=crate::cmd_interface::read_write::merge("input.path");}//4
            //"diff" => {res=crate::cmd_interface::read_write::diff(Some("input.path"),"input.path");}//5
            "cat" => {res=crate::cmd_interface::read_write::cat(Some("input.path"),"input.path");}//6
            "status" => {res=crate::cmd_interface::read_write::add("input.path");}//status
            "log" => {res=crate::cmd_interface::read_write::commit("input.path");}//log
            "heads" => {res=crate::cmd_interface::read_write::merge("input.path");}//heads
            "clone" => {res=crate::cmd_interface::createonly::clone("input.path","input.path");}
            _ => {}
        }
        Self::input_handling(res);
        unimplemented!();
    }

    fn input_handling(return_result:Result<&str,&str>){
        println!("{:?}",return_result)
    }

    fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }
}

