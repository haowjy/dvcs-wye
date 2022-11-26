use crate::cmd_function::{file_diff, FileDiff};
use crate::vc::{file, repository, revision};

pub fn heads<'a>(wd:&'a str)->Result<&'a str,&'a str>{
    //get VC::Repo::load
    let load=crate::vc::repository::load(wd);//Repo
    //use get_heads to load.current_head;
    let head="VC::Repository::get_current_head()";
    /*
    return revision_id according to diff command line
    VC::Repo::load;
    let head=VC::Repository::get_current_head();//for heads
    (let head=VC::Repository::get_heads();
    //for heads -all with Result<Vec<&Revision>>)
    head
    */
    Ok(head)
}
pub fn log<'a>(wd:&'a str)->Result<&'a str,&'a str>{
    let load=crate::vc::repository::load(wd);//got Repo
    //load.get_log();
    //let version=vc::Repository::load();
    /*
    VC::Repository::load()
    let log=VC::Repository::get_log()
    VC::Revision::parent()
    log
    */
    //add log here
    let log="load.crate::vc::repository:get_log(),log information, information";
    Ok(log)
}
pub fn status<'a>(wd:&'a str)->Result<FileDiff,&'a str>{
    /*VC::from_stage();
    VC::Repo::load();
    VC::Repo::get_rev();//old_revision: &str
    VC::Rev::new();
    let diff=CF::file_diff(content1, content2);
    diff*/
    let load=crate::vc::repository::load(wd);//got Repo
   /* let w=load.get_rev();
    print!("{:?}",w);*/
    let diff=file_diff("content1", "content2");
    print!("{:?}",diff);
    //diff
    Ok(diff)
}

/*#[cfg(test)]
mod test{
    use super::*;
}*/
