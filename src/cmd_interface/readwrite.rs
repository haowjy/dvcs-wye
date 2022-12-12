// use crate::readonly::status;
use crate::ui::{Errors, Errors::*};
use crate::vc::{repository::{Repo, Stage}};
use crate::vc::{repository};
use crate::dsr::{*, self};
use std::collections::{HashMap,};

use crate::cmd_function::{*};
use crate::cmd_function::FileDiffType::{*};

// ---------- PRIVATE ----------
fn find_conflict_files(repo: &Repo, stage:&Stage) -> Result<Option<Vec<(String, String)>>, Errors> {

    let mut conflict_files = Vec::new();

    for (file, info) in stage.get_add() {
        let content = repo.get_file_content(info)?;
        let res_unmerged = find_unmerged(content.clone());
        if res_unmerged.is_ok() {continue;}
        conflict_files.push((file.to_owned(), res_unmerged.unwrap_err()));
    }

    if conflict_files.len() > 0 {
        return Ok(Some(conflict_files));
    } else {
        return Ok(None);
    }
}
// ---------- END PRIVATE ----------

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
            res.push_str(&format!("{}\n{:?}\n{}\n", file, diff.get_diff_type(),diff.to_string()));
        }
        res
    }
}

// 8. diff
pub fn diff<'a>(wd: &'a str, rev1_id:&'a str, rev2_id:&'a str) -> Result<RevDiff, Errors>{
    let mut rev_diff = RevDiff::new();

    let repo = repository::load(wd)?;

    let rev1 = repo.get_rev(rev1_id)?;
    let rev2 = repo.get_rev(rev2_id)?;
    
    let rev1_manifest = rev1.get_manifest();
    let rev2_manifest = rev2.get_manifest();

    for (file1, info1) in rev1_manifest.clone() {
        let content1 = repo.get_file_content(&info1)?;

        let file2_opt = rev2_manifest.get(&file1);
        if file2_opt.is_none() { // file deleted
            let file_diff = file_diff(content1, "".to_string());
            rev_diff.files.insert(file1.clone(), file_diff);
        } else { // file modified or unchanged
            let content2 = repo.get_file_content(file2_opt.unwrap())?;
            rev_diff.files.insert(file1.clone(), file_diff(content1, content2));
        }
    }
    for (file2, info2) in rev2_manifest.clone().iter() {
        // if file in rev_diff, skip
        if rev_diff.files.contains_key(file2) { continue; }

        let content2 = repo.get_file_content(info2)?;

        // If not in rev_diff, that means it does not exist in rev1_manifest
        // So it is a new file
        rev_diff.files.insert(file2.clone(), file_diff("".to_string(), content2));
    }
    return Ok(rev_diff);
        
}

// 9. cat
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
        let content = repo.get_file_content(file_info)?;
        return Ok(content);
    }
}

// 10. add
pub fn add<'a>(wd: &'a str, path:&'a str) -> Result<String, Errors>{
    let mut repo = repository::load(wd)?;
    
    let abs_path = path_compose(wd, path);
    repo.add_file(&abs_path)?;
    Ok("add success".to_string())
}

// 11. remove
pub fn remove<'a>(wd: &'a str, path:&'a str) -> Result<String, Errors>{
    // remove the file temporarily to the index branch by acting as if we have deleted the file (not committed yet)
    // just call repo.remove by obtaining absolute path
    let mut repo = repository::load(wd)?;
    let abs_path = path_compose(wd, path);
    let _ = repo.remove_file(&abs_path)?;
    dsr::delete_file(&abs_path)?;
    Ok("remove success".to_string())
}

// 12. commit
pub fn commit<'a>(wd: &'a str, message:&'a str) -> Result<String, Errors> {
    let mut repo = repository::load(wd)?;

    // blocks if there are no changes
    let stage = repo.get_stage();
    if stage.is_empty() {
        return Err(Errstatic("no change"));
    }
    
    // block if we find a conflict
    let conflicted_files = find_conflict_files(&repo, stage)?;
    if conflicted_files.is_some() {
        conflicted_files.unwrap().iter().fold("".to_string(), |acc, (file, content)| {
            acc + &format!("conflict in file {}\n{}\n", file, content)
        });
        return Err(Errstatic("conflict found"));
    }

    repo.commit(message)?; // creates new head called "main" if initial commit
    
    let head2 = repo.get_current_head()?;
    let rev_id2 = head2.get_id().unwrap(); // new rev
    return Ok(rev_id2.to_string());
    
}

