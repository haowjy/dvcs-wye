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
        /// Name of the package to search
        path: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// remove specific files from tracking list
    Remove {
        /// Name of the package to search
        path: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// commit changes and create a new revision
    Commit {
        /// Name of the package to search
        message: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// merge two revisions
    Merge {
        /// Name of the package to search
        rev_id: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// check the changes between revisions
    Diff {
        //#[arg(short, long, default_value_t = dsr::get_wd_path())]
        /// Name of the package to search
        rev_id_1: String,
        rev_id_2: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// inspect a file of a given revision
    Cat {
        /// Name of the package to search
        rev_id: String,
        path: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    ///  check the current status of current repository
    Status {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// view the change log
    Log {
        #[arg(default_value_t)]
        rev_id: String,
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
        new_branch_alias: String,
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
        head: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
    },
    /// create an empty repository
    Init {
        /// Name of the package to search
        #[arg(default_value_t)]
        wd_path: String,
    },
    /// create an empty repository
    Test {
        /// Name of the package to search
        #[command(subcommand)]
        wd_path: SubCommand,
    },
}
#[derive(Parser,Debug)]
enum SubCommand {
    Defalut {
        #[arg(default_value_t)]
        revision: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    DefalutPath {
        revision: String,
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    DefalutRev {
        wd_path: String,
        #[arg(default_value_t)]
        revision: String,
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
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                println!("wd_path is: {:?}", wd_path);
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                println!("path is: {:?}", path);
                if path.is_empty() {
                    res=Err(Errstatic("Wrong Empty Path"));
                    println!("path is empty");
                }
                else {
                    let path_spoilt:Vec<&str>=path.split(',').collect();
                    path_spoilt.iter().fold(0, |acc, &x| {
                        if Self::check_file_path_valid(Some(x))
                        {
                            res=readwrite::add(&wd_path,x);
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
            Command::Remove { mut wd_path,path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                println!("wd_path is: {:?}", wd_path);
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                println!("path is: {:?}", path);
                if path.is_empty() {
                    res=Err(Errstatic("Wrong Empty Path"));
                    println!("path is empty");
                }
                else {
                    let path_spoilt:Vec<&str>=path.split(',').collect();
                    path_spoilt.iter().fold(0, |acc, &x| {
                        if Self::check_file_path_valid(Some(x))
                        {
                            res=readwrite::remove(&wd_path,x);
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
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                //let mut res:Result<RevDiff,Errors>=Err(Errstatic("1"));
                let res=readwrite::commit(&wd_path,&message);
                Self::input_handling_new_String(res);
                println!("message is: {:?}", message)
            }
            Command::Merge { mut wd_path,rev_id } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                println!("wd_path is: {:?}", wd_path);
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                res=readwrite::merge(&wd_path, rev_id.clone());
                Self::input_handling(res);
                println!("path1 is: {:?}", rev_id);
            }
            Command::Diff { mut wd_path,rev_id_1,rev_id_2 } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res_diff:Result<RevDiff,Errors>=Err(Errstatic("2"));
                res_diff=readwrite::diff(&wd_path,&rev_id_1, &rev_id_2);
                Self::input_handling_special(res_diff);
                println!("rev_id_1 is: {:?} rev_id_2 is: {:?}", rev_id_1,rev_id_2)
            }
            Command::Cat { mut wd_path,rev_id,path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                res=readwrite::cat(&wd_path,&rev_id,&path);
                Self::input_handling(res);
                println!("rev_id is: {:?}", rev_id);
                println!("path is: {:?}", path)
            }
            Command::Status { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let res_file_diff=readonly::status(&wd_path);
                Self::input_handling_status(res_file_diff);
                println!("wd_path is: {:?}", wd_path)
            }
            Command::Log { mut wd_path,rev_id } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res_log:Result<Option<Vec<String>>,Errors>;
                res_log=readonly::log(&wd_path,&rev_id);
                //parse_error(readonly::log(&path).unwrap_err());
                Self::input_handling_log(res_log);
                println!("wd_path is: {:?}", wd_path)
            }
            Command::Heads { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let res_head=readonly::heads(&wd_path);
                //parse_error(readonly::heads(&path).unwrap_err());
                Self::input_handling_rev(res_head);
                println!("path is: {:?}", wd_path)
            }
            Command::Clone { remote,mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let res=createonly::clone(&wd_path, &remote);
                Self::input_handling(res);
                println!("wd_path is: {:?}", wd_path)
            }
            Command::Checkout { option,new_branch_alias } => {
                let mut rev =String::new(); let mut path =String::new();
                match option{
                    SubCommand::Defalut { ref wd_path,ref revision} => {
                        path= wd_path.clone();
                        rev=revision.clone();
                    }
                    SubCommand::DefalutPath { ref wd_path,ref revision} => {
                        path= wd_path.clone();
                        rev=revision.clone();
                    }
                    SubCommand::DefalutRev { ref wd_path,ref revision} => {
                        path= wd_path.clone();
                        rev=revision.clone();
                    }
                    _ => {}
                }
                if rev.eq(""){
                    let a= repository::load(&path).unwrap().get_heads().get(&new_branch_alias).unwrap().clone();
                    rev=a.clone();
                    println!("{}",rev);
                }
                let res=createonly::checkout(&path, &rev,Some(new_branch_alias)); // TODO:
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            Command::Pull { mut path,remote,head } => {
                if path.eq("-d") || path.eq("-"){
                    path=default_wd_path;
                }
                let res=createonly::pull(&path, &remote);
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            Command::Push { mut path,remote,head } => {
                if path.eq("-d") || path.eq("-"){
                    path=default_wd_path;
                }
                let res=createonly::push(&path, &remote);
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            Command::Init { mut wd_path } => {
                let mut opt_path:Option<&str>=None;
                if wd_path.eq("-d") || wd_path.eq("-") || wd_path.is_empty(){
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

    fn input_handling(return_result:Result<String,Errors>){
        println!("{:?}",return_result);
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
    fn input_handling_log(return_result:Result<Option<Vec<String>>,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else { let vec = return_result.unwrap();
            if vec.is_none() {println!("{:?}", vec); }
            else { println!("{:?}", vec.unwrap()); } }
    }

    fn input_handling_rev(return_result:Result<Vec<String>,Errors>){
        if return_result.is_err() {
            parse_error(return_result.unwrap_err());
        }
        else {
            let vec = return_result.unwrap();
            println!("{:?}", vec);
        }
    }
    /*fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }*/
}

