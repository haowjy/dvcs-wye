use std::io;
use std::path::Path;

use crate::cmd_function;
use crate::dsr;
use crate::cmd_interface::{createonly, readwrite, readonly};
use crate::cmd_interface::readwrite::RevDiff;
use druid::widget::{Align, Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};
use crate::cmd_function::FileDiff;
use crate::vc::repository::Repo;
use std::ptr::null;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 300.0;
const WINDOW_TITLE: LocalizedString<Command> = LocalizedString::new("Welcome to Wye's DVCS System!");

//type input=fn()->String;
#[derive(Clone, Data, Lens)]
pub struct Command{
    path: String,
    res: String,
    command_input:String,
}
pub(crate) struct UserInterface{
    commands: Vec<Command>
}

impl UserInterface {
    fn new()-> Self{
        Self{commands:vec![]}
    }

    pub fn gui() {
        let main_window = WindowDesc::new(Self::build_root_widget)
            .title(WINDOW_TITLE)
            .window_size((400.0, 400.0));
        let initial_state = Command {
            path: "".to_string(),
            res: "".to_string(),
            command_input: "".into(),
        };
        AppLauncher::with_window(main_window)
            .launch(initial_state)
            .expect("Failed to launch application");
    }

    fn build_root_widget() -> impl Widget<Command> {
        let label = Label::new(|data: &Command, _env: &Env| format!("Your input is {}!", data.command_input));
        let button = Button::new("submit")
            .on_click(
                |_etc,data:&mut Command,_env|{
                    let a=Self::match_command(data.clone());
                    data.res=a;
                    println!("click");
                })
            .padding(5.0);
        let button2 = Button::new("submit")
            .on_click(
                |_etc,data:&mut Command,_env|{
                    let a=Self::match_command(data.clone());
                    data.res=a;
                    println!("click");
                })
            .padding(5.0);
        let textbox = TextBox::new()
            .with_placeholder("Please input command line here")
            .fix_width(TEXT_BOX_WIDTH)
            .lens(Command::command_input);
        let label2 = Label::new(|data: &Command, _env: &Env| format!("Response {}!", data.res));

        let layout = Flex::column()
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(textbox)
            .with_child(button)
            .with_child(label)
            .with_child(label2);

        Align::centered(layout)
    }

    pub fn receive_input_command_test_inside() ->io::Result<()>{//start here temporary
        let mut buffer=String::new();
        let stdin=io::stdin();
        stdin.read_line(&mut buffer);
        println!("input {}",buffer);
        let path=dsr::get_wd_path();
        println!("path {}",path);
        let mut command: Command = Command{path, res: "".to_string(), command_input: buffer};
        //self.commands.push(Command{path: input_test.clone(),command_input: "input_test.clone()" });
        UserInterface::match_command(Command{path: command.path, res: "".to_string(), command_input: command.command_input });
        Ok(())
    }

    fn match_command(mut input:Command)->String{//old:->String, new no return
        //input.path
        let mut res:Result<&str,&str>=Err("1");
        let mut res_diff:Result<RevDiff,&str>=Err("2");
        let mut res_file_diff:Result<FileDiff,&str>=Err("3");
        let mut arg= input.command_input.split_whitespace();
        //println!("input {:?}",arg.next());
        let input_1=arg.next();
        let input_2=arg.next();
        let file=dsr::read_file_as_string(input_2.unwrap_or("1"));//add D://ur//test.txt
        if input_2!=None{
        println!("file content:{}",file.unwrap());}//just test read file
        match input_1{
            Some("add") => {
                println!("add");
                res=crate::cmd_interface::readwrite::add(&*input.path);
            }//1
            Some("remove")=> {res=crate::cmd_interface::readwrite::remove("input.path");}//2
            Some("commit") => {res=crate::cmd_interface::readwrite::commit("input.path");}//3
            Some("merge") => {res=crate::cmd_interface::readwrite::merge("input.path");}//4
            Some("diff") => {res_diff=crate::cmd_interface::readwrite::diff("input.path","input.path");}//5
            Some("cat") => {res=crate::cmd_interface::readwrite::cat("input.path","input.path");}//6
            Some("status") => {res_file_diff=crate::cmd_interface::readonly::status("input.path");}//status1
            Some("log") => {
                println!("log");
                res=crate::cmd_interface::readonly::log("input.path");}//log2
            Some("heads") => {res=crate::cmd_interface::readonly::heads("input.path");}//heads3
            Some("clone") => {res=crate::cmd_interface::createonly::clone("input.path","input.path");}//1
            Some("checkout") => {res=crate::cmd_interface::createonly::checkout("input.path","input.path");}//2
            Some("pull") => {res=crate::cmd_interface::createonly::pull("input.path","input.path",Some("input.path"));}//3
            Some("push") => {res=crate::cmd_interface::createonly::push("input.path","input.path",Some("input.path"));}//4
            Some("init") => {let init:Repo=crate::vc::repository::init();
                let paths=dsr::get_wd_path();
                println!("{:?}",paths);
                res=Ok("&*paths")}//1
            _ => {}
        }
        if res!=Err("1")
        {
            Self::input_handling(res);
            input.res=res.unwrap().to_string();
        }
        else if res!=Err("2"){
            Self::input_handling_special(res_diff);
            input.res= "res_diff.unwrap()".parse().unwrap();
        }
        else{
            Self::input_handling_special_file(res_file_diff);
            input.res="res_file_diff.unwrap()".parse().unwrap();
        }
        return input.res;
    }

    fn input_handling(return_result:Result<&str,&str>){
        println!("{:?}",return_result)
    }

    fn input_handling_special(return_result:Result<RevDiff,&str>){
        //unimplemented!();
        println!("{:?}","return_result")
    }
    fn input_handling_special_file(return_result:Result<FileDiff,&str>){
        //unimplemented!();
        println!("{:?}","return_result")
    }
    fn input_handling_backup<E: std::fmt::Debug>(return_result:Result<(), E>){
        println!("{:?}",return_result)
    }
}

