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
    //let rev=revision::Rev::from(wd);//got Rev
    let load=repository::load(wd)?;//got Repo以后可以换成？
    let renew=load.get_current_head()?;//get Rev
    let stage=load.get_stage();// got stage暂存区里的一个版本
    let stage_inside_add=stage.get_add();
    let stage_inside_remove=stage.get_remove();
    //readall

    for (path,iteminfo) in stage_inside_add {
        let wd_rev=file::retrieve_info(path)?;
        let wd_rev=file::retrieve_info(path)?;
    }
    //let wd_rev=file::retrieve_info()?;//ItemInfo, read file path
    let last_commit= load.get_current_head()?;//Rev
    let last_commit_file=last_commit.get_manifest().get(wd).unwrap().clone();//iteminfo//TODO???
    /*if last_commit_file==wd_rev{
        println!("eq")
    }*/
    //iteminfo compare eq, iteminfo use get_content() to get string and use diff compare
    //let id =rev.as_ref().unwrap().get_id().unwrap();
    //let revision=load.as_ref().unwrap().get_rev(id);
    /*
    let stage=revision::Rev::from_stage();// got stage暂存区里的一个版本
    if stage==null返回nothing to commit, working tree clean
    // WD 从  file里的   read, retieve_info, return iteminfo
    // last commit repo.get_current_head()
    //
    read stage: let stage=revision::Rev::from_stage();// got stage's revision
    read WD: let wd_rev=retieve_info; iteminfo --hashmap read balabala
    read last commit: let last_commit= repo.get_current_head();

    WD Stage Last
    1  0      0      add
    1  1      0      modify?
    1  1      1      modify->check
    1  0      1      delete?
    0  0      1      delete?
    0  0      0      no change
    0  1      0      add?
    0  1      1      modify?
    else compare with commit最后的一个版本？//aka current head?
    如果是很多file返回的可能是vec<string>?So read iter() compare?

    let stage=revision::Rev::from_stage();
    stage.get_files();//获得文件，return hashmap
    each file get_content();

    //同理
    get_current head()?
    ???又是get_revision? getfile???
    有没有getfilename???和通过filename找到file content的hashmap的key是path
    path same, compare id; id not same, output;
    WD's path not exist, deleted.
    Stage

    VC::Repo::load();
    VC::Repo::get_rev();//old_revision: &str
    VC::Rev::new();
    let diff=CF::file_diff(content1, content2);
    diff*/
    //let load=repository::load(wd);//got Repo
    /* let w=load.get_rev();
     print!("{:?}",w);*/
    //let file=load.unwrap().get_file_content(id).unwrap();
    //let file1="content1".to_string();
    let diff = file_diff("&file1".to_string(), "&file".to_string()).clone();
    //
    let flag= diff.is_diff();
    if flag==true {
        let d= diff.get_patch();
        println!("{}",d);
        return Ok("diff")
    }
    else { println!("No difference, same");
        Err(Errors::Errstatic("diff"))
    }
    //diff
    //Ok(diff)
}

#[cfg(test)]
mod test {
    use crate::dsr;
    use super::*;

    #[test]
    fn test_heads() {
        let wd = dsr::get_wd_path();
        let res = heads(&wd).unwrap();
        println!("{:?}", res);
        //assert_eq!(res.get_parent_id().unwrap(), "VC::Repository::get_current_head()");
    }

    #[test]
    fn test_logs() {
        let wd = dsr::get_wd_path();
        let res = log(&wd,"");
        println!("{:?}", res.unwrap().unwrap());
        //assert_eq!(res.unwrap(), "load.crate::vc::repository:get_log(),log information, information");
    }

    #[test]
    fn test_status() {
        let wd = "remoterepo/remote/.dvcs/HEAD";
        let res = status(wd);
        assert_eq!(res.unwrap(),"ok");
    }
}
