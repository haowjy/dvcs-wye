use crate::ui::{Errors, Errors::*};
use std::path::Path;
use std::io::Error;

use crate::dsr::*;
use crate::readwrite::*;


pub fn clone<'a>(wd: &'a str, remote:&'a str) -> Result<String, Errors> {
    copy_dir(remote, wd)?;

    let head = "HEAD"; // TODO: Connect to VC

    let co_res = checkout(wd, head, None);
    match co_res {
        Ok(_) => Ok("clone success".to_string()),
        Err(_) => Err(Errstatic("clone failed: checkout failed")),
    }
}

pub fn checkout<'a>(wd:&'a str, rev:&'a str, new_branch_alias: Option<String>) -> Result<String, Errors> {
    // NEW BRANCH DETATCH-HEAD, rev
    // set current head to DETATCH-HEAD
    let rev = rev; // TODO: Connect to VC

    let clear_resp = clear_dir(wd, vec![".dvcs"]);
    let _ : Result<&str, ()> = match clear_resp {
        Ok(_) => Ok("checkout success"),
        Err(_) => return Err(Errstatic("checkout failed: clear_dir failed")),
    };

    // if rev is a branch, then get the head of the branch
    // if the revision is detached, then create a new branch
    // TODO: for f in rev.get_files() -> DSR::create_file(f.get_path(), f.get_content())
    // DSR::make_wd

    Ok("checkout success".to_string())
}

pub fn pull<'a>(wd:&'a str, remote:&'a str) -> Result<String, Errors> {
    // check if wd and remote are directories and have same name

    // VC::Repo::load(wd)
    // VC::Repo.fetch(remote, head)
    // repo_remote
    // repo_remote.get_head("name")
    // create head remote/name
    // merge(wd, remote/name)

    // merge(wd, head, remote/head)
    unimplemented!(); //TODO
}

pub fn push<'a>(wd:&'a str, remote:&'a str) -> Result<String, Errors> {
    // check if wd and remote are directories and have same name
    
    // VC::Repo::load() // load wd and remote repos
    // diff(Repo.remote/head, remoteRepo.head) // if the remote tracked is different from what is actually on remote, then block and ask to pull
    let diff_res : Result<RevDiff, ()> = match diff(wd,"curRepo.remote/head", "remoteRepo.head") {
        Ok(repo_diff) => Ok(repo_diff),
        Err(_) => return Err(Errstatic("push failed: diff failed")),
    };
    
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
