use crate::cmd_function::file_diff;

pub fn heads<'a>(wd:&'a str)->Result<&'a str,&'a str>{
    /*
    //return revision_id according to diff command line
    VC::Repo::load
    let head=VC::Repository::get_current_head();//for heads
    (let head=VC::Repository::get_heads();
    //for heads -all with Result<Vec<&Revision>>)
    head
    */
    unimplemented!();
}
pub fn log<'a>(wd:&'a str)->Result<&'a str,&'a str>{
    /*
    VC::Repository::load()
    let log=VC::Repository::get_log()
    VC::Revision::parent()
    log

    */
    unimplemented!();
}
pub fn status<'a>(wd:&'a str)->Result<&'a str,&'a str>{
    /*VC::from_stage();
    VC::Repo::load();
    VC::Repo::get_rev();//old_revision: &str
    VC::Rev::new();
    let diff=CF::file_diff(content1, content2);
    diff*/
    let diff=file_diff("content1", "content2");
    //diff
    unimplemented!();
}

/*#[cfg(test)]
mod test{
    use super::*;
}*/