// 13. merge from src into the current head of wd
pub fn merge<'a>(wd: &'a str, rev_id_src:String,
//  rev_id_dst: String
) -> Result<String, Errors>{

    let mut repo = repository::load(wd)?;

    let rev_dst = repo.get_current_head()?; // is current head

    // Check if there are uncommitted changes (checks only stage instead)
    if !repo.get_stage().is_empty(){ return Err(Errstatic("pull failed: uncommitted changes in working directory, commit changes first"));}
    // TODO: status doesn't work???
    // let (staged, unstaged, untracked) = status(wd)?; // print status
    // if !(staged.is_empty() && unstaged.is_empty() && untracked.is_empty()){ // not empty
    //     return Err(Errstatic("merge failed: uncommitted changes"));
    // }

    let cur_head_rev = rev_dst.clone(); // Will only create conflict files if dst is the current head, otherwise will simply just return the errors
    let rev2 = repo.get_rev(&rev_id_src)?;

    if cur_head_rev.get_id().unwrap() == rev2.get_id().unwrap() { 
        return Ok("already up to date".to_string());
    }

    let rev_origin = find_rev_lca(&repo, cur_head_rev.clone(), rev2.clone())?;
    let rev_diff1_files = diff(wd, rev_origin.get_id().unwrap(), cur_head_rev.clone().get_id().unwrap())?.files;
    let rev_diff2_files = diff(wd, rev_origin.get_id().unwrap(), rev2.clone().get_id().unwrap())?.files;

    let mut merged_files = HashMap::new(); // file, content
    let mut merge_conflicts = HashMap::new();
    for (file, diff1) in rev_diff1_files.clone() {

        let diff2 = rev_diff2_files.get(file.as_str());
        if diff2.is_none() { // this would mean that the file DNE in rev2 (file added in rev1, but not in rev2) -> definitely no conflict
            merged_files.insert(file.clone(), diff1.get_modified());
            continue; 
        } 
        let diff2 = diff2.unwrap();
        let conflict = conflict_find(diff1, diff2.clone())?;

        merged_files.insert(file.clone(), conflict.get_content());

        if conflict.is_conflict() {
            merge_conflicts.insert(file, conflict);
        }
    }

    for (file, diff2) in rev_diff2_files.clone() {
        let diff1 = rev_diff1_files.get(file.as_str());
        if diff1.is_some() { continue; } // we have already covered the case where diff1 and diff2 exist

        merged_files.insert(file.clone(), diff2.get_modified());
    }

    if merge_conflicts.len() > 0 {
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
            repo.add_file(&abs_path)?; // don't use add() because we need it to update here
        }

        repo.merge_commit(cur_head_rev.get_id().unwrap(), rev2.get_id().unwrap(), Some(&format!("merge {} into {}", cur_head_rev.get_id().unwrap(), rev2.get_id().unwrap())))?;

        // let repo = repository::load(wd)?;
        let cur_head = repo.get_current_head()?;
        let cur_rev_id = cur_head.get_id().unwrap(); // new rev
        return Ok(cur_rev_id.to_string()); // The merge commit is the new head
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::createonly::checkout;
    use crate::dsr;
    use crate::vc::repository::{self, init};
    use crate::test_help::{*};
    

    #[test]
    fn test_diff_1() {
        let cwd = "./a_test_repo/";

        remove_git_and_init(cwd);

        let rev1 = create_files_and_commit_ab1(cwd);
        let rev2 = write_create_files_and_commit_abc2(cwd);

        let diff = diff(cwd, rev1.as_str(), rev2.as_str()).unwrap();
        // println!("diff:\n{:}", diff.to_string());
        assert_eq!(diff.get_files().get("a.txt").unwrap().get_diff_type(), &Modified);
        assert_eq!(diff.get_files().get("b.txt").unwrap().get_diff_type(), &Modified);
        assert_eq!(diff.get_files().get("c.txt").unwrap().get_diff_type(), &Added);
    }

    #[test]
    fn test_cat_2() {
        let cwd = "./a_test_repo/";

        remove_git_and_init(cwd);

        let rev1 = create_files_and_commit_ab1(cwd);
        
        let rev2 = write_create_files_and_commit_abc2(cwd);
        let a = cat(cwd, &rev1, "a.txt").unwrap();
        let c = cat(cwd, &rev1, "c.txt");
        assert_eq!(a, "hello world");
        assert!(c.is_err()); // c doesn't exist in rev1

        let a2 = cat(cwd, &rev2, "a.txt").unwrap();
        let c2 = cat(cwd, &rev2, "c.txt").unwrap();
        assert_eq!(a2, "A Change");
        assert_eq!(c2, "hello world from C");
    }

    #[test]
    fn test_add_3() {
        let cwd = "./a_test_repo/";
        
        remove_git_and_init(cwd);

        create_files_ab(cwd);
        
        let add1 = add(&cwd, "a.txt");
        // It doesn't actually add the file to the index branch?
        assert!(add1.is_ok());

        let nodef = add(&cwd, "nodef_file.txt");
        assert!(nodef.is_err());
    }

    #[test]
    fn test_remove_4() {
        let cwd = "./a_test_repo/";

        remove_git_and_init(cwd);

        let rev1 = create_files_and_commit_ab1(cwd);
        let rm1 = remove(cwd, "a.txt");
        assert!(rm1.is_ok());
        let is_in_to_rm_stage = repository::load(cwd).unwrap().get_stage().get_remove().contains_key("a.txt");
        assert!(is_in_to_rm_stage);

        let rm2 = remove(cwd, "a.txt");
        assert!(rm2.is_err()); // already removed

        let rev2 = commit(cwd, "remove a.txt").unwrap();
        let d = diff(cwd, rev1.as_str(), rev2.as_str()).unwrap();
        assert_eq!(d.get_files().get("a.txt").unwrap().get_diff_type(), &Deleted);
    }

    #[test]
    fn test_commit_5() {
        let cwd = "./a_test_repo";
        
        remove_git_and_init(cwd);

        create_files_ab(cwd);

        let com1 = commit(cwd, "test commit");
        assert!(com1.is_err()); // error because no files added to stage

        let _ = add(cwd, "a.txt");
        
        let com2 = commit(cwd, "test commit add A");
        assert!(com2.is_ok());

        let com3 = commit(cwd, "test commit");
        assert!(com3.is_err()); // error because nothing to commit

        let _ = add(cwd, "b.txt");
        let com4 = commit(cwd, "test commit add B");
        assert!(com4.is_ok());
    }

    #[test]
    fn test_merge_6() {
        let cwd = "./a_test_repo";
        remove_git_and_init(cwd);

        let rev1 = create_files_and_commit_ab1(cwd);

        checkout(cwd, rev1.as_str(), Some("otherbranch".to_string())).unwrap();

        let rev2 = write_create_files_and_commit_abc2(cwd); // on other branch, make some changes
        checkout(cwd, "main", None).unwrap(); // back to main

        let rev3 = merge(cwd, rev2);
        assert!(rev3.is_ok());

        let repo = repository::load(cwd).unwrap();
        let heads = repo.get_heads();
        assert_eq!(heads.get("main").unwrap(), &rev3.unwrap());
    }

    #[test]
    fn test_merge_conflict_7() {
        let cwd = "./a_test_repo";
        remove_git_and_init(cwd);

        let rev1 = create_files_and_commit_ab1(cwd);

        let _ = write_create_files_and_commit_abc2(cwd); // make additional changes on main

        checkout(cwd, &rev1, Some("otherbranch".to_string())).unwrap(); // make branch from rev1
        let _ = write_files_edit_and_commit_ab3(cwd); // make some changes on main

        let merg = merge(cwd, "main".to_string()); // merge main into otherbranch
        assert!(merg.is_err());

        let atxt = dsr::read_file_as_string(&path_compose(cwd, "a.txt")).unwrap();
        assert_eq!(atxt.as_str(), "<<<<<<< ours\nInsertFirst\nA Change 2\n||||||| original\nhello world\n=======\nA Change\n>>>>>>> theirs\n"); // a.txt should have both changes

        let c = commit(cwd, "merge"); // commit the merge will fail because it finds conflict pattern
        assert_eq!(c.is_err(), true);
    }

}