#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
// use std::time::SystemTime;
// external crates:
// use petgraph::graphmap::DiGraphMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::dsr::*;
use crate::vc::file::*;
use crate::vc::revision::*;
use crate::ui::Errors;



#[derive(Debug, Serialize, Deserialize)] 
pub struct Repo {
    current_head: Option<String>, // alias 
    branch_heads: HashMap<String,String>, // <K=alias, V=sha_rev_id>
    paths: RepoPaths,
    stage: Stage,
    // revs: DiGraphMap<String, RevInfo>, // will change to wrapper struct
    // remote_head: Option<String>
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct Stage {
    to_add: HashMap<String, ItemInfo>,
    to_remove: HashMap<String, ItemInfo>,
}

impl Stage {
    pub fn get_add(&self) -> &HashMap<String, ItemInfo> {
        &self.to_add
    }

    pub fn get_remove(&self) -> &HashMap<String, ItemInfo> {
        &self.to_remove
    }
    
    pub fn is_empty(&self) -> bool {
        self.to_add.is_empty() && self.to_remove.is_empty()
    }

    fn clear(&mut self) -> &Self {
        self.to_add.clear();
        self.to_remove.clear();
        self
    }

    fn new() -> Self {
        Stage {
            to_add: HashMap::new(),
            to_remove: HashMap::new()
        }
    }
}

// ------ pub Repo fns ------

impl Repo {
    pub fn get_current_head(&self) -> Result<Rev, Errors> {

        match &self.current_head {
            Some(alias) => {
                match self.branch_heads.get(alias) {
                    Some(id) => self.get_rev(id),
                    None => Err(Errors::ErrStr("Unable to locate current head by alias".to_string()))
                }
            },
            None => Err(Errors::ErrStr("Null current head".to_string()))
        }
    }

    pub fn get_rev(&self, rev_id: &str) -> Result<Rev, Errors>  {
        Rev::from(&path_compose(&self.paths.revs, rev_id))
    }

    // *** In the process to be rewritten 
    pub fn commit(&mut self) -> Result<(), Errors> {
        Err(Errors::ErrUnknown)
    }
    
    // Result<CommitResult, Errors> { // Result<CommitResult, Errors> *** TODO
    //     self.stage.to_add.

    //     self.stage.store_files(&self.paths.files)?;// need to figure out logging to report
    //     self.stage.update_time();

    //     let commit_id = self.stage.gen_id()?;
        
    //     self.stage.save(&self.paths.revs)?; // save to revs 

    //     // update head pointers
    //     match &self.current_head {
    //         Some(alias) => {if let Some(id) = self.branch_heads.get_mut(self.current_head.as_ref()?) {
    //             *id = commit_id;
    //             } // update id
    //         },
    //         None => {
    //             let new_head_alias = "main";
    //             self.branch_heads.insert(new_head_alias.to_string(), commit_id);
    //             self.current_head = Some(new_head_alias.to_string());
    //         }
    //     }
    //     self.save();
    //     Some(())
    // }


    pub fn get_heads(&self) -> &HashMap<String, String> {
        &self.branch_heads
    }

    pub fn new_head(&mut self, head_alias:&str, rev_id:&str) -> &Self { // *** needs revisiting later
        self.branch_heads.entry(head_alias.to_string()).or_insert(rev_id.to_string());
        self
    }

    // pub fn fetch(&mut self, rwd:&str) -> &Self; // *** to be implemented

// ------ newly added pub functions ------
    pub fn get_file_content(&self, file_id: &str) -> Result<String, Errors> { // support cat 
        read_file_as_string(&path_compose(&self.paths.files, file_id))
    }

    pub fn get_stage(&self) -> &Stage {
        &self.stage
    }

    pub fn clear_stage(&mut self) -> &Self {
        self.stage.clear();
        self
    }

    pub fn clone(rwd:&str) -> Option<()> { // clone from a remote repo
        None // *** to be impl
        // needs to update remote head and have data structure tracking the rwd path,
    }

    // pub fn add_files(&mut self, abs_paths: Vec<&str>) -> Option<()> { // *** list version to be impl
    pub fn add_file(&mut self, abs_path: &str) -> Result<(), Errors> {
        // abs_paths.iter()
        let mut temp_rev = Rev::new();
        temp_rev.add_file(abs_path)?; // *** iterator operations and branching tbd here
        self.stage.to_add.extend(temp_rev.get_manifest().clone());
        self.save()
    }

    pub fn remove_file(&mut self, abs_path:&str) -> Result<(), Errors>{
        let mut temp_rev = Rev::new();
        temp_rev.add_file(abs_path)?; // *** iterator operations and branching tbd here
        self.stage.to_remove.extend(temp_rev.get_manifest().clone());
        self.save()
    }


