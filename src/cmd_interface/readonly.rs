use std::fs::read;
use crate::cmd_function::{file_diff};
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
    let cur_head = load.get_current_head_alias().unwrap();
    for(key,value) in head{
        string = if key == cur_head {
            "*".to_owned() + &key.to_owned()+":"+value
        } else {
            key.to_owned()+":"+value
        };

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
    string.push("user: ".to_owned()+hashmap.get("user").unwrap());
    string.push("id: ".to_owned()+hashmap.get("id").unwrap());
    string.push("message: ".to_owned()+hashmap.get("message").unwrap());
    string.push("time: ".to_owned()+hashmap.get("time").unwrap());
        /*for(key,value) in hashmap{
            string.push(key.to_owned()+ ":"+ &value);
        }*/
    string.push("".parse().unwrap());
        let mut parent_head = "";
        let parent_head_pre = current_head.get_parent_id();
        if  parent_head_pre.is_none(){ return Ok(Some(string));}
        else {parent_head=parent_head_pre.unwrap(); }

        new_rev = load.get_rev(parent_head).unwrap();
    hashmap=new_rev.get_log();//here get hashmap, this is log need print, maybe put into Vec<String>
    string.push("user: ".to_owned()+hashmap.get("user").unwrap());
    string.push("id: ".to_owned()+hashmap.get("id").unwrap());
    string.push("message: ".to_owned()+hashmap.get("message").unwrap());
    string.push("time: ".to_owned()+hashmap.get("time").unwrap());
    /*for(key,value) in hashmap{
        string.push(key.to_owned()+ ":"+ &value);
    }*/
    string.push("".parse().unwrap());
    while !new_rev.get_parent_id().is_none() {
        new_rev = load.get_rev(new_rev.get_parent_id().unwrap()).unwrap();
        hashmap=new_rev.get_log();//here get hashmap, this is log need print, maybe put into Vec<String>
        string.push("user: ".to_owned()+hashmap.get("user").unwrap());
        string.push("id: ".to_owned()+hashmap.get("id").unwrap());
        string.push("message: ".to_owned()+hashmap.get("message").unwrap());
        string.push("time: ".to_owned()+hashmap.get("time").unwrap());
        string.push("".parse().unwrap());
    }
    Ok(Some(string))
}

