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
fn parse_error(res: Errors) -> String {
    match res {
        ErrSerde(Error) => {println!("{:?}", Error);Error.to_string()},
        ErrIo(Error) => {println!("{:?}", Error);Error.to_string()},
        ErrSys(Error) => {println!("{:?}", Error);Error.to_string()},
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
    /// add specific files that you want to track
    add {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        path: Vec<String>,
    },
    /// remove specific files from tracking list
    remove {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        path: Vec<String>,
    },
    /// commit changes and create a new revision
    commit {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        #[arg(default_value_t)]
        /// Name of the package to search
        message: String,
    },
    /// merge two revisions
    merge {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        rev_id: String,
        /// Name of the package to search
        rev_dest: String,
    },
    /// check the changes between revisions
    diff {
        //#[arg(short, long, default_value_t = dsr::get_wd_path())]
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        rev_id_1: String,
        rev_id_2: String,
    },
    /// inspect a file of a given revision
    cat {
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
        /// Name of the package to search
        rev_id: String,
        path: String,
    },
    ///  check the current status of current repository
    status {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    /// view the change log
    log {
        #[arg(default_value_t = dsr::get_wd_path())]
        /// Name of the package to search
        wd_path: String,
        #[arg(default_value_t)]
        rev_id: String,
    },
    /// show the current heads
    heads {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd_path: String,
    },
    ///  copy an existing repository
    clone {
        /// Name of the package to search
        #[arg(default_value_t = dsr::get_wd_path())]
        wd: String,
        remote: String,
    },
    /// check out a specific revision
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
    /// create an empty repository
    init {
        /// Name of the package to search
        #[arg(default_value_t)]
        wd_path: String,
    },
}
impl Wye {
    pub fn input_command() ->io::Result<()>{//start here temporary
        //cli start here
        let args = Wye::parse();
        println!("args {:?}!", args);
        let default_wd_path=dsr::get_wd_path();
        match args.command {
            Command::add { mut wd_path,mut path } => {
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
                    path.iter().fold(0, |acc, x| {
                    if Self::check_file_path_valid(Some(&*x))
                    {
                        res=readwrite::add(&wd_path,&*x);
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
            Command::remove { mut wd_path,path } => {
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
                    path.iter().fold(0, |acc, x| {
                        if Self::check_file_path_valid(Some(&*x))
                        {
                            res=readwrite::remove(&wd_path,&*x);
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
            Command::commit {mut wd_path, message } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res:Result<RevDiff,Errors>=Err(Errstatic("1"));
                res=readwrite::commit(&wd_path,&message);
                Self::input_handling_special(res);
                println!("message is: {:?}", message)
            }
            Command::merge { mut wd_path,rev_id,rev_dest } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                println!("wd_path is: {:?}", wd_path);
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                res=readwrite::merge(&wd_path, rev_id.clone());
                Self::input_handling(res);
                println!("path1 is: {:?}", rev_id);
            }
            Command::diff { mut wd_path,rev_id_1,rev_id_2 } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res_diff:Result<RevDiff,Errors>=Err(Errstatic("2"));
                res_diff=readwrite::diff(&wd_path,&rev_id_1, &rev_id_2);
                Self::input_handling_special(res_diff);
                println!("rev_id_1 is: {:?} rev_id_2 is: {:?}", rev_id_1,rev_id_2)
            }
            Command::cat { mut wd_path,rev_id,path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                res=readwrite::cat(&wd_path,&rev_id,&path);
                Self::input_handling(res);
                println!("rev_id is: {:?}", rev_id);
                println!("path is: {:?}", path)
            }
            Command::status { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let res_file_diff=readonly::status(&wd_path);
                Self::input_handling_status(res_file_diff);
                println!("wd_path is: {:?}", wd_path)
            }
            Command::log { mut wd_path,rev_id } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let mut res_log:Result<Option<Vec<String>>,Errors>;
                res_log=readonly::log(&wd_path,&rev_id);
                //parse_error(readonly::log(&path).unwrap_err());
                Self::input_handling_log(res_log);
                println!("wd_path is: {:?}", wd_path)
            }
            Command::heads { mut wd_path } => {
                if wd_path.eq("-d") || wd_path.eq("-"){
                    wd_path=default_wd_path;
                }
                let res_head=readonly::heads(&wd_path);
                //parse_error(readonly::heads(&path).unwrap_err());
                Self::input_handling_rev(res_head);
                println!("path is: {:?}", wd_path)
            }
            Command::clone { mut wd, remote} => {
                if wd.eq("-d") || wd.eq("-"){
                    wd=default_wd_path;
                }
                let res=createonly::clone(&wd, &remote);
                Self::input_handling(res);
                println!("wd_path is: {:?}", wd)
            }
            Command::checkout { mut path,rev } => {
                if path.eq("-d") || path.eq("-"){
                    path=default_wd_path;
                }
                let res=createonly::checkout(&path, &rev); // TODO:
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            Command::pull { mut path,remote,head } => {
                if path.eq("-d") || path.eq("-"){
                    path=default_wd_path;
                }
                let res=createonly::pull(&path, &remote);
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            Command::push { mut path,remote,head } => {
                if path.eq("-d") || path.eq("-"){
                    path=default_wd_path;
                }
                let res=createonly::push(&path, &remote);
                Self::input_handling(res);
                println!("path is: {:?}", path)
            }
            Command::init { mut wd_path } => {
                let mut opt_path:Option<&str>=None;
                if wd_path.eq("-d") || wd_path.eq("-") || wd_path.is_empty(){
                    opt_path=None;
                }
                else { opt_path=Some(&wd_path)}
                let mut res:Result<String,Errors>=Err(Errstatic("1"));
                let init=crate::vc::repository::init(opt_path);
                if init.unwrap()==() { res=Ok("init successfully".to_string());}
                else {
                    res=Err(Errstatic("init error!")) }
                Self::input_handling(res);
                println!("path is: {:?}", wd_path)
            }
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
            println!("{:?}",return_result);
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
        println!("{:?}","return_result");
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
        let file=dsr::read_file_as_string(input_2.unwrap_or("1"));//add D://ur//test.txt
        if file.is_err()
        {
            false
        }
        else {
            true
        }
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

