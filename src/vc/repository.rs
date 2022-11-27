#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
use std::time::SystemTime;

// external crates:
use petgraph::graphmap::DiGraphMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::dsr::*;
use crate::vc::file::*;
use crate::vc::revision::*;

#[derive(Debug)] 
pub struct Repo {
    current_head: Option<String>, // alias 
    branch_heads: HashMap<String,String>, // <K=alias, V=sha_rev_id>
    paths: RepoPaths,
    // revs: DiGraphMap<String, RevInfo>, // will change to wrapper struct
    remote_head: Option<String>
}
// #[derive(Debug, Hash, PartialEq)]
// struct RevInfo {
//     id: String,
//     parent_id: Option<String>,
//     parent
// }

impl Repo {
    pub fn get_current_head(&self) -> Option<Rev> {
        match &self.current_head {
            Some(alias) => Rev::from(self.branch_heads.get(alias)?),
            None => None
        }
    }

    pub fn get_rev(&self, rev_id: &str) -> Option<Rev> {
        Rev::from(&path_compose(&self.paths.revs, rev_id))
    }

    pub fn commit(&mut self) -> Option<()> {
        let mut stage = Rev::from(&self.paths.stage)?;
        stage.store_files(&self.paths.files)?;// need to figure out logging to report
        stage.update_time();
        let commit_id = stage.gen_id()?;
        stage.save()?;
        if let Some(id) = self.branch_heads.get_mut(self.current_head.as_ref()?) {
            *id = commit_id;
        }
        
        Some(())
    }

    // pub fn repo_root()

    pub fn get_log(&self) -> Option<Vec<&str>> {None} // *** to be implemented
    // called by command "log", trace from the current head to parent(s) recursively to get a complete history of metadata in relevant revisions

    // pub fn get_heads(&self) ->Option<Vec<&Rev>> {None}; // *** to be implemented
    // get heads for all branches

    pub fn new_head(&mut self, head_alias:&str, rev_id:&str) -> &Self {
        self.branch_heads.entry(head_alias.to_string()).or_insert(rev_id.to_string());
        self
    } 

    // pub fn fetch(&mut self, rwd:&str) -> &Self; // *** to be implemented

// ------ newly added pub functions ------
    pub fn get_file_content(&self, file_id: &str) -> Option<String> {
        read_file_as_string(&path_compose(&self.paths.files, file_id)).ok()
    }

    pub fn get_stage(&self) -> Option<Rev> {
        Rev::from(&self.paths.stage)
    }


// ------ private fns ------
    fn save_stage(&self) -> Option<()> {
        None // *** to be impl
    }

    fn save(&self) -> Option<()> { // Result<(), ()> {
        // *** to add: save head
        write_file(&self.paths.branch_heads, &serde_json::to_string(&self.branch_heads).ok()?).ok()
    }

}

pub fn init() -> Option<()> { // Result<(),()>{ // error handling to be impl
    let wd = get_wd_path();
    let paths = RepoPaths::new(&wd);
    // ***** error handling needed *****
    // esp: handle running init again with existing repo
    // try loading existing repo first 
    // match let new_repo = load():
    
    create_dir(&paths.files).ok()?; // *** CHANGE ERR HANDLING LATER // root .dvcs automatically added
    create_dir(&paths.revs).ok()?;
    // create_file(&paths.branch_heads); 
    // create_file(&paths.head);
    let new_repo = Repo {
        current_head: None, // Some("main".to_string())
        branch_heads: HashMap::new(),
        paths: paths,
        remote_head: None,
        // revs: None, // *** CHANGE LATER
    };
    new_repo.save();
    return Some(());
}

pub fn load(wd:&str) -> Option<Repo> { // Result<Repo, ()>
    let paths = RepoPaths::new(wd);
    let load_repo = Repo {
        current_head: read_file_as_string(&paths.head).ok(),
        branch_heads: serde_json::from_str(&paths.branch_heads).ok()?,
        paths: paths,
        remote_head: None,// *** CHANGE LATER
        // revs: None, 
    };
    return Some(load_repo);
}

// new pub fn
pub fn clone(rwd:&str) -> Option<()> { // clone from a remote repo
    None
    // *** to be implemented
    // needs to update remote head and have data structure tracking the rwd path,
}

pub fn add_file(abs_path: &str) -> Option<()> {
    None // *** to be impl
}

pub fn remove_file(abs_path:&str) -> Option<()> {
    None // *** to be impl
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