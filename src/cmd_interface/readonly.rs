use std::fs::read;
use crate::cmd_function::{file_diff, FileDiff};
use crate::dsr;
use crate::dsr::get_files;
use crate::vc::{file, repository, revision};
use crate::vc::revision::Rev;
use crate::ui::Errors;

pub fn heads(wd: &str) -> Result<Vec<String>, Errors> {
    let load = repository::load(wd)?;//get Repo//Result<Repo, Errors>//if error return directly
    let head = load.get_heads();//&Hashmap//waiting
    let mut res=Vec::new();
    let mut string=String::new();
    for(key,value) in head{
        string=key.to_owned()+":"+value;
        res.push(string);
    }
    return if head.is_empty() { Err(Errors::Errstatic("No heads found")) } else { Ok(res) }
}

pub fn log(wd: &str) -> Result<Option<Vec<String>>, Errors> {//alias,rev_id: &str
    let mut string=Vec::new();
    let load = repository::load(wd)?;//got Repo
    let mut hashmap;
    let mut new_rev;
        let current_head = load.get_current_head()?;//
        hashmap=current_head.get_log();//here get hashmap, this is log need print, maybe put into Vec<String>
        for(key,value) in hashmap{
            string.push(key.to_owned()+ ":"+ &value);
        }
        let mut parent_head = "";
        let parent_head_pre = current_head.get_parent_id();
        if  parent_head_pre.is_none(){ return Ok(Some(string));}
        else {parent_head=parent_head_pre.unwrap(); }

        new_rev = load.get_rev(parent_head).unwrap();
    while !new_rev.get_parent_id().is_none() {
        new_rev = load.get_rev(new_rev.get_parent_id().unwrap()).unwrap();
        hashmap=new_rev.get_log();//here get hashmap, this is log need print, maybe put into Vec<String>
        for(key,value) in hashmap{
            string.push(key.to_owned()+":"+&value);
        }
    }
    Ok(Some(string))
    //first current head->know parent head-> get_rev(return Revision碰到revision就读Parent id->until None
}

