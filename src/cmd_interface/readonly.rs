use crate::cmd_function::{file_diff, FileDiff};
use crate::vc::{file, repository, revision};
use crate::vc::revision::Rev;
use crate::ui::Errors;

pub fn heads(wd: &str) -> Result<Rev, Errors> {
    let load = repository::load(wd);//get Repo
    if load.is_none() { return Err(Errors::Errstatic("No heads found")); } else {
        //let head = load.unwrap().get_heads();//Hashmap//waiting
        //if head.is_none() { return Err(Errors::Errstatic("No heads found")); } else { Ok(head.unwrap()) }
    }
    unimplemented!(); //TODO
}

pub fn log(wd: &str,rev_id: &str) -> Result<Option<Vec<String>>, Errors> {//alias,rev_id: &str
    if rev_id.is_empty() {
        //normal
        println!("empty")
    } else {
        //
        println!("not empty")
    }
    let mut string=Vec::new();
    let load = repository::load(wd);//got Repo暂时先读

    if load.is_none() { return Err(Errors::Errstatic("No log found")); } else {
        let log = load.as_ref().unwrap().get_log();//change into Hashmap
        if log.is_none() { return Err(Errors::Errstatic("No log found")); } else {
            //let vec = log.as_ref().unwrap();
            let current_head = load.as_ref().unwrap().get_current_head().unwrap();//
            current_head.get_manifest();//here get hashmap, this is log need print, maybe put into Vec<String>
            string.push(("example").parse().unwrap());
            let parent_head = current_head.get_parent_id().unwrap();
            let mut new_rev = load.as_ref().unwrap().get_rev(parent_head).unwrap();
            while new_rev.get_parent_id().is_none() {
                new_rev = load.as_ref().unwrap().get_rev(new_rev.get_parent_id().unwrap()).unwrap();
                new_rev.get_manifest();//here get hashmap, this is log need print, maybe put into Vec<String>
                string.push(("example").parse().unwrap());
            }
            Ok(Some(string))
        }
    }
    //first current head->know parent head-> get_rev(return Revision碰到revision就读Parent id->until None
}

pub fn status(wd: &str) -> Result<&str, Errors> {
    //let rev=revision::Rev::from(wd);//got Rev
    let load=repository::load(wd).unwrap();//got Repo以后可以换成？
    let renew=load.get_current_head();
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
    1  1      0
    1  1      1
    1  0      1
    0  0      1
    0  0      0
    0  1      0
    0  1      1
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
    let diff = file_diff("&file1", "&file").clone();
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
    use super::*;

    #[test]
    fn test_heads() {
        let wd = "remoterepo/remote/.dvcs/HEAD";
        let res = heads(wd).unwrap();
        assert_eq!(res.get_parent_id().unwrap(), "VC::Repository::get_current_head()");
    }

    #[test]
    fn test_logs() {
        let wd = "remoterepo/remote/.dvcs/HEAD";
        let res = log(wd,"123");
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
