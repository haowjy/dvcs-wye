use std::io;
//use std::path::Path;
use clap::{Parser, Subcommand, FromArgMatches, ArgMatches};
use clap::error::{Error,ErrorKind};
//use crate::cmd_function;
use crate::dsr;
use crate::cmd_interface::{createonly, readwrite, readonly};
use crate::cmd_interface::readwrite::RevDiff;
use crate::cmd_function::FileDiff;
use crate::vc::repository::{load, Repo};
use std::io::{stdout, Write};
/*use log::{info, warn};
use log4rs;*/
use crate::vc::revision::Rev;
#[derive(Debug)]
pub enum Errors {
    ErrSerde(serde_json::Error),
    ErrIo(std::io::Error),
    ErrSys(Error),
    ErrStr(String),
    Errstatic(&'static str),
    ErrUnknown,
}
use Errors::{ErrSerde,ErrIo,ErrSys, ErrStr,Errstatic, ErrUnknown};
use crate::vc::repository;

fn parse_error(res: Errors) -> String {
    match res {
        ErrSerde(Error) => {println!("{}", Error);Error.to_string()},
        ErrIo(Error) => {println!("{}", Error);Error.to_string()},
        ErrSys(Error) => {println!("{}", Error);Error.to_string()},
        ErrStr(String) => {println!("{}", String);String},
        Errstatic(Str) => {println!("{}", Str);Str.to_string()},
        ErrUnknown => {println!("ErrUnknown");"ErrUnknown".to_string()},
    }
}
/*fn match_Errors(error: Errors) -> String {
    match error {
        Errors::Error => "error inside".to_string(),
        Errors::Error_betweenbalabalabala => "Error_betweenbalabalabala".to_string(),
        Errors::String_content => "more than string".to_string(),
        Errors::None =>"None".to_string(),
    }
}*/
//type input=fn()->String;
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Wye {
    #[command(subcommand)]
    command: Command,
}
#[derive(Parser,Debug)]
enum Command {
    /// add specific files(multi files use "," to spilt) that you want to track
    Add {
        path: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// remove specific files from tracking list
    Remove {
        path: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// commit changes and create a new revision
    Commit {
        /// Message need to be single word
        message: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// merge two revisions
    Merge {
        rev_id: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// check the changes between revisions
    Diff {
        rev_id_1: String,
        rev_id_2: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// inspect a file of a given revision
    Cat {
        rev_id: String,
        path: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    ///  check the current status of current repository
    Status {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// view the change log
    Log {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// show the current heads
    Heads {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    ///  copy an existing repository
    Clone {
        remote: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// check out a specific revision
    Checkout {
        rev_id: String,
        #[command(subcommand)]
        option: SubCommand,
    },
    /// pull the changes from another repository
    Pull {
        remote: String,
        head: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
    },
    /// push changes into another repository
    Push {
        remote: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
    },
    /// create an empty repository
    Init {
        #[arg(default_value_t)]
        wd_path: String,
    },
    /*/// create an empty repository
    Test {
        /// Name of the package to search
        #[command(subcommand)]
        wd_path: SubCommand,
    },*/
}
#[derive(Parser,Debug)]
enum SubCommand {
    /// default branch_alias
    B {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// default path
    P {
        #[arg(default_value_t)]
        new_branch_alias: String,
    },
    /// need to input both new_branch_alias and wd_path
    A {
        new_branch_alias: String,
        wd_path: String,
    }

}
impl Wye {
    pub fn input_command() ->io::Result<()>{//start here temporary
        //cli start here
        let args = Wye::parse();
        //println!("args {:?}!", args);
        let default_wd_path=dsr::get_wd_path();
        match args.command {
            Command::Add { mut wd_path,mut path } => {
                if wd_path.eq("-d") || wd_path.eq("-")||wd_path.eq("."){
                    wd_path=default_wd_path.clone();
                }
                if path.eq("-d") || path.eq("-")||path.eq("."){
                    path=default_wd_path;
                }
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                if path.is_empty() {
                    res=Err(Errstatic("Wrong Empty Path"));
                    println!("path is empty");
                }
                else {
                    let path_spoilt:Vec<&str>=path.split(',').collect();
                    path_spoilt.iter().fold(0, |acc, &x| {
                        if Self::check_file_path_valid(Some(x))
                        {
                            if Self::check_file_file_or_path(Some(x)) {res=readwrite::add(&wd_path,x); }
                            else {
                                let mut list_files:Vec<String> = vec![];
                                let mut ignore:Vec<&str> = vec![".dvcs"];
                                dsr::get_files(x,ignore,&mut list_files);//file from fd
                                list_files.iter().fold(0,|acc,x1| {
                                    res=readwrite::add(&wd_path,x1);
                                    0});

                            }

                        }

                        else {
                            res=Err(Errstatic("error file path or unreadable file path"));
                        }
                        0
                    }
                    );
                }
                Self::input_handling(res);
            }
            Command::Remove { mut wd_path,mut path } => {
                if wd_path.eq("-d") || wd_path.eq("-")||path.eq("."){
                    wd_path=default_wd_path.clone();
                }
                if path.eq("-d") || path.eq("-")||path.eq("."){
                    path=default_wd_path;
                }
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                if path.is_empty() {
                    res=Err(Errstatic("Wrong Empty Path"));
                    println!("path is empty");
                }
                else {
                    let path_spoilt:Vec<&str>=path.split(',').collect();
                    path_spoilt.iter().fold(0, |acc, &x| {
                        if Self::check_file_path_valid(Some(x))
                        {
                            if Self::check_file_file_or_path(Some(x)) {res=readwrite::remove(&wd_path,x); }
                            else {
                                let mut list_files:Vec<String> = vec![];
                                let mut ignore:Vec<&str> = vec![".dvcs"];
                               dsr::get_files(x,ignore,&mut list_files);//file from fd
                                list_files.iter().fold(0,|acc,x1| {
                                    res=readwrite::remove(&wd_path,x1);
                                    0});

                            }

                        }
                        else {
                            res=Err(Errstatic("error file path or unreadable file path"));
                        }
                        0
                    }
                    );
                }
                Self::input_handling(res);
            }
            Command::Commit {mut wd_path, message } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                //let mut res:Result<RevDiff,Errors>=Err(Errstatic("1"));
                let res=readwrite::commit(&wd_path,&message);
                Self::input_handling_new_String(res);
            }
            Command::Merge { mut wd_path,rev_id } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                res=readwrite::merge(&wd_path, rev_id.clone());
                Self::input_handling(res);
            }
            Command::Diff { mut wd_path,rev_id_1,rev_id_2 } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let mut res_diff:Result<RevDiff,Errors>=Err(Errstatic("2"));
                res_diff=readwrite::diff(&wd_path,&rev_id_1, &rev_id_2);
                Self::input_handling_special(res_diff);
            }
            Command::Cat { mut wd_path,rev_id,path } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                res=readwrite::cat(&wd_path,&rev_id,&path);
                Self::input_handling_cat(res);
            }
            Command::Status { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let res_file_diff=readonly::status(&wd_path);
                //Self::input_handling_status(res_file_diff);
            }
            Command::Log { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let mut res_log:Result<Option<Vec<String>>,Errors>;
                res_log=readonly::log(&wd_path);
                //parse_error(readonly::log(&path).unwrap_err());
                Self::input_handling_log(res_log);
            }
            Command::Heads { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let res_head=readonly::heads(&wd_path);
                //parse_error(readonly::heads(&path).unwrap_err());
                Self::input_handling_rev(res_head);
            }
            Command::Clone { remote,mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-")|| wd_path.eq("."){
                    wd_path=default_wd_path;
                }
                let res=createonly::clone(&wd_path, &remote);
                Self::input_handling(res);
            }
            Command::Checkout { option,rev_id } => {
                let mut alias =String::new(); let mut path =String::new();
                match option{
                    SubCommand::B { ref wd_path} => {
                        path= wd_path.clone();
                        alias= "".parse().unwrap();
                    }
                    SubCommand::P { ref new_branch_alias} => {
                        path= default_wd_path;
                        alias=new_branch_alias.clone();
                    }
                    SubCommand::A { ref wd_path,ref new_branch_alias} => {
                        path= wd_path.clone();
                        alias=new_branch_alias.clone();
                    }
                    _ => {}
                }
                let mut option_alias =None;
                if !alias.eq(""){
                    option_alias=Some(alias);
                }

                let res=createonly::checkout(&path, &rev_id,option_alias); // TODO:
                Self::input_handling(res);
            }
            Command::Pull { mut path,remote,head } => {
                if path.eq("-d") || path.eq("-")|| path.eq("."){
                    path=default_wd_path;
                }
                let res=createonly::pull(&path, &remote);
                Self::input_handling(res);
            }
            Command::Push { mut path,remote } => {
                if path.eq("-d") || path.eq("-")|| path.eq("."){
                    path=default_wd_path;
                }
                let res=createonly::push(&path, &remote);
                Self::input_handling(res);
            }
            Command::Init { mut wd_path } => {
                let mut opt_path:Option<&str>=None;
                if wd_path.eq("-d") || wd_path.eq("-") || wd_path.is_empty()|| wd_path.eq("."){
                    opt_path=None;
                }
                else { opt_path=Some(&wd_path)}
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                let init=crate::vc::repository::init(opt_path);
                match init { Ok(string)=>{res=Ok(string);}
                    Err(String)=>{res=Err(String)} }
                Self::input_handling_new_String(res);
            }
            /*Command::Test { wd_path } => {
                match wd_path{
                    SubCommand::O { ref wd_path} => {
                        println!("path is: {:?}", wd_path)
                    }
                    /*SubCommand::DefalutRev { ref , .. } => {
                        println!("path is: {:?}", wd_path)
                    }*/
                    _ => {}
                }

                println!("path is: {:?}", wd_path)
            }*/
            _ => {
                println!("Sorry! Wrong input! Command not found");
            }
        }

        //cli close here

        //log4rs::init_file("src/log4rs.yml", Default::default()).unwrap();
        Ok(())
    }

    fn input_handling_new(return_result:Result<&str,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            println!("{}",return_result.unwrap());
        }
    }
    fn input_handling_new_String(return_result:Result<String,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            println!("{}",return_result.unwrap());
        }
    }
    fn input_handling_new_commit(return_result:Result<String,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            println!("Revision ID: {}",return_result.unwrap());
        }
    }
    fn input_handling_status(return_result:Result<&str,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
    }

    fn input_handling_init(return_result:Result<(),Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else if return_result.unwrap()==() {
            println!("init successfully");
        }

    }
    fn input_handling_cat(return_result:Result<String,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            println!("content: {}",return_result.unwrap());
        }
    }