pub fn status(wd: &str) -> Result<(Vec<String>, Vec<String>, Vec<String>), Errors> {
    let load=repository::load(wd)?;//got Repo
    let stage=load.get_stage();// got stage
    let stage_inside_add=stage.get_add();
    let stage_inside_remove=stage.get_remove();
    //modified:   a.txt
    //add
    //ahead of ?commit
    //up to date
    //   println!("Changes to be committed:");
    //already add file but modified, so just need commit
    //commit delete stage, last commit
//    println!("Changes not staged for commit:");
    //need add first, then commit
    //
    //last commit has, stage has in add, but not commit yet, so not in wd
    //   println!("Untracked files:");
    //working directory has, stage don't have. last commit has not,never shows forever
    //
    let mut list_files:Vec<String> = vec![];
    let mut ignore:Vec<&str> = vec![];
    ignore.push(".dvcs");
    ignore.push("target");
    ignore.push("src");
    ignore.push(".idea");
    ignore.push(".DS_Store");
    ignore.push(".git");
    get_files(wd,ignore,&mut list_files);//file from fd
    //compare between wd and stage and last commit
    //println!("{:?}",list_files);
    let mut untrack:Vec<String>=vec![];
    let mut Changes_to_be_committed:Vec<String>=vec![];
    let mut Changes_not_staged_for_commit:Vec<String>=vec![];
    let a =list_files.iter().fold(0,|acc,x1| {
        let wd_item=dsr::read_file_as_string(x1).unwrap();
        let mut stage_status =false;
        let name=dsr::get_name(x1).unwrap();
        let contain_add= stage_inside_add.contains_key(&name);

        let contain_remove= stage_inside_remove.contains_key(&name);
        /*if contain_remove { println!("wd has, stage has");}
        else { println!("wd has, stage has not->maybe untrack file{}",x1); }*/


        //compare stage with wd, not stage commit
        // if diff,need to add first
         if contain_add==true {
             let contentoofstage=load.get_file_content(stage_inside_add.get(&name).unwrap()).unwrap();
             if wd_item!=contentoofstage{
                 Changes_not_staged_for_commit.push("Modified file: ".to_owned()+&name);
                 stage_status=true;
             }

         }
        if contain_remove==true {
            let contentoofstage=load.get_file_content(stage_inside_remove.get(&name).unwrap()).unwrap();
            if wd_item!=contentoofstage{
                Changes_not_staged_for_commit.push("Modified file: ".to_owned()+&name);
                stage_status=true;
            }
        }

        let last_commit= load.get_current_head();//Rev
        let mut contain_last_commit =false;
        if  last_commit.is_err(){ //println!("no head, means last commit is empty");
        }
        else
        {//contain_last_commit
            let last_commit_file=last_commit.unwrap();
            let last_commit_hashmap=last_commit_file.get_manifest();//iteminfo
            contain_last_commit= last_commit_hashmap.contains_key(&name);
            if contain_last_commit {
                //println!("wd has, last commit has");
//stage has, wd has, last commit has !!!!!! and stage_status=false
                //stage
                //compare stage and last commit, if same->no change
                //if not same->modified
                if contain_add==true && contain_last_commit==true&&stage_status==false {
                    println!("Modified file{}",name);
                    //compare inside content see modify
                    let it1= last_commit_hashmap.get(&name).unwrap();
                    let it2= stage_inside_add.get(&name).unwrap();
                        if it1.eq(it2) {
//same
                        }
                        else {
                            //diff
                            let a=load.get_file_content(it2);//stage
                            let b=load.get_file_content(it1);//last_commit
                            let diff = file_diff(a.unwrap(), b.unwrap()).clone();
                            let flag= diff.is_diff();
                            if flag==true {
                                Changes_to_be_committed.push("Modified file: ".to_owned()+&name);
                            }
                            else { println!("No difference, same"); }
                        }
                }
            }//contain_last_commit true
            else { //println!("wd has, last commit has not->maybe untrack file{}",x1);
            }
        }
        //so
        if contain_add==false && contain_remove==false && contain_last_commit==false{
            untrack.push(name);
        } else if contain_add==true && contain_last_commit==false {
            Changes_to_be_committed.push("Modified/Add new file: ".to_owned()+&name);
            //println!("Add new file{}",name);
            //Changes to be committed:
        }
        else if contain_remove==true && contain_last_commit==false {
            Changes_to_be_committed.push("Remove file: ".to_owned()+&name);
            //Changes to be committed:
        }
        else if contain_remove==true && contain_last_commit==true { println!("Remove/Modified file{}",name);
            //compare inside content see modify
        }
        0});
    //wd has not，stage and commit？//stage has, last  delete
    for(key, value)in stage_inside_remove{
        let contain_delete=list_files.contains(key);
        if  contain_delete==false{
            let last_commit_again= load.get_current_head();//Rev
            let mut contain_last_commit_again =false;
            if  last_commit_again.is_err(){ //println!("no head, means last commit is empty");
            }
            else
            {
                let last_commit_file_again=last_commit_again.unwrap();
                let last_commit_hashmap=last_commit_file_again.get_manifest();//iteminfo
                contain_last_commit_again= last_commit_hashmap.contains_key(key);
                if contain_last_commit_again {//wd no, stage yes, last commit yes
                    Changes_not_staged_for_commit.push("Delete file: ".to_owned()+key);
                }
                else { //wd no,stage yes, last commit yes
                    Changes_to_be_committed.push("Delete file: ".to_owned()+key);
                     }
            }


        }


    }

    println!("Changes to be committed:");
    if  Changes_to_be_committed.capacity()==0{ println!("nothing to change");}
    else{
        Changes_to_be_committed.iter().fold(0,|acc,x|{
            println!("{:?}",x);
        0});
        }
    //already add file but modified, so just need commit
    //commit delete stage, last commit
    println!("Changes not staged for commit:");
    if  Changes_not_staged_for_commit.capacity()==0{ println!("nothing to change");}
    else{
        Changes_not_staged_for_commit.iter().fold(0,|acc,x|{
            println!("{:?}",x);
            0});
        //println!("{:?}",Changes_not_staged_for_commit);
    }
    //need add first, then commit
    //
    //last commit has, stage has in add, but not commit yet, so not in wd
    println!("Untracked files:");
    if  untrack.capacity()==0{ println!("nothing to change");}
    else{
        untrack.iter().fold(0,|acc,x|{
            println!("{:?}",x);
            0});
        //println!("{:?}",untrack);
    }
    Ok((Changes_to_be_committed,Changes_not_staged_for_commit,untrack))
}

#[cfg(test)]
mod test {
    use crate::dsr;
    use crate::readwrite::{add, commit};
    use crate::vc::repository::init;
    use super::*;

    #[test]
    fn test_heads() {
        let wd = dsr::get_wd_path();
        init(None);
        add(&wd,"src");
        let res = heads(&wd).is_err();
        assert_eq!(res, true);
    }
    #[test]
    fn test_heads_2() {
        let wd = dsr::get_wd_path();
        init(None);
        add(&wd,"src");
        commit(&wd,"test");
        let res = heads(&wd).is_err();
        assert_eq!(res, true);
    }

    #[test]
    fn test_logs() {
        let wd = dsr::get_wd_path();
        let res = log(&wd);
        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_status() {
        let wd = dsr::get_wd_path();
        let res = status(&wd);
        assert_eq!(res.is_ok(), true);
        //assert_eq!(res.unwrap(),"ok");
    }
}
