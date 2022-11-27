use crate::vc::{file, repository, revision};

pub struct RevDiff {
    // TODO
}

pub fn diff<'a>(rev1_id:&'a str, rev2_id:&'a str) -> Result<RevDiff, &'a str>{
    // go through all files in rev1 and rev2
    // if file in rev1 but not in rev2 -> file deleted
    // if file in rev2 but not in rev1 -> file added
    // if file in rev1 and rev2, but there is a diff -> file modified
    // if file in rev1 and rev2, and there is no diff -> file unchanged
    unimplemented!(); //TODO
}

pub fn cat<'a>(rev_id:&'a str, path:&'a str) -> Result<&'a str, &'a str>{
    // find path in rev
    // return file content or error
    unimplemented!(); //TODO
}

pub fn add<'a>(path:&'a str) -> Result<&'a str, &'a str>{
    // add the file temporarily to the index branch (not committed yet)
    // just call repo.add by obtaining absolute path
    // if pwd is /Users/jimmyyao/gitrepos/dvcs-wyesrc/cmd_function
    // git add mod.rs

    // get split based on /Users/jimmyyao/gitrepos/dvcs-wye
    // /Users/jimmyyao/gitrepos/dvcs-wye/src/cmd_function/mod.rs -> 
    // pass rel path (src/cmd_function/mod.rs)
    unimplemented!(); //TODO
}

pub fn remove<'a>(path:&'a str) -> Result<&'a str, &'a str>{
    // remove the file temporarily to the index branch by acting as if we have deleted the file (not committed yet)
    // just call repo.remove by obtaining absolute path
    unimplemented!(); //TODO
}

pub fn commit<'a>(message:&'a str) -> Result<&'a str, &'a str>{
    // commit the index branch to the head branch, create a new revision and update the head
    // write to log -> where would this be?

    // return a RevDiff if successful
    unimplemented!(); //TODO
}

pub fn merge<'a>(rev_id:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {
    use super::*;
}