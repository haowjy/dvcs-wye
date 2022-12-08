use crate::vc::{file, repository, revision};
use crate::dsr::*;
use std::collections::HashMap;

use crate::cmd_function::FileDiff;

pub struct RevDiff<'a> {
    files: HashMap<&'a str, FileDiff<'a>>,
}

impl <'a> RevDiff<'a> {
    pub fn new() -> RevDiff<'a> {
        RevDiff {
            files: HashMap::new(),
        }
    }
}

pub fn diff<'a>(wd: &'a str, rev1_id:&'a str, rev2_id:&'a str) -> Result<RevDiff<'a>, &'a str>{
    // go through all files in rev1 and rev2
    // if file in rev1 but not in rev2 -> file deleted
    // if file in rev2 but not in rev1 -> file added
    // if file in rev1 and rev2, but there is a diff -> file modified
    // if file in rev1 and rev2, and there is no diff -> file unchanged

    let repo_opt = repository::load(wd);
    if repo_opt.is_none() { 
        return Err("No repository found");
    } else {
        let repo = repo_opt.unwrap();
        let rev1 = repo.get_rev(rev1_id);
        let rev2 = repo.get_rev(rev2_id);
        if rev1.is_none() || rev2.is_none() { return Err("No revision found"); 
    } else {
            let rev1 = rev1.unwrap();
            let rev2 = rev2.unwrap();

            for (file, info) in rev1.get_manifest() {
                println!("file: {}, info: {:?}", file, info);
            }
        }
    }
    unimplemented!(); //TODO
}

pub fn cat<'a>(wd: &'a str, rev_id:&'a str, path:&'a str) -> Result<&'a str, &'a str>{
    // find path in rev
    // return file content or error
    unimplemented!(); //TODO
}

pub fn add<'a>(wd: &'a str, path:&'a str) -> Result<&'a str, &'a str>{
    
    let cwd = get_wd_path();
    if let Some(mut repo) = repository::load(&cwd) {
        let abs_path = path_compose(&cwd, path);
        
        let res = repo.add_file(&abs_path);
        match res {
            Some(_) => return Ok("add success"),
            None => return Err("add failed: add_file failed"),
        }
    } else {
        return Err("add failed: no repository found")
    }
}

pub fn remove<'a>(wd: &'a str, path:&'a str) -> Result<&'a str, &'a str>{
    // remove the file temporarily to the index branch by acting as if we have deleted the file (not committed yet)
    // just call repo.remove by obtaining absolute path
    unimplemented!(); //TODO
}

pub fn commit<'a>(wd: &'a str, message:&'a str) -> Result<&'a str, &'a str>{
    let cwd = get_wd_path();
    if let Some(mut repo) = repository::load(&cwd) {
        // TODO: message, error handling
        let res = repo.commit();
        match res {
            Some(_) => return Ok("commit success"),
            None => return Err("commit failed: repo.commit failed"),
        }
    } else {
        return Err("commit failed: no repository found")
    }
    // commit the index branch to the head branch, create a new revision and update the head
    // write to log -> where would this be?

    // return a RevDiff if successful
}

pub fn merge<'a>(wd: &'a str, rev_id_source:&'a str, rev_id_dst:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dsr;
    use crate::vc::repository::{self, init};

    #[test]
    fn test_diff() {
        let _ = dsr::delete_dir(".dvcs");
        init();

        // add(wd, path)
    }

    #[test]
    fn test_cat() {
        // TODO
    }

    #[test]
    fn test_add() {
        let wd = get_wd_path();
        // let _ = dsr::delete_dir(".dvcs");
        // let _ = dsr::delete_file("a.txt");
        // init();
        
        let _ = dsr::create_file("a.txt");
        let _ = dsr::write_file("a.txt", "hello world");
        
        let add1 = add(&wd, "a.txt");
        assert_eq!(add1, Ok("add success"));

        let nodef = add(&wd, "nodef_file.txt");
        assert_eq!(nodef, Err("add failed: add_file failed"));
    }

    #[test]
    fn test_remove() {
        // TODO
    }

    #[test]
    fn test_commit() {
        let _ = dsr::delete_dir(".dvcs");
        let _ = dsr::delete_file("predef_file.txt");
        let _ = repository::init().unwrap();

        let _ = dsr::create_file("predef_file.txt");
        let _ = dsr::write_file("predef_file.txt", "hello world");

        // let com1 = commit("test commit");
        // println!("com1: {:?}", com1);
        // assert_eq!(com1, Err("commit failed: repo.commit failed"));

        let _ = add("./", "predef_file.txt");
        
        let com2 = commit("./", "test commit");
        println!("com2: {:?}", com2);
        assert_eq!(com2, Ok("commit success"));
    }

    #[test]
    fn test_merge() {
        // TODO
    }

}