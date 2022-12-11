use crate::ui::{Errors, Errors::*};
use crate::vc::repository::{Repo};
use crate::vc::{repository};
use std::path::Path;
use std::io::Error;

use crate::dsr::*;
use crate::readwrite::*;


pub fn clone<'a>(wd: &'a str, remote:&'a str) -> Result<String, Errors> {
    let dvcs_cwd = path_compose(wd, ".dvcs");
    let dvcs_remote = path_compose(remote, ".dvcs");
    if is_path_valid(&dvcs_cwd){
        return Err(Errstatic("clone failed: repo already exists"));
    }
    if !is_path_valid(&dvcs_remote){
        return Err(Errstatic("clone failed: remote repo does not exist"));
    }

    let remote_repo = repository::load(remote)?;
    let head_alias = remote_repo.get_current_head_alias();
    if head_alias.is_none() {
        return Err(Errstatic("clone failed: no head found")); // will get the same head as the remote
    }

    copy_dir(&dvcs_remote, &dvcs_cwd)?;

    // create remote tracking branch
    checkout(wd, head_alias.unwrap(), Some("remote/".to_owned()+head_alias.unwrap()))?;
    // checkout the on cwd 
    checkout(wd, head_alias.unwrap(), None)
}

pub fn checkout<'a>(wd:&'a str, rev:&'a str, new_branch_alias: Option<String>) -> Result<String, Errors> {
    // NEW BRANCH DETATCH-HEAD, rev
    // set current head to DETATCH-HEAD
    let mut repo_mut = repository::load(wd)?;
    let repo = repository::load(wd)?;
    let branch_heads = repo.get_heads();
    let new_head_id = branch_heads.get_key_value(rev);

    clear_dir_adv(wd, vec![".dvcs", "src", "Cargo.toml"])?; // NOTE: this is potentially dangerous when using it in default directory
    if new_head_id.is_some() {
        let (rev_alias, rev_id) = new_head_id.unwrap();
        if new_branch_alias.is_some() { // we will create a new branch
            let new_branch_alias = new_branch_alias.unwrap();
            if branch_heads.contains_key(&new_branch_alias) {
                return Err(Errstatic("checkout failed: branch already exists"));
            }
            let rev1 = repo.get_rev(rev_id)?;
            make_wd(&rev1, wd)?;

            repo_mut.new_head(&new_branch_alias, rev_id)?; // create new branch
            repo_mut.set_current_head(&new_branch_alias)?; // set current head to new branch
        } else { // we are not creating a new branch, just switching to an existing one
            let rev1 = repo.get_rev(rev_id)?;
            make_wd(&rev1, wd)?;

            repo_mut.set_current_head(&rev_alias)?; // is also saved
        }
    }else{
        if new_branch_alias.is_some() { // we will create a new branch with the rev as rev_id
            let new_branch_alias = new_branch_alias.unwrap();
            if branch_heads.contains_key(&new_branch_alias) {
                return Err(Errstatic("checkout failed: branch already exists"));
            }
            let rev1 = repo.get_rev(rev)?;
            make_wd(&rev1, wd)?;

            repo_mut.new_head(&new_branch_alias, rev)?; // create new branch
            repo_mut.set_current_head(&new_branch_alias)?; // set current head to new branch
        } else { // no branch alias, no new branch, just make a detached head
            let rev1 = repo.get_rev(rev)?;
            make_wd(&rev1, wd)?;

            repo_mut.new_head("DETACHED-HEAD", rev)?; // is also saved
            repo_mut.set_current_head("DETACHED-HEAD")?; // is also saved
        }
    }

    Ok("checkout success".to_string())
}

pub fn pull<'a>(wd:&'a str, remote:&'a str) -> Result<String, Errors> {
    // check if wd and remote are directories and have same name
    if get_name(wd) != get_name(remote){
        return Err(Errstatic("pull failed: wd and remote do not have the same working directory name, either rename working directory or use a different directory"));
    } 
    if wd == remote {
        return Err(Errstatic("pull failed: working and remote are the same directory"));
    }
    let mut cur_repo_mut = repository::load(wd)?;
    let cur_repo = repository::load(wd)?;
    let cur_head_alias = cur_repo.get_current_head_alias();
    if cur_head_alias.is_none() {return Err(Errstatic("pull failed: no head found"));}
    let cur_head_alias = cur_head_alias.unwrap();

    if !cur_repo.get_stage().is_empty(){ // not empty
        return Err(Errstatic("pull failed: uncommitted changes in working directory, commit changes first"));
    }

    let remote_repo = repository::load(remote)?;
    let remote_heads = remote_repo.get_heads();
    if !remote_heads.contains_key(cur_head_alias) {
        return Err(ErrStr(format!("pull failed: remote repo does not have the current head {}", cur_head_alias)));
    }

    cur_repo.fetch(remote)?; // copies files from remote to wd
    // create new branch remote/cur_head_alias
    let remote_rev_id = remote_heads.get(cur_head_alias);
    cur_repo_mut.new_head(format!("remote/{}",cur_head_alias).as_str(), remote_rev_id.unwrap())?;
    
    // merge cur_head_alias, remote/cur_head_alias
    merge(wd, format!("remote/{}",cur_head_alias))
}

pub fn push<'a>(wd:&'a str, remote:&'a str) -> Result<String, Errors> {
    // check if wd and remote are directories and have same name
    if get_name(wd) != get_name(remote){
        return Err(Errstatic("pull failed: wd and remote do not have the same working directory name, either rename working directory or use a different directory"));
    }
    if wd == remote {
        return Err(Errstatic("pull failed: working and remote are the same directory"));
    }

    let cur_repo = repository::load(wd)?;
    let remote_repo = repository::load(remote)?;

    // remote_repo.fetch(cur_repo) // fetch on the remote side, but don't need to merge

    // // VC::Repo::load() // load wd and remote repos
    // // diff(Repo.remote/head, remoteRepo.head) // if the remote tracked is different from what is actually on remote, then block and ask to pull
    // let diff_res : Result<RevDiff, ()> = match diff(wd,"curRepo.remote/head", "remoteRepo.head") {
    //     Ok(repo_diff) => Ok(repo_diff),
    //     Err(_) => return Err(Errstatic("push failed: diff failed")),
    // };
    
    // if diff_res.some_diff() {
    //     return Err("push failed: remote is not up to date");
    // }
    
    // VC::Repo.fetch(wd, head) // fetch on the remote side

    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {

    use super::*;
    
    use crate::dsr;

    #[test]
    fn test_clone() {
        // TODO: can replace with init or fs::?
        let _ = dsr::delete_dir("remoterepo/remote/.dvcs");
        let _ = dsr::delete_dir("local/.dvcs");

        let _ = dsr::create_dir("remoterepo/remote/.dvcs");
        let _ = dsr::create_file("remoterepo/remote/.dvcs/HEAD");
        let _ = dsr::write_file("remoterepo/remote/.dvcs/HEAD", "Stuff");

        let res = clone("local", "remoterepo/remote");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), "clone success");
    }
}
