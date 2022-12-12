
use crate::dsr::{self, path_compose};
use crate::readwrite::{add, commit};
use crate::vc::repository::{self, init};

pub fn remove_git_and_init(cwd:&str) {
    let _ = dsr::clear_dir(cwd, vec![]);
    let _ = init(Some(cwd));
}

pub fn create_files_ab(cwd:&str){
    let _ = dsr::create_file(&path_compose(cwd, "a.txt"));
    let _ = dsr::write_file(&path_compose(cwd, "a.txt"), "hello world");
    let _ = dsr::create_file(&path_compose(cwd, "b.txt"));
    let _ = dsr::write_file(&path_compose(cwd, "b.txt"), "hello world");
}

pub fn write_files_ab(cwd:&str){
    let _ = dsr::write_file(&path_compose(cwd, "a.txt"), "A Change");
    let _ = dsr::write_file(&path_compose(cwd, "b.txt"), "B Change");
}

pub fn create_file_c(cwd:&str){
    let _ = dsr::create_file(&path_compose(cwd, "c.txt"));
    let _ = dsr::write_file(&path_compose(cwd, "c.txt"), "hello world from C");
}

pub fn create_files_and_commit_ab1(cwd:&str) -> String {
    create_files_ab(cwd);
    let _ = add(cwd, "a.txt");
    let _ = add(cwd, "b.txt");
    commit(cwd, "commit a and b").unwrap()
}

pub fn write_create_files_and_commit_abc2(cwd:&str) -> String {
    write_files_ab(cwd);
    create_file_c(cwd);
    let _ = add(cwd, "a.txt");
    let _ = add(cwd, "b.txt");
    let _ = add(cwd, "c.txt");
    commit(cwd, "commit a and b and create c").unwrap()
}

pub fn write_files_edit_and_commit_ab3(cwd:&str) -> String{
    let _ = dsr::write_file(&path_compose(cwd, "a.txt"), "InsertFirst\nA Change 2");
    let _ = dsr::write_file(&path_compose(cwd, "b.txt"), "InsertFirst\nB Change 2");
    let _ = add(cwd, "a.txt");
    let _ = add(cwd, "b.txt");
    commit(cwd, "commit a and b edit").unwrap()
}