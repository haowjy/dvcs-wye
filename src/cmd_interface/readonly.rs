use crate::cmd_function::{file_diff, FileDiff};
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
                let parent_head = current_head.get_parent_id().unwrap();
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

pub fn status(wd: &str) -> Result<&str, Errors> {
    //modified:   a.txt
    //add
    //ahead of ?commit
    //up to date
    let load=repository::load(wd)?;//got Repo以后可以换成？
    //let renew=load.get_current_head()?;//get Rev
    let stage=load.get_stage();// got stage暂存区
    let stage_inside_add=stage.get_add();
    let stage_inside_remove=stage.get_remove();
    //readall
let res:&str;
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
            let diff = file_diff(wd_rev.unwrap().get_content().unwrap(), ItemInfo.get_content().unwrap()).clone();
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
    else { //no stage, now compare last commit with wd
        let last_commit= load.get_current_head();//Rev//TODO???
        if  last_commit.is_err(){ println!("no head, means last commit is empty");}
        else
        {
            let last_commit_file=last_commit.unwrap();
        let last_commit_hashmap=last_commit_file.get_manifest();//iteminfo
        for (path, ItemInfo) in last_commit_hashmap {//read last commit
            let wd_rev=file::retrieve_info(path);
            if  wd_rev.as_ref().is_err(){//Cannot find the proper working directory path for file
                println!("{} is ahead of work directory",ItemInfo.get_file_name());
            }
            else {//compare diff
                if ItemInfo.clone()==wd_rev.as_ref().unwrap().clone(){
                    return Err(Errors::Errstatic("No difference, same, means up to date"))
                }
                else {
                    let diff = file_diff(ItemInfo.get_content().unwrap(), wd_rev.unwrap().get_content().unwrap());
                    let flag= diff.is_diff();
                    if flag==true {
                        println!("{}is ahead of work directory",ItemInfo.get_file_name());
                        return Err(Errors::Errstatic("Please push first!"))
                    }
                    else {
                        return Ok("No difference, same, means up to date")
                    }
            }
        }
    }
        }

}//commit but not push end
    Ok("???")
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
        assert_eq!(res.unwrap(),"ok");
    }
}
