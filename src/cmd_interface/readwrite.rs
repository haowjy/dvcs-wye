use crate::ui::{Errors, Errors::*};
use crate::vc::{repository::{Repo, Stage}, revision::{Rev}};
use crate::vc::{repository};
use crate::dsr::{*, self};
use std::collections::{HashMap, VecDeque, HashSet};

use crate::cmd_function::{*};

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

// TODO: should work, but not tested cuz repo is not working atm
pub fn diff<'a>(wd: &'a str, rev1_id:&'a str, rev2_id:&'a str) -> Result<RevDiff, Errors>{
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

// TODO: test
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

// TODO: test
pub fn add<'a>(wd: &'a str, path:&'a str) -> Result<String, Errors>{
    println!("wd: {:?}", wd);
    let mut repo = repository::load(wd)?;
    
    let abs_path = path_compose(wd, path);
    println!("abs_path: {:?}", abs_path);
    let res = repo.add_file(&abs_path)?;
    Ok("add success".to_string())
}

// TODO: test
pub fn remove<'a>(wd: &'a str, path:&'a str) -> Result<String, Errors>{
    // remove the file temporarily to the index branch by acting as if we have deleted the file (not committed yet)
    // just call repo.remove by obtaining absolute path
    let mut repo = repository::load(wd)?;
    let abs_path = path_compose(wd, path);
    let _ = repo.remove_file(&abs_path)?;
    Ok("remove success".to_string())
}

fn find_conflict_files(stage:&Stage) -> Option<Vec<(String, String)>> {
    // let mut conflict_files = Vec::new();
    let conflict_files:Vec<(String, String)> = stage.get_add().iter()
    .filter_map(|(file, info)| {
        let content = info.get_content().unwrap();
        let res_unmerged = find_unmerged(content);
        if res_unmerged.is_ok() {return None;}
        Some((file.to_owned(), res_unmerged.unwrap_err()))
    }).collect();

    if conflict_files.len() > 0 {
        return Some(conflict_files);
    } else {
        return None;
    }
}

// TODO: test
pub fn commit<'a>(wd: &'a str, message:&'a str) -> Result<RevDiff, Errors> {
    let mut repo = repository::load(wd)?;
    let head1 = repo.get_current_head()?;
    let rev_id1 = head1.get_id().unwrap();

    // blocks if there are no changes
    let stage = repo.get_stage();
    if stage.is_empty() {
        return Err(Errstatic("no change"));
    }
    
    // block if we find a conflict
    let conflicted_files = find_conflict_files(stage);
    if conflicted_files.is_some() {
        conflicted_files.unwrap().iter().fold("".to_string(), |acc, (file, content)| {
            acc + &format!("conflict in file {}\n{}\n", file, content)
        });
        return Err(Errstatic("conflict found"));
    }

    repo.commit(message)?;
    let head2 = repo.get_current_head()?;
    let rev_id2 = head2.get_id().unwrap();

    // TODO: write to log
    // TODO: update head

    // commit the index branch to the head branch, create a new revision and update the head
    // write to log -> where would this be?

    // return a RevDiff if successful

    diff(wd, rev_id1, rev_id2)
}

// TODO: test
// merge from src to dst, dst must be named revisions tracked by the repo so we can have something to update
pub fn merge<'a>(wd: &'a str, rev_id_src:String,
//  rev_id_dst: String
) -> Result<String, Errors>{
    // let r1 = VC::Revision::from(rev1)
    // let r2 = VC::Revision::from(rev2)
    // uses conflict_find(content1, content2) on on content of r1.files, f2.files
    // DSR::write_file(wd+r1.files, r2.files, conflict_find results)
    // add()
    // merge_commit() [extended from commit] // blocks if there are conflicts

    
    let repo = repository::load(wd)?;
    let stage = repo.get_stage();

    let rev_dst = repo.get_current_head()?; // is current head

    // Check stage before merge into current HEAD
    if stage.get_add().is_empty() && stage.get_remove().is_empty() {
        return Err(Errstatic("Stage must be empty before merge"));
    }

    let rev1 = rev_dst.clone(); // Will only create conflict files if dst is the current head, otherwise will simply just return the errors
    let rev2 = repo.get_rev(&rev_id_src)?;

    let rev_origin = find_rev_lca(&repo, rev1.clone(), rev2.clone())?;
    let rev_diff1_files = diff(wd, rev_origin.get_id().unwrap(), rev1.clone().get_id().unwrap())?.files;
    let rev_diff2_files = diff(wd, rev_origin.get_id().unwrap(), rev2.clone().get_id().unwrap())?.files;

    let mut merged_files = HashMap::new();
    let mut merge_conflicts = HashMap::new();
    for (file, diff1) in rev_diff1_files {

        let diff2 = rev_diff2_files.get(file.as_str());
        if diff2.is_none() { continue; } // this would mean that the file DNE in rev2 (file added in rev1, but not in rev2) -> definitely no conflict
        let diff2 = diff2.unwrap();
        let conflict = conflict_find(diff1, diff2.clone())?;

        merged_files.insert(file.clone(), conflict.get_content());

        if conflict.is_conflict() {
            merge_conflicts.insert(file, conflict);
        }
    }

    if merge_conflicts.len() > 0 {
        // write the conflict files to the repo
        
        // write the conflict files to the repo
        for (file, new_content) in merged_files {
            let abs_path = path_compose(wd, file.as_str());
            let _ = dsr::write_file(&abs_path, &new_content);
            add(wd, &file)?;
        }
        return Err(ErrStr("Conflicts found, please resolve the conflicts and try to commit again".to_string()));
        
    } else {
        
        // write the merged files to the repo
        for (file, content) in merged_files {
            let abs_path = path_compose(wd, file.as_str());
            let _ = dsr::write_file(&abs_path, &content);
            add(wd, &file)?;
        }
        // merge_commit(); // TODO repo.merge_commit()

        return Ok("No conflicts, but cannot create new revision except in current head".to_string());
        
    }

    
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

        let _ = init(Some(cwd));

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