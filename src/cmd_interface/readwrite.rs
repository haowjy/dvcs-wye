use crate::ui::{Errors, Errors::*};
use crate::vc::{file, repository, revision};
use crate::dsr::{*};
use std::collections::HashMap;

use crate::cmd_function::{FileDiff, file_diff};

#[derive(Debug)]
pub struct RevDiff {
    files: HashMap<String, FileDiff>,
}

impl RevDiff {
    pub fn new() -> RevDiff {
        RevDiff {
            files: HashMap::new(),
        }
    }
    pub fn get_files(&self) -> &HashMap<String, FileDiff> {
        &self.files
    }
    pub fn to_string(&self) -> String {
        let mut res:String = String::new();
        for (file, diff) in &self.files {
            res.push_str(&format!("{}\n{:?}\n", file, diff.get_diff_type()));
        }
        res
    }
}

pub fn diff<'a>(wd: &'a str, rev1_id:&'a str, rev2_id:&'a str) -> Result<RevDiff, Errors>{
    // TODO: should work, but not tested cuz repo is not working atm
    // go through all files in rev1 and rev2
    // if file in rev1 but not in rev2 -> file deleted
    // if file in rev2 but not in rev1 -> file added
    // if file in rev1 and rev2, but there is a diff -> file modified
    // if file in rev1 and rev2, and there is no diff -> file unchanged
    let mut rev_diff = RevDiff::new();

    let repo = repository::load(wd)?;

    let rev1 = repo.get_rev(rev1_id)?;
    let rev2 = repo.get_rev(rev2_id)?;
    
    let rev1_manifest = rev1.get_manifest();
    let rev2_manifest = rev2.get_manifest();

    for (file1, info1) in rev1_manifest.clone() {
        let content1 = info1.get_content()?;

        let file2_opt = rev2_manifest.get(&file1);
        if file2_opt.is_none() { // file deleted
            let file_diff = file_diff(content1, "".to_string());
            rev_diff.files.insert(file1.clone(), file_diff);
        } else { // file modified or unchanged
            let content2 = file2_opt.unwrap().get_content()?;
            rev_diff.files.insert(file1.clone(), file_diff(content1, content2));
        }
    }
    for (file2, info2) in rev2_manifest.clone().iter() {
        // if file in rev_diff, skip
        if rev_diff.files.contains_key(file2) { continue; }

        let content2 = info2.get_content()?;

        // If not in rev_diff, that means it does not exist in rev1_manifest
        // So it is a new file
        rev_diff.files.insert(file2.clone(), file_diff("".to_string(), content2));
    }
    return Ok(rev_diff);
        
}

pub fn cat<'a>(wd: &'a str, rev_id:&'a str, path:&'a str) -> Result<String, Errors>{
    // find path in rev
    // return file content or error
    let repo = repository::load(wd)?;
    let rev = repo.get_rev(rev_id)?;
    let manifest = rev.get_manifest();
    let file_info = manifest.get(path);

    if file_info.is_none() {
        return Err(Errstatic("file not found"));
    }else{
        let file_info = file_info.unwrap();
        let content = file_info.get_content()?;
        return Ok(content);
    }
}

pub fn add<'a>(wd: &'a str, path:&'a str) -> Result<String, Errors>{
    println!("wd: {:?}", wd);
    let mut repo = repository::load(wd)?;
    
    let abs_path = path_compose(wd, path);
    println!("abs_path: {:?}", abs_path);
    let res = repo.add_file(&abs_path)?;
    Ok("add success".to_string())
}

pub fn remove<'a>(wd: &'a str, path:&'a str) -> Result<String, Errors>{
    // remove the file temporarily to the index branch by acting as if we have deleted the file (not committed yet)
    // just call repo.remove by obtaining absolute path
    unimplemented!(); //TODO
}

pub fn commit<'a>(wd: &'a str, message:&'a str) -> Result<RevDiff, Errors>{
    let mut repo = repository::load(wd)?;
    let head1 = repo.get_current_head()?;
    let rev_id1 = head1.get_id().unwrap();

    let commit_res = repo.commit()?;
    let head2 = repo.get_current_head()?;
    let rev_id2 = head2.get_id().unwrap();

    // commit the index branch to the head branch, create a new revision and update the head
    // write to log -> where would this be?

    // return a RevDiff if successful

    diff(wd, rev_id1, rev_id2)
}

pub fn merge<'a>(wd: &'a str, rev_id_source:&'a str, rev_id_dst:&'a str) -> Result<String, Errors>{
    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dsr;
    use crate::vc::repository::{self, init};

    fn create_files_and_commit1(){
        let _ = dsr::create_file("a.txt");
        let _ = dsr::write_file("a.txt", "file a\nhello world");
        let _ = dsr::create_file("b.txt");
        let _ = dsr::write_file("b.txt", "file b\nhello world");
    }

    #[test]
    fn test_diff() {
        // let _ = dsr::delete_dir(".dvcs");
        // let _ = init();
        let wd = get_wd_path();

        let a = add(&wd, "a.txt");
        println!("a: {:?}", a);
    }

    #[test]
    fn test_cat() {
        // TODO
    }

    #[test]
    fn test_add() {
        // let cwd = &path_compose(&get_wd_path(), "test_repo/");
        let cwd = "./test_repo/";
        
        let _ = dsr::delete_dir(&path_compose(cwd, ".dvcs"));
        let _ = dsr::delete_file(&path_compose(cwd, "a.txt"));

        let _ = init(Some(cwd));

        let _ = dsr::create_file(&path_compose(cwd, "a.txt"));
        let _ = dsr::write_file(&path_compose(cwd, "a.txt"), "hello world");
        
        // TODO: this add doesn't seem to add anything to repos file
        let add1 = add(&cwd, "a.txt");
        println!("add1: {:?}", add1);
        // // It doesn't actually add the file to the index branch?
        // assert!(add1.is_ok());

        // let nodef = add(&cwd, "nodef_file.txt");
        // println!("nodef: {:?}", nodef);
        // assert!(nodef.is_err());
    }

    #[test]
    fn test_remove() {
        // TODO
    }

    #[test]
    fn test_commit() {
        let cwd = "./test_repo";

        let _ = dsr::delete_dir(&path_compose(cwd, ".dvcs"));
        let _ = dsr::delete_file(&path_compose(cwd, "a.txt"));

        init(Some(cwd));

        let _ = dsr::create_file(&path_compose(cwd, "a.txt"));
        let _ = dsr::write_file(&path_compose(cwd, "a.txt"), "hello world");

        let com1 = commit(cwd, "test commit");
        println!("com1: {:?}", com1);
        assert!(com1.is_err());

        let _ = add(cwd, "a.txt");
        
        let com2 = commit(cwd, "test commit");
        println!("com2: {:?}", com2);
        assert!(com2.is_ok());
    }

    #[test]
    fn test_merge() {
        // TODO
    }

}