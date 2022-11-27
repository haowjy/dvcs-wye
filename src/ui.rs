use std::io;
use std::path::Path;

use crate::cmd_function;
use crate::dsr;
use crate::cmd_interface::{createonly, readwrite, readonly};
use crate::cmd_interface::readwrite::RevDiff;
use crate::cmd_function::FileDiff;
use crate::vc::repository::{Repo};
use std::io::{stdout, Write};
use log::{info, warn};
pub trait Log {
    fn log_for_dev(&self);
}
impl Log for FileDiff<'_> {
     fn log_for_dev(&self){
        info!(target: "a","Difference between File {} and {}", "a","b");
        //info!(target: "a","Difference between {} and {}", diff.patch(get));
    }
}
impl Log for Repo {
    fn log_for_dev(&self){
        info!(target: "a","Repo update {}", "a");
    }
}
impl Log for RevDiff {
    fn log_for_dev(&self){
        info!(target: "a","Difference between rev {} and {}", "a","b");
    }
}
impl Log for Command{
    fn log_for_dev(&self){
        info!(target: "a","{} update {}", "command line","b");
    }
}


//type input=fn()->String;
pub struct Command{
    path: String,
    command_input:String,
    temp: String
}
pub(crate) struct UserInterface{
    commands: Vec<Command>
}

impl UserInterface {
    fn new()-> Self{
        Self{commands:vec![]}
    }
    /*pub fn receive_input_command(&mut self,input_test: T) ->io::Result<()>{
        let mut buffer=String::new();
        let stdin=io::stdin();
        stdin.read_line(&mut buffer);
        println!("input {}",buffer);
        let path=dsr::get_wd_path();
        self.commands.push(Command{path: path.clone(),command_input: buffer.clone(), temp:input_test.clone() });
        Self::match_command(Command{path,command_input: buffer, temp: input_test.clone() });
        Ok(())
    }*/
    pub fn receive_input_command_loop() ->io::Result<()>{//start here temporary
        print!("input q to quit");
        loop {
            print!("dvcs command>: ");
            stdout().flush().unwrap();
            let mut buffer=String::new();
            let stdin=io::stdin();
            stdin.read_line(&mut buffer);
            if buffer=="q\r\n".to_string() ||buffer=="q".to_string() ||buffer=="q\n".to_string(){
                println!("quit!");
                break;
            }
            println!("input {}",buffer);
            let path=dsr::get_wd_path();
            println!("path {}",path);
            let mut command: Command = Command{path,command_input: buffer, temp: "111".parse().unwrap() };
            //self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
            UserInterface::match_command(Command{path: command.path,command_input: command.command_input, temp: "111".parse().unwrap() });
        }
        Ok(())
    }

    pub fn receive_input_command_test_inside() ->io::Result<()>{//start here temporary
        let mut buffer=String::new();
        let stdin=io::stdin();
        stdin.read_line(&mut buffer);
        println!("input {}",buffer);
        let path=dsr::get_wd_path();
        println!("path {}",path);
        let mut command: Command = Command{path,command_input: buffer, temp: "111".parse().unwrap() };
        //self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        UserInterface::match_command(Command{path: command.path,command_input: command.command_input, temp: "111".parse().unwrap() });
        Ok(())
    }

    fn match_command(mut input:Command){//old:->String, new no return
        //input.path
        let mut res:Result<&str,&str>=Err("1");
        let mut res_diff:Result<RevDiff,&str>=Err("2");
        let mut res_file_diff:Result<FileDiff,&str>=Err("3");
        let mut arg= input.command_input.split_whitespace();
        //println!("input {:?}",arg.next());
        let input_1=arg.next();
        let input_2=arg.next();
        let file=dsr::read_file_as_string(input_2.unwrap_or("1"));//add D://ur//test.txt
        //println!("file content:{}",file.unwrap());//just test read file
        match input_1{
            Some("add") => {
                println!("add");
                res=crate::cmd_interface::readwrite::add(&*input.path);
            }//1
            Some("remove")=> {res=crate::cmd_interface::readwrite::remove("input.path");
                input.temp= "ok".parse().unwrap();
                input.log_for_dev();
            }//2
            Some("commit") => {res=crate::cmd_interface::readwrite::commit("input.path");}//3
            Some("merge") => {res=crate::cmd_interface::readwrite::merge("input.path");}//4
            Some("diff") => {res_diff=crate::cmd_interface::readwrite::diff("input.path","input.path");
                res=Err("2");}//5
            Some("cat") => {res=crate::cmd_interface::readwrite::cat("input.path","input.path");}//6
            Some("status") => {res_file_diff=crate::cmd_interface::readonly::status("input.path");
                res=Err("3");}//status1
            Some("log") => {
                res=crate::cmd_interface::readonly::log("input.path");}//log2
            Some("heads") => {res=crate::cmd_interface::readonly::heads("input.path");}//heads3
            Some("clone") => {res=crate::cmd_interface::createonly::clone("input.path1","input.path");}//1
            Some("checkout") => {res=crate::cmd_interface::createonly::checkout("input.path","input.path");}//2
            Some("pull") => {res=crate::cmd_interface::createonly::pull("input.path","input.path",Some("input.path"));}//3
            Some("push") => {res=crate::cmd_interface::createonly::push("input.path","input.path",Some("input.path"));}//4
            Some("init") => {let init:Repo=crate::vc::repository::init();let paths=dsr::get_wd_path();
                println!("{:?}",paths);
                res=Ok("init successfully")}//1
            _ => {info!(target: "a","{} update {}", "command line","wrong");}
        }
        if res!=Err("1") && res!=Err("2") && res!=Err("3")
        {
            Self::input_handling(res);
        }
        else if res==Err("2"){
            Self::input_handling_special(res_diff);
        }
        else  if res==Err("3"){
            Self::input_handling_special_file(res_file_diff);
        }
    }

    fn input_handling(return_result:Result<&str,&str>){
        println!("{:?}",return_result);
        info!(target: "a","{} update {}", "command line","b");
    }

    fn input_handling_special(return_result:Result<RevDiff,&str>){
        //unimplemented!();
        println!("{:?}","return_result")
    }
    fn input_handling_special_file(return_result:Result<FileDiff,&str>){
        let flag=return_result.clone().unwrap().is_diff;
        if flag==true {let d=return_result.clone().unwrap().patch;
            println!("{}",d); }
        else { println!("No difference, same");}
    }

    fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }
}