pub fn status(wd: &str) -> Result<(Vec<String>, Vec<String>, Vec<String>), Errors> {
    let load=repository::load(wd)?;//got Repo
    let stage=load.get_stage();// got stage
    let stage_inside_add=stage.get_add();
    let stage_inside_remove=stage.get_remove();

    let mut list_files:Vec<String> = vec![];
    let mut ignore:Vec<&str> = vec![];
    ignore.push(".dvcs");
    ignore.push("target");
    ignore.push("src");
    ignore.push(".idea");
    ignore.push(".DS_Store");
    ignore.push(".git");
    get_files(wd,ignore,&mut list_files);//file from fd

    let mut untrack:Vec<String>=vec![];
    let mut changes_to_be_committed:Vec<String>=vec![];
    let mut changes_not_staged_for_commit:Vec<String>=vec![];
    let a =list_files.iter().fold(0,|acc,x1| {
        let wd_item=dsr::read_file_as_string(x1).unwrap();
        let mut stage_status =false;
        let name=dsr::get_name(x1).unwrap();
        let contain_add= stage_inside_add.contains_key(&name);

        let contain_remove= stage_inside_remove.contains_key(&name);

         if contain_add==true {
             let contentoofstage=load.get_file_content(stage_inside_add.get(&name).unwrap()).unwrap();
             if wd_item!=contentoofstage{
                 changes_not_staged_for_commit.push("Modified file: ".to_owned()+&name);
                 stage_status=true;
             }

         }
        if contain_remove==true {
            let contentoofstage=load.get_file_content(stage_inside_remove.get(&name).unwrap()).unwrap();
            if wd_item!=contentoofstage{
                changes_not_staged_for_commit.push("Modified file: ".to_owned()+&name);
                stage_status=true;
            }
        }

        let last_commit= load.get_current_head();//Rev
        let mut contain_last_commit =false;
        if  last_commit.is_err(){ //println!("no head, means last commit is empty");
        }
        else
        {
            let last_commit_file=last_commit.unwrap();
            let last_commit_hashmap=last_commit_file.get_manifest();//iteminfo
            contain_last_commit= last_commit_hashmap.contains_key(&name);
            if contain_last_commit {

                if contain_add==true && contain_last_commit==true&&stage_status==false {
                    //println!("Modified file: {}",name);
                    //changes_not_staged_for_commit.push("Modified file: ".to_owned()+&name);
                    //compare inside content see modify
                    let it1= last_commit_hashmap.get(&name).unwrap();
                    let it2= stage_inside_add.get(&name).unwrap();
                        if it1.eq(it2) {
                        }
                        else {
                            //diff
                            let a=load.get_file_content(it2);//stage
                            let b=load.get_file_content(it1);//last_commit
                            let diff = file_diff(a.unwrap(), b.unwrap()).clone();
                            let flag= diff.is_diff();
                            if flag==true {
                                changes_to_be_committed.push("Modified file: ".to_owned()+&name);
                            }
                            else {
                            }
                        }
                }
                if  contain_add==false && contain_last_commit==true&&stage_status==false{
                    let it1= last_commit_hashmap.get(&name).unwrap();
                    let a=load.get_file_content(it1).unwrap();
                    if wd_item!=a{
                        changes_not_staged_for_commit.push("Modified file: ".to_owned()+&name);
                    }

                }
            }
            else {
            }
        }

        if contain_add==false && contain_remove==false && contain_last_commit==false{
            untrack.push(name);
        } else if contain_add==true && contain_last_commit==false {
            changes_to_be_committed.push("Modified/Add new file: ".to_owned()+&name);

        }
        else if contain_remove==true && contain_last_commit==false {
            changes_to_be_committed.push("Remove file: ".to_owned()+&name);

        }
        else if contain_remove==true && contain_last_commit==true {
            let last_commit_again= load.get_current_head();//Rev
            if  last_commit_again.is_err(){
            }
            else {
                let last_commit_file_again = last_commit_again.unwrap();
                let last_commit_hashmap = last_commit_file_again.get_manifest();//iteminfo
                let it1= last_commit_hashmap.get(&name).unwrap();
                let it2= stage_inside_remove.get(&name).unwrap();
                let a=load.get_file_content(it1).unwrap();
                let b=load.get_file_content(it2).unwrap();
                if a !=b{
                    changes_to_be_committed.push("Remove/Modified file: ".to_owned()+&name);
                }
                else {
                    changes_to_be_committed.push("File need to commit although same in stage and last commit: ".to_owned()+&name);
                }
                }
        }
        0});
    for(key, value)in stage_inside_remove{
        let contain_delete=list_files.contains(key);
        if  contain_delete==false{
            let last_commit_again= load.get_current_head();//Rev
            let mut contain_last_commit_again =false;
            if  last_commit_again.is_err(){
            }
            else
            {
                let last_commit_file_again=last_commit_again.unwrap();
                let last_commit_hashmap=last_commit_file_again.get_manifest();//iteminfo
                contain_last_commit_again= last_commit_hashmap.contains_key(key);
                if contain_last_commit_again {//wd no, stage yes, last commit yes
                    changes_not_staged_for_commit.push("Delete file: ".to_owned()+key);

                }
                else { //wd no,stage yes, last commit yes
                    changes_to_be_committed.push("Delete file: ".to_owned()+key);
                     }
            }


        }


    }
    let mut name_list:Vec<String>=vec![];
    list_files.iter().fold(0,|a,x1|{ name_list.push(dsr::get_name(x1).unwrap());
        0});
    let last_commit_again= load.get_current_head();//Rev
    if  last_commit_again.is_err(){ //println!("no head, means last commit is empty");
    }
    else
    {
        let last_commit_file_again=last_commit_again.unwrap();
        let last_commit_hashmap=last_commit_file_again.get_manifest();//iteminfo
        for(key,value)in last_commit_hashmap{
            if !name_list.contains(key) &&!stage_inside_remove.contains_key(key){
                changes_not_staged_for_commit.push("Delete file: ".to_owned()+key);
            }
        }
    }


    Ok((changes_to_be_committed, changes_not_staged_for_commit, untrack))
}

#[cfg(test)]
mod test {
    use crate::dsr;
    use crate::dsr::{create_dir, create_file};
    use crate::readwrite::{add, commit};
    use crate::vc::repository::init;
    use super::*;

    #[test]
    fn test_heads() {
        let wd = dsr::get_wd_path();
        let _c=init(None);
        let _a=add(&wd,"src");
        let res = heads(&wd).is_err();
        assert_eq!(res, true);
    }
    #[test]
    fn test_heads_2() {
        let wd = dsr::get_wd_path();
        let _a=init(None);
        let _b=add(&wd,"src");
        let _c=commit(&wd,"test");
        let res = heads(&wd).is_err();
        assert_eq!(res, true);
    }

    #[test]
    fn test_logs_3() {
        let wd = dsr::get_wd_path();
        let res = log(&wd);
        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_logs_4() {
        let wd = dsr::get_wd_path();
        let _c=create_dir("test_logs_4");
        let _a=create_file("a.txt");
        let _b=add(&wd,"a.txt");
        let _d=commit(&wd,"test_log");
        let res = log(&wd);
        assert_eq!(res.is_ok(), true);
    }
    #[test]
    fn test_status_5() {
        let wd = dsr::get_wd_path();
        let res = status(&wd);
        assert_eq!(res.is_ok(), true);
    }
    #[test]
    fn test_status_6() {
        let wd = dsr::get_wd_path();
        let _c=create_dir("test_status_6");
        let _a=create_file("a.txt");
        let _b=add(&wd,"a.txt");
        let res = status(&wd);
        assert_eq!(res.is_ok(), true);
    }
}
