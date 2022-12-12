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
    // checkout(wd, head_alias.unwrap(), Some("remote/".to_owned()+head_alias.unwrap()))?;
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

    clear_dir_adv(wd, vec![".dvcs", "src"])?; // NOTE: this is potentially dangerous when using it in default directory

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
            Ok(format!("checkout successful: currently on `{}`", new_branch_alias))
        } else { // we are not creating a new branch, just switching to an existing one
            let rev1 = repo.get_rev(rev_id)?;
            make_wd(&rev1, wd)?;

            repo_mut.set_current_head(&rev_alias)?; // is also saved
            Ok(format!("checkout successful: currently on `{}`", rev_alias))
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
            Ok(format!("checkout successful: currently on `{}`", new_branch_alias))
        } else { // no branch alias, no new branch, just make a detached head
            let rev1 = repo.get_rev(rev)?;
            make_wd(&rev1, wd)?;

            repo_mut.new_head("DETACHED-HEAD", rev)?; // is also saved
            repo_mut.set_current_head("DETACHED-HEAD")?; // is also saved
            Ok(format!("checkout successful: currently on `DETACHED-HEAD`"))
        }
    }
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
    if !cur_repo.get_stage().is_empty(){ return Err(Errstatic("pull failed: uncommitted changes in working directory, commit changes first"));}

    let cur_head_alias = cur_repo.get_current_head_alias();
    if cur_head_alias.is_none() {return Err(Errstatic("pull failed: no head found"));}
    let cur_head_alias = cur_head_alias.unwrap();

    let remote_repo = repository::load(remote)?;
    let remote_heads = remote_repo.get_heads();
    let head = remote_heads.get(cur_head_alias);
    if head.is_some() { // remote has the same branch, check if up to date
        let head = head.unwrap();
        let cur_rev = cur_repo.get_rev(cur_head_alias)?;
        let remote_rev = remote_repo.get_rev(head)?;
        if cur_rev.get_id() != remote_rev.get_id() {
            return Err(Errstatic("push failed: remote branch is not up to date. Please pull first"));
        }
    }

    checkout(wd, format!("remote/{}",cur_head_alias).as_str(), None)?;
    let m = merge(wd, cur_head_alias.to_string()); // NOTE: there shouldn't be any conflicts unless the user tries to checkout a remote tracking branch. 
    checkout(wd, cur_head_alias, None)?; // back to original branch
    if m.is_err() {
        return Err(Errstatic("push failed: merge failed. Something unexpected went wrong."));
    }

    remote_repo.fetch(wd)?; // copies files from wd to remote, then they can pull

    Ok("push success".to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    
    use crate::dsr;

    use crate::vc::repository::{self, init};

    use crate::test_help::{*};

    #[test]
    fn test_clone() {
        let remote_wd = "./a_test_remote";
        remove_git_and_init(remote_wd);
        create_files_and_commit_ab1(remote_wd);

        let cwd = "./a_test_repo";
        let _ = dsr::clear_dir(cwd, vec![]);

        let res = clone(cwd, remote_wd);
        assert!(res.is_ok());

        let res2 = clone(cwd, remote_wd);
        assert_eq!(res2.is_err(), true);
    }

    #[test]
    fn test_clone_remote_no_change() {
        let remote_wd = "./a_test_remote";
        remove_git_and_init(remote_wd);
        let rev1 = create_files_and_commit_ab1(remote_wd);

        let cwd = "./a_test_repo";
        let _ = dsr::clear_dir(cwd, vec![]);

        let res = clone(cwd, remote_wd);
        assert!(res.is_ok());

        let rev2 = write_create_files_and_commit_abc2(cwd);
        let repo = repository::load(cwd).unwrap();
        assert_eq!(repo.get_current_head().unwrap().get_id().unwrap(), rev2);

        let remote_repo = repository::load(remote_wd).unwrap();
        assert_eq!(remote_repo.get_current_head().unwrap().get_id().unwrap(), rev1); // different than local
    }

    #[test]
    fn test_checkout() {
        let cwd = "./a_test_repo";
        remove_git_and_init(cwd);
        let rev1 = create_files_and_commit_ab1(cwd);
        let rev2 = write_create_files_and_commit_abc2(cwd);

        let repo = repository::load(cwd).unwrap();
        assert_eq!(repo.get_current_head_alias().unwrap(), "main"); // head is named main initially
        let head2 = repo.get_current_head().unwrap();
        let cur_head = head2.get_id();
        assert_eq!(cur_head.unwrap(), rev2);

        let res = checkout(cwd, rev1.as_str(), None);
        assert_eq!(res.is_ok(), true);
        let repo = repository::load(cwd).unwrap();

        assert_eq!(repo.get_current_head_alias().unwrap(), "DETACHED-HEAD"); // head is detached
        let head1 = repo.get_current_head().unwrap();
        let cur_head = head1.get_id();
        assert_eq!(cur_head.unwrap(), rev1);
    }

    #[test]
    fn test_checkout_new_branch() {
        let cwd = "./a_test_repo";
        remove_git_and_init(cwd);
        let rev1 = create_files_and_commit_ab1(cwd);
        let _ = write_create_files_and_commit_abc2(cwd);

        let res = checkout(cwd, rev1.as_str(), Some("old_rev1".to_string()));
        assert_eq!(res.is_ok(), true);

        let repo = repository::load(cwd).unwrap();
        assert_eq!(repo.get_current_head_alias().unwrap(), "old_rev1"); // head is detached
        let head1 = repo.get_current_head().unwrap();
        let cur_head = head1.get_id();
        assert_eq!(cur_head.unwrap(), rev1);

        let res = checkout(cwd, "main", None); // back to main
        assert_eq!(res.is_ok(), true);
        let repo = repository::load(cwd).unwrap();
        assert_eq!(repo.get_current_head_alias().unwrap(), "main"); // head is detached
    }

    #[test]
    fn test_checkout_new_branch_fail() {
        let cwd = "./a_test_repo";
        remove_git_and_init(cwd);
        let rev1 = create_files_and_commit_ab1(cwd);
        let _ = write_create_files_and_commit_abc2(cwd);

        let res = checkout(cwd, rev1.as_str(), Some("main".to_string()));
        assert_eq!(res.is_err(), true);

        let res = checkout(cwd, "wrong", None);
        println!("{:?}", res);
        assert_eq!(res.is_err(), true);

        let repo = repository::load(cwd).unwrap();
        assert_eq!(repo.get_current_head_alias().unwrap(), "main"); // did not change
    }

    #[test]
    fn test_pull() {
        let remote_wd = "./a_remote/a_test_repo";
        remove_git_and_init(remote_wd);
        let rev1 = create_files_and_commit_ab1(remote_wd);

        let cwd = "./a_test_repo";
        let _ = dsr::clear_dir(cwd, vec![]);

        let _ = clone(cwd, remote_wd);

        write_create_files_and_commit_abc2(remote_wd);

        let res = pull(cwd, remote_wd);
        print!("{:?}", res);
        // TODO
    }

    #[test]
    fn test_push() {
        // TODO
    }
}
