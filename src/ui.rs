use std::io;
//use std::path::Path;
use clap::{Parser, Subcommand, FromArgMatches, ArgMatches};
use clap::error::{Error,ErrorKind};
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
#[derive(Debug)]
pub enum Errors {
    ErrSys(Error),
    ErrStr(String),
    ErrUnknown,
}
use Errors::{ErrSys, ErrStr, ErrUnknown};
fn parse_error(res: Result<(), Errors>) -> String {
    match res.unwrap_err() {
        ErrSys(Error) => {print!("{:?}", Error);Error.to_string()},
        ErrStr(String) => {print!("{}", String);String},
        ErrUnknown => {print!("ErrUnknown");"ErrUnknown".to_string()},
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
    /// Add file
    add {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        path: Vec<String>,
    },
    /// remove file
    remove {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        path: Vec<String>,
    },
    /// commit changes
    commit {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        #[arg(default_value_t)]
        /// Name of the package to search
        message: String,
    },
    /// merge version
    merge {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        rev_id: String,
        /// Name of the package to search
        rev_dest: String,
    },
    /// Init the system
    diff {
        //#[arg(short, long, default_value_t = dsr::get_wd_path())]
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        rev_id_1: String,
        rev_id_2: String,
    },
    /// see file inside
    cat {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        rev_id: String,
        path: String,
    },
    /// show difference between old and current version
    status {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
    },
    /// Show log to user
    log {
        #[arg(default_value_t = dsr::get_wd_path())]
        /// Name of the package to search
        path: String,
    },
    /// show heads
    heads {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
    },
    /// clone file and content
    clone {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd: String,
        remote: String,
    },
    /// checkout the version
    checkout {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
        rev: String,
    },
    /// pull the version from server
    pull {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
        remote: String,
        head: String,
    },
    /// push new version
    push {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
        remote: String,
        head: String,
    },
    /// init file structure in computer
    init {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        path: String,
    },
}
impl Wye {
    pub fn input_command() ->io::Result<()>{//start here temporary
        //cli start here
        let args = Wye::parse();
        println!("args {:?}!", args);
        let wd_path=dsr::get_wd_path();
        match args.command {
            Command::add { wd_path,mut path } => {
                let mut res:Result<&str,&str>=Err("1");
                println!("path is: {:?}", path);
                if path.is_empty() {
                    res=Err("Wrong Empty Path");
                    println!("path is empty");
                    }
                else {
                    path.iter().fold(0, |acc, x| {
                    if Self::check_file_path_valid(Some(&*x))
                    {
                        res=readwrite::add(&wd_path,&*x);
                    }
                    else {
                        res=Err("error file path or unreadable file path");
                    }
                    0
                }
                );
                }
                Self::input_handling(res);
                info!(target: "add","{} update {}", "command line","b");
            }
            Command::remove { wd_path,path } => {
                let mut res:Result<&str,&str>=Err("1");
                println!("path is: {:?}", path);
                if path.is_empty() {
                    res=Err("Wrong Empty Path");
                    println!("path is empty");
                }
                else {
                    path.iter().fold(0, |acc, x| {
                        if Self::check_file_path_valid(Some(&*x))
                        {
                            res=readwrite::remove(&wd_path,&*x);
                        }
                        else {
                            res=Err("error file path or unreadable file path");
                        }
                        0
                    }
                    );
                }
                Self::input_handling(res);
                info!(target: "remove","{} update {}", "command line","b");
            }
            Command::commit {wd_path, message } => {
                let mut res:Result<&str,&str>=Err("1");
                res=readwrite::commit(&wd_path,&message);
                Self::input_handling(res);
                info!(target: "commit","{} update {}", "command line","b");
                println!("message is: {:?}", message)
            }
            Command::merge { wd_path,rev_id,rev_dest } => {
                let mut res:Result<&str,&str>=Err("1");
                res=readwrite::merge(&wd_path, &rev_id, &rev_dest);
                Self::input_handling(res);
                info!(target: "merge","{} update {}", "command line","b");
                println!("path1 is: {:?}", rev_id);
            }
            Command::diff { wd_path,rev_id_1,rev_id_2 } => {
                let mut res_diff:Result<RevDiff,&str>=Err("2");
                res_diff=readwrite::diff(&wd_path,&rev_id_1, &rev_id_2);
                Self::input_handling_special(res_diff);
                info!(target: "a","{} update {}", "command line","b");
                println!("rev_id_1 is: {:?} rev_id_2 is: {:?}", rev_id_1,rev_id_2)
            }
            Command::cat { wd_path,rev_id,path } => {
                let mut res:Result<&str,&str>=Err("1");
                res=readwrite::cat(&wd_path,&rev_id,&path);
                Self::input_handling(res);
                info!(target: "cat","{} update {}", "command line","b");
                println!("rev_id is: {:?}", rev_id);
                println!("path is: {:?}", path)
            }
            Command::status { path } => {
                let mut res_file_diff:Result<FileDiff,&str>=Err("3");
                res_file_diff=readonly::status(&path);
                Self::input_handling_special_file(res_file_diff);
                info!(target: "status","{} update {}", "command line","b");
                println!("path is: {:?}", path)
            }
            Command::log { path } => {
                let mut res_log:Result<Option<Vec<String>>,&str>=Err("4");
                res_log=readonly::log(&path);
                Self::input_handling_log(res_log);
                info!(target: "log","{} update {}", "command line","b");
                println!("path is: {:?}", path)
            }
            Command::heads { path } => {
                let res_head=readonly::heads(&path);
                Self::input_handling_rev(res_head);
                info!(target: "heads","{} update {}", "command line","b");
                println!("path is: {:?}", path)
            }
            Command::clone { wd, remote, .. } => {
                let res=createonly::clone(&wd, &remote);
                Self::input_handling(res);
                info!(target: "clone","{} update {}", "command line","b");
                println!("wd_path is: {:?}", wd)
            }
            Command::checkout { path,rev } => {
                let res=createonly::checkout(&path, &rev);
                Self::input_handling(res);
                info!(target: "checkout","{} update {}", "command line","b");
                println!("path is: {:?}", path)
            }
            Command::pull { path,remote,head } => {
                let res=createonly::pull(&path, &remote, Some(&head));
                Self::input_handling(res);
                info!(target: "pull","{} update {}", "command line","b");
                println!("path is: {:?}", path)
            }
            Command::push { path,remote,head } => {
                let res=createonly::push(&path, &remote, Some(&head));
                Self::input_handling(res);
                info!(target: "push","{} update {}", "command line","b");
                println!("path is: {:?}", path)
            }
            Command::init { path } => {
                let mut res:Result<&str,&str>=Err("1");
                let init=crate::vc::repository::init();
                if init==Some(()) { res=Ok("init successfully");}
                else { res=Err("init error!") }
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            _ => {
                println!("Sorry! Wrong input! Command not found");
            }
        }

        //cli close here

        log4rs::init_file("src/log4rs.yml", Default::default()).unwrap();
        Ok(())
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

