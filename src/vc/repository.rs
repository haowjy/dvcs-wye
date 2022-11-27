#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
use std::time::SystemTime;

// external crates:
use petgraph::graphmap::DiGraphMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::dsr::*;


#[derive(Debug)] 
pub struct Repo {
    current_head: Option<String>, // sha_rev_id 
    branch_heads: Option<HashMap<String,String>>, // <K=alias, V=sha_rev_id>
    paths: RepoPaths,
    revs: Option<DiGraphMap<String, String>>, // will change to wrapper struct
}

#[derive(Debug)] 
pub (super) struct RepoPaths { 
    // wd: &str,// inconsistent types for paths, might need better type representation
    wd: String,
    root: String,
    pub files: String,
    pub revs: String,
    head: String, // THE current head
    branch_heads: String,
    stage: String,
}

impl RepoPaths {
    pub (super) fn new(wd: &str) -> RepoPaths { // absolute path config
        let root = path_compose(wd, ".dvcs"); 
        //experiment with relative path
        // let root = path_compose(".", ".dvcs");
        RepoPaths {
            wd: wd.to_string(),
            root: root.clone(),
            files: path_compose(&root, "files"),
            revs: path_compose(&root, "revs"),
            head: path_compose(&root, "head"),
            branch_heads: path_compose(&root, "branches"),
            stage: path_compose(&root, "stage"),
        }
    }

    pub (super) fn default() -> RepoPaths { // relative path config
            let wd = ".";
            let root = path_compose(wd, ".dvcs");
        RepoPaths {
            wd: wd.to_string(),
            root: root.clone(),
            files: path_compose(&root, "files"),
            revs: path_compose(&root, "revs"),
            head: path_compose(&root, "head"),
            branch_heads: path_compose(&root, "branches"),
            stage: path_compose(&root, "stage"),
        }
    }
}

impl Repo {

    // fn commit(&self) -> Self {
    //     let staged = Rev::from(self.paths.stage);
        
    //     // self.revs
    // }

    pub (crate) fn save(&self) -> () { // Result<(), ()> { 
        () // *** CHANGE LATER

        // write_result = write_file(&self.paths.head, &self.current_head.unwrap_or("").to_string());
        // write_file(self.paths.branch_heads, serde::serialize(self.branch_heads)); // *** CHANGE LATER
        // additional writing operations possible
    }

}

pub fn init() -> Repo { // Result<(),()>{ // error handling to be impl
    let wd = get_wd_path();
    let paths = RepoPaths::new(&wd);
    // ***** error handling needed *****
    // esp: handle running init again with existing repo
    // try loading existing repo first 
    // match let new_repo = load():
    
    create_dir(&paths.files).ok(); // *** CHANGE ERR HANDLING LATER // root .dvcs automatically added
    create_dir(&paths.revs).ok();
    // create_file(&paths.branch_heads); 
    // create_file(&paths.head);
    let new_repo = Repo {
        current_head: None,
        branch_heads: None,
        paths: paths,
        revs: None, // *** CHANGE LATER
    };
    // new_repo.save();
    return new_repo;
}

pub fn load(wd:&str) -> Repo { // Result<Repo, ()>
    let paths = RepoPaths::new(wd);
    let load_repo = Repo {
        current_head: read_file_as_string(&paths.head).ok(),
        branch_heads: None, // *** CHANGE, FOR TESTING ONLY

        // branch_heads: read_file_as_string(&paths.branch_heads).ok(),
        paths: paths,
        revs: None, // *** CHANGE LATER
    };
    return load_repo;
}

pub (super) fn sha<T: AsRef<[u8]> + ?Sized> (data: &T) -> String {
    format!("{:x}", Sha256::digest(data))
}

// preliminary fn might change later or make a trait
pub (super) fn sha_match<'a, T: Clone + Iterator + Iterator<Item=&'a String>> (sha: &'a String, pool: T) -> Vec<&'a String> {
    let sha_len = sha.len();
    pool.filter(|v| {
        if v.len() < sha_len {
            false
        } else {
            v[0..sha_len] == *sha
        }
    }).clone().collect::<Vec<_>>()
}
// resolving sha conflict
// pub (crate) fn sha_resolve(sha: &str, pool: ) -> 

#[cfg(test)]
mod tests {
        use super::*;
        use std::fs;
        // #[test]
        // fn test_make_repo_paths() {
        //     use std::path::PathBuf;
        //     let wd = get_wd_path();

        //     let paths = RepoPaths::new(&wd);
        //     print!("{}",paths.root);
        //     let mut path = PathBuf::from(&wd);
        //     path.push(".dvcs");
        //     assert_eq!(paths.root, path.to_str().unwrap());
        // }

        #[test]
        fn test_init() {
            let wd = get_wd_path();
            let paths = RepoPaths::new(&wd);
            print!("{}", &paths.revs);
            init();
            assert!(fs::read_dir(paths.revs).is_ok());
            assert!(delete_dir(&paths.root).is_ok());
        }
    }