    pub fn merge_commit(&mut self, parent_1: &str, parent_2: &str, msg: Option<&str>) -> Result<(), Errors> {
        Ok(()) // *** to be impl
    }
}


// ------ pub mod fns ------
pub fn init(opt_path: Option<&str>) -> Result<(), Errors> {
    let wd = match opt_path {
        Some(path) => path.to_string(),
        None => get_wd_path()
    };
    let paths = RepoPaths::new(&wd);
    if !is_path_valid(&paths.root) { // bypass recreating dirs if .dvcs already exists
        create_dir(&paths.files)?;
        create_dir(&paths.revs)?;
    }

    if !is_path_valid(&paths.repos) {
        let new_repo = Repo {
            current_head: None, // Some("main".to_string())
            branch_heads: HashMap::new(),
            paths: paths,
            stage: Stage::new(),
            // remote_head: None,
        };
        new_repo.save()?;
    }
    return Ok(());
}

pub fn load(wd:&str) -> Result<Repo, Errors> { // Result<Repo, ()>
    let checked_wd = match check_wd(wd) {
        Some(s) => s,
        None => return Err(Errors::ErrStr("Directory untracked, fail to locate repository".to_string()))
    };
    let paths = RepoPaths::new(&checked_wd);
    let mut load_repo: Repo = match serde_json::from_str(&read_file_as_string(&paths.repos)?) {
        Ok(x) => x,
        Err(e) => return Err(Errors::ErrSerde(e))
    };
    load_repo.paths = paths;
    Ok(load_repo)
}

pub fn get_wd_root() -> Result<String, Errors> {
    let wd = get_wd_path();
    match check_wd(&wd) {
        Some(path) => Ok(path),
        None => Err(Errors::ErrStr("Directory untracked, fail to locate repository".to_string()))
    }
}


// ------ private Repo fns ------
impl Repo {
    fn save(&self) -> Result<(), Errors> {
        let content = serialize(self)?;
        write_file(&self.paths.repos, &content)?;
        Ok(())
        // write_file(&self.paths.current_head)
        // write_file(&self.paths.branch_heads, &serde_json::to_string(&self.branch_heads).ok()?).ok()?;
        // write_file(&self.paths.branch_heads, &serde_json::to_string(&self.branch_heads).ok()?).ok()?;
    }
}

// ------ private mod fns ------
fn check_wd(wd_path: &str) -> Option<String> {
    if is_path_valid(&path_compose(wd_path, ".dvcs")) {
        return Some(wd_path.to_string());
    } else {
        match get_parent_name(wd_path) {
            Some(parent) => check_wd(&parent),
            None => None
        }
    }
}

// weak untested fn prone to error
pub (super) fn get_rel_path(abs_path: &str) -> Option<String> {
    let wd = get_name(&check_wd(&get_wd_path())?)?;
    let rel_path = abs_path.rsplit_once(&wd)?.1.trim_matches('/'); // not tested on windows
    Some(rel_path.to_string())
}

pub (super) fn get_abs_path(rel_path: &str) -> Option<String> {
    let wd_path = check_wd(&get_wd_path())?;
    Some(path_compose(&wd_path, rel_path))
}

pub (super) fn sha<T: AsRef<[u8]> + ?Sized> (data: &T) -> String {
    format!("{:x}", Sha256::digest(data))
}

// // preliminary fn might change later or make a trait
// pub (super) fn sha_match<'a, T: Clone + Iterator + Iterator<Item=&'a str>> (sha: &'a str, pool: T) -> Vec<&'a str> {
//     let sha_len = sha.len();
//     pool.filter(|v| {
//         if v.len() < sha_len {
//             false
//         } else {
//             v[0..sha_len] == *sha
//         }
//     }).clone().collect::<Vec<_>>()
// }
// resolving sha conflict
pub (crate) fn checked_sha(data: &str, matching_pool_path: &str) -> String {
    let mut sha_id = format!("{:x}", Sha256::digest(data));

    while is_path_valid(&path_compose(&matching_pool_path, &sha_id)) { // if same sha_id already exists 
        let compare_content = read_file_as_string(&path_compose(&matching_pool_path, &sha_id)).unwrap();
        if &compare_content == data {
            break;
        }
        sha_id.push('0');
    }
    sha_id
}

// ------ private mod structs ------

#[derive(Debug, Serialize, Deserialize)] 
pub (super) struct RepoPaths { 
    // wd: &str,// inconsistent types for paths, might need better type representation
    wd: String,
    root: String,
    pub files: String,
    pub revs: String,
    repos: String,
    // head: String, // THE current head
    // branch_heads: String,
    // stage: String,
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
            repos: path_compose(&root, "repos")

            // head: path_compose(&root, "head"),
            // branch_heads: path_compose(&root, "branches"),
            // stage: path_compose(&root, "stage"),
        }
    }

    // pub (super) fn default() -> RepoPaths { // relative path config
    //         let wd = ".";
    //         let root = path_compose(wd, ".dvcs");
    //     RepoPaths {
    //         wd: wd.to_string(),
    //         root: root.clone(),
    //         files: path_compose(&root, "files"),
    //         revs: path_compose(&root, "revs"),
    //         repos: path_compose(&root, "repos")
    //         // head: path_compose(&root, "head"),
    //         // branch_heads: path_compose(&root, "branches"),
    //         // stage: path_compose(&root, "stage"),
    //     }
    // }
}

pub(super) fn serialize<T: Serialize> (data_struct: &T) -> Result<String, Errors> {
    match serde_json::to_string(data_struct) {
        Ok(x) => Ok(x),
        Err(e) => Err(Errors::ErrStr(e.to_string()))
    }
}

// in-process attempt to generalize deserialize with trait / trait obj

// pub trait SerDe {
//     type Item;

//     fn deserialize(str_in: &str) -> Result<Self::Item, Errors> {
//         match serde_json::from_str(str_in) {
//             Ok(x) => Ok(x),
//             Err(_) =>  Err(Errors::ErrStr("Deserialization failed!".to_string()))
//         } 

//     }

// }

// pub(super) fn deserialize(str_in: &str) -> Result<Box<dyn Deserialize>, Errors> {
//     serde_json::from_str(str_in) match {
//         Ok(val) => Ok(val),
//         Err(e) => Err(Errors::ErrStr(e.to_string()))
//     }

// }

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
            init(None);
            assert!(fs::read_dir(paths.revs).is_ok());
            assert!(delete_dir(&paths.root).is_ok());
        }
    }