    fn input_handling(return_result:Result<String,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            println!("{}",return_result.unwrap());
        }
    }

    fn input_handling_special(return_result:Result<RevDiff,Errors>){
        //waiting structure inside RevDiff, similar with FileDiff
        println!("{:?}",return_result);
    }
    /*fn input_handling_special_file(return_result:Result<FileDiff,&str>){
        let fd = return_result.unwrap();
        let flag= fd.is_diff();
        if flag==true {
            let d= fd.get_patch();
            println!("{}",d);
        }
        else { println!("No difference, same");}
        info!(target: "a","{} update {}", "command line","b");
    }*/
    fn check_file_path_valid(input_2:Option<&str>) ->bool{
        dsr::is_path_valid(input_2.unwrap_or("1"))//add D://ur//test.txt
    }
    fn check_file_file_or_path(input_2:Option<&str>) ->bool{
        let a=dsr::read_file_as_string(input_2.unwrap_or("1"));
        if a.is_ok(){return true} else { return false }
    }
    fn input_handling_log(return_result:Result<Option<Vec<String>>,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else { let vec = return_result.unwrap();
            if vec.is_none() {println!("{:?}", vec); }
            else { //println!("{:?}", vec.clone().unwrap());
                vec.unwrap().iter().fold(0,|acc,x|{
                    println!("{}",x);
                    //println!();
                    0});
            } }
    }

    fn input_handling_rev(return_result:Result<Vec<String>,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            println!("heads:");
            let vec = return_result.unwrap();
            vec.iter().fold(0,|acc,x|{
                println!("{}",x);
                0});
        }
    }
    /*fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }*/
}

