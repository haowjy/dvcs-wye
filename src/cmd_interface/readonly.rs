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

pub fn log(wd: &str,rev_id: &str) -> Result<Option<Vec<String>>, Errors> {//alias,rev_id: &str
    let mut string=Vec::new();
    let load = repository::load(wd)?;//got Repo
    let mut hashmap;
    let mut new_rev;
    if rev_id.is_empty() {
        //normal
        println!("empty rev_id");
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
    } else {
        //
        println!("not empty rev_id");
        new_rev = load.get_rev(rev_id).unwrap();
    }
    while new_rev.get_parent_id().is_none() {
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
    //let res=(Vec<String>, Vec<String>, Vec<String>);
    let load=repository::load(wd)?;//got Repo
    //let renew=load.get_current_head()?;//get Rev
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
        println!("{}",x1);
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
            if contain_last_commit { println!("wd has, last commit has");
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
                            let a=load.get_file_content(it2);//stage
                            let b=load.get_file_content(it1);//last_commit
                        }
                        let diff = file_diff("wd_rev.unwrap().get_content().unwrap()".to_string(), "ItemInfo.get_content().unwrap()".to_string()).clone();
                        let flag= diff.is_diff();
                        if flag==true {
                            /*let d= diff.get_patch();
                            println!("add: {},{}",value.get_file_name(),d);*/
                        }
                        else { println!("No difference, same"); }
                        //println!("delete: {}",value.get_file_name());
                }

            }//contain_last_commit true
            else { println!("wd has, last commit has not->maybe untrack file{}",x1); }
        }
        //so
        if contain_add==false && contain_remove==false && contain_last_commit==false{
            untrack.push(name);
        } else if contain_add==true && contain_last_commit==false {
            Changes_to_be_committed.push("Add new file: ".to_owned()+&name);
            //println!("Add new file{}",name);
            //Changes to be committed:
        }
        else if contain_remove==true && contain_last_commit==false {
            Changes_to_be_committed.push("Remove file: ".to_owned()+&name);
            //Changes to be committed:
        }
        else if contain_remove==true && contain_last_commit==true { println!("Modified file{}",name);
            //compare inside content see modify
        }
        0});
    //还差wd没有，stage和commit有没有？
    println!("Changes to be committed:");
    if  Changes_to_be_committed.capacity()==0{ println!("nothing to change");}
    println!("{:?}",Changes_to_be_committed);
    //already add file but modified, so just need commit
    //commit delete stage, last commit
    println!("Changes not staged for commit:");
    if  Changes_not_staged_for_commit.capacity()==0{ println!("nothing to change");}
    println!("{:?}",Changes_not_staged_for_commit);
    //need add first, then commit
    //
    //last commit has, stage has in add, but not commit yet, so not in wd
    println!("Untracked files:");
    if  untrack.capacity()==0{ println!("nothing to change");}
    println!("{:?}",untrack);
    //readall
    /*let res:&str;
        let mut count_add =0;
        let mut count_delete=0;
        println!("Changes not staged for commit:");
        for (path, ItemInfo) in stage_inside_add {//stage add
            //println!("add: {}",ItemInfo.get_file_name());//compare modify or not?
            let wd_rev=file::retrieve_info(path);
            if  wd_rev.is_err(){//Cannot find the proper working directory path for file
                println!("add: {}",ItemInfo.get_file_name());
            }
            else {
                println!("modified: {}",ItemInfo.get_file_name());
            }
        }//stage add end

        for (path, ItemInfo) in stage_inside_remove {//stage remove
            let wd_rev=file::retrieve_info(path);
            if  wd_rev.is_err(){//Cannot find the proper working directory path for file
                println!("{} already be deleted",ItemInfo.get_file_name());
            }
            else {
                let a=load.get_file_content(ItemInfo);
                // TODO: use different get content method,use repo to get_content
                let diff = file_diff("wd_rev.unwrap().get_content().unwrap()".to_string(), "ItemInfo.get_content().unwrap()".to_string()).clone();
                let flag= diff.is_diff();
                if flag==true {
                    let d= diff.get_patch();
                    println!("add: {},{}",ItemInfo.get_file_name(),d);
                }
                else { println!("No difference, same"); }
                println!("delete: {}",ItemInfo.get_file_name());
            }
        }//stage remove end

        //commit but not push
        //let wd_rev=file::retrieve_info()?;//ItemInfo, read file path
        if stage_inside_add.capacity()!=0 && stage_inside_remove.capacity()!=0 {
            return Err(Errors::Errstatic("Please commit first!"))
        }
        else { //no stage, now compare last commit with wd//untrack, create a new file
            let last_commit= load.get_current_head();//Rev//TODO???
            if  last_commit.is_err(){ println!("no head, means last commit is empty");}
            else
            {
                let last_commit_file=last_commit.unwrap();
            let last_commit_hashmap=last_commit_file.get_manifest();//iteminfo
            for (path, ItemInfo) in last_commit_hashmap {//read last commit
                let wd_rev=file::retrieve_info(path);
                if  wd_rev.as_ref().is_err(){//Cannot find the proper working directory path for file
                    println!("{} is ahead of working directory",ItemInfo.get_file_name());
                }
                else {//compare diff
                    if ItemInfo.clone()==wd_rev.as_ref().unwrap().clone(){
                        return Err(Errors::Errstatic("No difference, same, means up to date"))
                    }
                    else {
                        let diff = file_diff("ItemInfo.get_content().unwrap()".to_string(), "wd_rev.unwrap().get_content().unwrap()".to_string());
                        let flag= diff.is_diff();
                        if flag==true {
                            println!("{}is ahead of working directory",ItemInfo.get_file_name());
                            return Err(Errors::Errstatic("Please push first!"))
                        }
                        else {
                            return Ok("No difference, same, means up to date")
                        }
                }
            }
        }
            }

    }//commit but not push end*/
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
        assert_eq!(res, false);
    }

    #[test]
    fn test_logs() {
        let wd = dsr::get_wd_path();
        let res = log(&wd,"");
        println!("{:?}", res.is_ok());
        //assert_eq!(res.unwrap(), "load.crate::vc::repository:get_log(),log information, information");
    }

    #[test]
    fn test_status() {
        let wd = "remoterepo/remote/.dvcs/HEAD";
        let res = status(wd);
        //assert_eq!(res.unwrap(),"ok");
    }
}
