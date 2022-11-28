use std::io;
//use std::path::Path;

//use crate::cmd_function;
use crate::dsr;
use crate::cmd_interface::{createonly, readwrite, readonly};
use crate::cmd_interface::readwrite::RevDiff;
use crate::cmd_function::FileDiff;
use crate::vc::repository::{Repo};
use std::io::{stdout, Write};
use log::{info, warn};
use log4rs;
use crate::vc::revision::Rev;

pub trait Log {
    fn log_for_dev(&self);
}
impl Log for FileDiff<'_> {
     fn log_for_dev(&self){
        info!(target: "a","Difference between File {} and {}", "a","b");
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
    pub fn receive_input_command_loop() ->io::Result<()>{//start here temporary
        log4rs::init_file("src/log4rs.yml", Default::default()).unwrap();
        println!("input q to quit");
        loop{
            print!("dvcs command>: ");
            stdout().flush().unwrap();
            let mut buffer=String::new();
            let stdin=io::stdin();
            let _k=stdin.read_line(&mut buffer);
            if buffer=="q\r\n".to_string() ||buffer=="q".to_string() ||buffer=="q\n".to_string(){
                println!("quit!");
                break;
            }
            //println!("input {}",buffer);
            let path=dsr::get_wd_path();
            //println!("path {}",path);
            let mut command: Command = Command{path,command_input: buffer, temp: "111".parse().unwrap() };
            //self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
            UserInterface::match_command(Command{path: command.path,command_input: command.command_input, temp: "111".parse().unwrap() });
        }
        Ok(())
    }

    fn match_command(mut input:Command){//old:->String, new no return
        //input.path
        let mut res:Result<&str,&str>=Err("1");
        let mut res_diff:Result<RevDiff,&str>=Err("2");
        let mut res_file_diff:Result<FileDiff,&str>=Err("3");
        let mut res_log:Result<Option<Vec<String>>,&str>=Err("4");
        let mut res_head:Result<Rev,&str>=Err("4");
        let mut arg= input.command_input.split_whitespace();
        //println!("input {:?}",arg.next());
        let input_1=arg.next();
        let input_2=arg.next();
        let _file=dsr::read_file_as_string(input_2.unwrap_or("1"));//add D://ur//test.txt
        //println!("file content:{}",file.unwrap());//just test read file
        match input_1{
            Some("add") => {
                //println!("add");
                if Self::check_file_path_valid(input_2)
                {res=readwrite::add(input_2.unwrap());}
                else { res=Err("error file path or unreadable file path"); }
            }//1
            Some("remove")=> {res=readwrite::remove(&*input.path);
                input.temp= "ok".parse().unwrap();
                input.log_for_dev();
            }//2
            Some("commit") => {res=readwrite::commit(input_2.unwrap_or(""));}//3
            Some("merge") => {res=readwrite::merge("input.path");}//4
            Some("diff") => {res_diff=readwrite::diff("input.path","input.path");
                res=Err("2");}//5
            Some("cat") => {res=readwrite::cat("input.path",&*input.path);}//6
            Some("status") => {res_file_diff=readonly::status("input.path");
                res=Err("3");}//status1
            Some("log") => {
                res_log=readonly::log("input.path");
                res=Err("4");}//log2
            Some("heads") => {res_head=readonly::heads("input.path");res=Err("5");}//heads3
            Some("clone") => {res=createonly::clone("input.path1",input_2.unwrap());}//1
            Some("checkout") => {res=createonly::checkout("input.path","input.path");}//2
            Some("pull") => {res=createonly::pull("input.path","input.path",Some("input.path"));}//3
            Some("push") => {res=createonly::push("input.path","input.path",Some("input.path"));}//4
            Some("init") => {
                let init=crate::vc::repository::init();
                if init==Some(()) { res=Ok("init successfully")}
                else { res=Err("init error!") }
                }//1
            _ => {warn!(target: "aaaaaa","{} update {}", "command line","wrong");}
        }
        if res!=Err("1") && res!=Err("2") && res!=Err("3") && res!=Err("4") && res!=Err("5")
        {
            Self::input_handling(res);
            info!(target: "a","{} update {}", "command line","b");
        }
        else if res==Err("2"){
            Self::input_handling_special(res_diff);
            info!(target: "a","{} update {}", "command line","b");
        }
        else  if res==Err("3"){
            Self::input_handling_special_file(res_file_diff);
            info!(target: "a","{} update {}", "command line","b");
        }
        else  if res==Err("4"){
            Self::input_handling_log(res_log);
            info!(target: "a","{} update {}", "command line","b");
        }
        else  if res==Err("5"){
            Self::input_handling_rev(res_head);
            info!(target: "a","{} update {}", "command line","b");
        }
    }

    fn input_handling(return_result:Result<&str,&str>){
        println!("{:?}",return_result);
        info!(target: "a","{} update {}", "command line","b");
    }

    fn input_handling_special(return_result:Result<RevDiff,&str>){
        //waiting structure inside RevDiff, similar with FileDiff
        println!("{:?}","return_result");
        info!(target: "a","{} update {}", "command line","b");
    }
    fn input_handling_special_file(return_result:Result<FileDiff,&str>){
        let fd = return_result.unwrap();
        let flag= fd.is_diff();
        if flag==true {
            let d= fd.get_patch();
            println!("{}",d);
        }
        else { println!("No difference, same");}
        info!(target: "a","{} update {}", "command line","b");
    }
    fn check_file_path_valid(input_2:Option<&str>) ->bool{
        let file=dsr::read_file_as_string(input_2.unwrap_or("1"));//add D://ur//test.txt
        if file.is_err()
        {
            false
        }
        else {
            true
        }
    }
    fn input_handling_log(return_result:Result<Option<Vec<String>>,&str>){
        if return_result.is_err() {println!("{:?}", return_result); }
        else { let vec = return_result.unwrap();
            if vec.is_none() {println!("{:?}", vec); }
            else { println!("{:?}", vec.unwrap()); } }
        info!(target: "a","{} update {}", "command line","b");
    }

    fn input_handling_rev(return_result:Result<Rev,&str>){
        if return_result.is_err() {println!("{:?}", return_result); }
        else {
            let vec = return_result.unwrap();
            println!("{:?}", vec);
        }

        info!(target: "a","{} update {}", "command line","b");
    }
    /*fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }*/
}

