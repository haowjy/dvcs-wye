use crate::cmd_function::{file_diff, FileDiff};
use crate::vc::{file, repository, revision};
use crate::vc::revision::Rev;
use crate::ui::Errors;

pub fn heads(wd: &str) -> Result<Rev, Errors> {
    let load = repository::load(wd);//get Repo
    if load.is_none() { return Err(Errors::Errstatic("No heads found")); } else {
        let head = load.unwrap().get_current_head();
        if head.is_none() { return Err(Errors::Errstatic("No heads found")); } else { Ok(head.unwrap()) }
    }
}

pub fn log(wd: &str) -> Result<Option<Vec<String>>, Errors> {
    let load = repository::load(wd);//got Repo
    if load.is_none() { return Err(Errors::Errstatic("No log found")); } else {
        let log = load.unwrap().get_log();
        if log.is_none() { return Err(Errors::Errstatic("No log found")); } else {
            let vec = log.as_ref().unwrap();
            Ok(log)
        }
    }
}

pub fn status(wd: &str) -> Result<&str, Errors> {
    let rev=revision::Rev::from(wd);//got Rev
    let load=repository::load(wd);//got Repo
    let id =rev.as_ref().unwrap().get_id().unwrap();
    let revision=load.as_ref().unwrap().get_rev(id);
    /*
    let stage=revision::Rev::from_stage();// got stage暂存区里的一个版本
    if stage==null返回nothing to commit, working tree clean
    else compare with commit最后的一个版本？//aka current head?
    如果是很多file返回的可能是vec<string>?So read iter() compare?

    let stage=revision::Rev::from_stage();
    stage.get_files();//获得文件，return hashmap
    each file get_content();

    //同理
    get_current head()?
    ???又是get_revision? getfile???
    有没有getfilename???和通过filename找到file content的hashmap的key是名字还是id?


    VC::Repo::load();
    VC::Repo::get_rev();//old_revision: &str
    VC::Rev::new();
    let diff=CF::file_diff(content1, content2);
    diff*/
    //let load=repository::load(wd);//got Repo
    /* let w=load.get_rev();
     print!("{:?}",w);*/
    let file=load.unwrap().get_file_content(id).unwrap();
    //let file1="content1".to_string();
    let diff = file_diff("&file1", &file).clone();
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
        let res = heads(wd);
        assert_eq!(res.unwrap(), "VC::Repository::get_current_head()");
    }

    #[test]
    fn test_logs() {
        let wd = "remoterepo/remote/.dvcs/HEAD";
        let res = log(wd);
        assert_eq!(res.unwrap(), "load.crate::vc::repository:get_log(),log information, information");
    }

    #[test]
    fn test_status() {
        let wd = "remoterepo/remote/.dvcs/HEAD";
        let res = status(wd);
        println!("{}", res.as_ref().unwrap().is_diff());
        println!("{}", res.as_ref().unwrap().get_patch());
    }
}
