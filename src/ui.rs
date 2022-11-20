use std::io;
use std::path::Path;

use crate::cmd_function;
use crate::cmd_interface::{createonly, readwrite, readonly};
use crate::cmd_interface::readwrite::RevDiff;

pub fn receive_input_command_test() ->io::Result<()>{
    let mut buffer=String::new();
    let stdin=io::stdin();
    stdin.read_line(&mut buffer);
    println!("input {}",buffer);
    let mut command: Command<&str> = Command{path: "111",command_input: "input_test.clone()" };
    //self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
    UserInterface::match_command(Command{path: command.path,command_input: "input_test.clone()" });
    Ok(())
}
//type input=fn()->String;
pub struct Command<T>{
    path: T,
    command_input:&'static str
}
pub(crate) struct UserInterface<T>{
    commands: Vec<Command<T>>
}

impl<T: Clone> UserInterface<T> {
    fn new()-> Self{
        Self{commands:vec![]}
    }
    pub fn receive_input_command(&mut self,input_test: T) ->io::Result<()>{
        let mut buffer=String::new();
        let stdin=io::stdin();
        stdin.read_line(&mut buffer);
        println!("input {}",buffer);
        self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        Self::match_command(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        Ok(())
    }

    pub fn receive_input_command_test_inside() ->io::Result<()>{
        let mut buffer=String::new();
        let stdin=io::stdin();
        stdin.read_line(&mut buffer);
        println!("input {}",buffer);
        let mut command: Command<&str> = Command{path: "111",command_input: "input_test.clone()" };
        //self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        UserInterface::match_command(Command{path: command.path,command_input: "input_test.clone()" });
        Ok(())
    }

    fn match_command(input:Command<T>){//old:->String, new no return
        //input.path
        let mut res:Result<&str,&str>=Err("1");
        let mut res_diff:Result<RevDiff,&str>=Err("1");
        match input.command_input{
            "add" => {res=crate::cmd_interface::readwrite::add("input.path");}//1
            "remove" => {res=crate::cmd_interface::readwrite::remove("input.path");}//2
            "commit" => {res=crate::cmd_interface::readwrite::commit("input.path");}//3
            "merge" => {res=crate::cmd_interface::readwrite::merge("input.path");}//4
            "diff" => {res_diff=crate::cmd_interface::readwrite::diff("input.path","input.path");}//5
            "cat" => {res=crate::cmd_interface::readwrite::cat("input.path","input.path");}//6
            "status" => {res=crate::cmd_interface::readonly::status("input.path");}//status1
            "log" => {res=crate::cmd_interface::readonly::log("input.path");}//log2
            "heads" => {res=crate::cmd_interface::readonly::heads("input.path");}//heads3
            "clone" => {res=crate::cmd_interface::createonly::clone("input.path","input.path");}//1
            "checkout" => {res=crate::cmd_interface::createonly::checkout("input.path","input.path");}//2
            "pull" => {res=crate::cmd_interface::createonly::pull("input.path","input.path",Some("input.path"));}//3
            "push" => {res=crate::cmd_interface::createonly::push("input.path","input.path",Some("input.path"));}//4
            "init" => {res=Ok("init here")}//1
            _ => {}
        }
        if res!=Err("1")
        {
            Self::input_handling(res);
        }
        else {
            Self::input_handling_special(res_diff);
        }
        //unimplemented!();
    }

    fn input_handling(return_result:Result<&str,&str>){
        println!("{:?}",return_result)
    }

    fn input_handling_special(return_result:Result<RevDiff,&str>){
        //unimplemented!();
        println!("{:?}","return_result")
    }

    fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }
}

