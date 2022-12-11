#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
use chrono::format::Item;
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
                    None => Err(Errors::Errstatic("Unable to locate current head by alias"))
                }
            },
            None => Err(Errors::Errstatic("Null current head"))
        }
    }

    pub fn get_rev(&self, rev_id: &str) -> Result<Rev, Errors>  {
        match self.branch_heads.get(rev_id) {
            Some(id) => Rev::from(&path_compose(&self.paths.revs, id)),
            None => Rev::from(&path_compose(&self.paths.revs, rev_id))
        }
    }

    pub fn commit(&mut self, message: &str) -> Result<(), Errors> {
        let mut head = match self.get_current_head() {
            Ok(rev) => rev,
            Err(e) => {match e {
                Errors::Errstatic("Null current head") => { // handling first commit
                    self.current_head = Some("main".to_string());
                    Rev::new()
                },
                err => return Err(err)
                }
            }
        };
        self.commit_from(&mut head, message)
    }
    
    pub fn merge_commit(&mut self, parent_1: &str, parent_2: &str, msg: Option<&str>) -> Result<(), Errors> {
        let mut head = self.get_current_head()?;

        if head.get_id().unwrap_or_default() != parent_1 {
            return Err(Errors::Errstatic("merge destination is not current head"));
        }

        head.set_parent_2(parent_2);
        self.commit_from(&mut head, msg.unwrap_or(format!("Merged from {}", parent_2).as_str()))
    }

    pub fn commit_from(&mut self, head: &mut Rev, message: &str) -> Result<(), Errors> {
        let manifest_copy = head.manifest.clone();
        head.manifest.extend(self.stage.to_add.clone());
        self.stage.to_remove.iter().for_each(|(path, _)| {head.manifest.remove(path);});
        
        if manifest_copy == head.manifest {
            return Err(Errors::Errstatic("No changes added to commit."))
        }

        // save files to repos
        head.manifest.iter().try_for_each(
            |(_, entry)| 
                self.save_file_to_repo(entry).map(|_s| ()) // map different ok strings to ()
        )?;

        // update head and save rev to repos
        let user = get_user();
        head.set_user(&user.1);
        head.set_message(message);
        head.update_time();
        let id = head.gen_id(&self.paths.revs)?;
        head.save(&self.paths.revs)?;

        // update repos
        self.branch_heads.insert(self.current_head.clone().unwrap_or("main".to_string()), id);
        self.clear_stage()
    }


    pub fn get_heads(&self) -> &HashMap<String, String> {
        &self.branch_heads
    }

    pub fn new_head(&mut self, head_alias:&str, rev_id:&str) -> Result<(), Errors> { // *** needs revisiting later for error handling
        self.branch_heads.insert(head_alias.to_string(), rev_id.to_string());
        self.save()
    }

    
    pub fn fetch(&self, rwd:&str) -> Result<(), Errors> { // fetch from remote
        let rwd_paths = RepoPaths::new(rwd);
        let mut files:Vec<String> = Vec::new();
        let mut revs: Vec<String> = Vec::new();
        get_files(&rwd_paths.files, Vec::<&str>::new(), &mut files)?;
        get_files(&rwd_paths.revs, Vec::<&str>::new(), &mut revs,)?;

        files.iter().try_for_each(|file_path| {
            let cwd_file_path = path_compose(&self.paths.files, &get_name(file_path).ok_or(Errors::Errstatic("Unknown error when fetching files from remote directory"))?);
            if !is_path_valid(&cwd_file_path) {
                copy_file(file_path, &cwd_file_path)?;
            };
            Ok(())
        })?;

        revs.iter().try_for_each(|rev_path| {
            let cwd_rev_path = path_compose(&self.paths.revs, &get_name(rev_path).ok_or(Errors::Errstatic("Unknown error when fetching revisions from remote directory"))?);
            if !is_path_valid(&cwd_rev_path) {
                copy_file(rev_path, &cwd_rev_path)?;
            };
            Ok(())
        })
    }

// ------ newly added pub functions ------
    pub fn get_file_content(&self, item: &ItemInfo) -> Result<String, Errors> { // support cat 
        match item.get_file_id() {
        Some(id) => read_file_as_string(&path_compose(&self.paths.files, id)),
        None => Err(Errors::ErrStr(format!("Not a file: {}",item.get_file_name())))
        }
    }

    pub fn get_stage(&self) -> &Stage {
        &self.stage
    }

    pub fn clear_stage(&mut self) -> Result<(), Errors> {
        self.stage.clear();
        self.save()
    }


    pub fn add_file(&mut self, abs_path: &str) -> Result<(), Errors> {
        let mut temp_rev = Rev::new();
        temp_rev.add_file(abs_path)?;
        self.stage.to_add.extend(temp_rev.get_manifest().clone());
        
        temp_rev.manifest.iter().try_for_each(|(_, item)| self.save_file_to_repo(item).map(|_s| ()))?;
        self.save()
    }

    pub fn add_files(&mut self, abs_paths: &Vec<String>) -> Result<(), Errors> {   
        let mut temp_rev = Rev::new();
        abs_paths.iter().try_for_each(|path| temp_rev.add_file(path))?; // will abort if any errors appear
        
        self.stage.to_add.extend(temp_rev.get_manifest().clone());
        temp_rev.manifest.iter().try_for_each(|(_, item)| self.save_file_to_repo(item).map(|_s| ()))?;

        self.save()
    }

    pub fn remove_file(&mut self, abs_path:&str) -> Result<(), Errors>{
        self.remove_file_from_stage(abs_path)?; // automatically remove file in stage as well

        let mut temp_head = self.get_current_head()?;
        let f_to_remove = temp_head.remove_file(abs_path)?; // get the info from head

        self.stage.to_remove.insert(f_to_remove.get_file_wd_path().to_string(), f_to_remove);
        self.save()
    }

    pub fn remove_files(&mut self, abs_paths: &Vec<String>)-> Result<(), Errors> {
        abs_paths.iter().try_for_each(|path| self.remove_file(path))
    }

    pub fn remove_file_from_stage(&mut self, abs_path: &str) -> Result<(), Errors> {
        let entry = retrieve_info(abs_path)?;

        self.stage.to_add.remove(entry.get_file_wd_path());
        self.save()
    }

    pub fn remove_files_from_stage(&mut self, abs_paths:  &Vec<String>) -> Result<(), Errors> {
        abs_paths.iter().try_for_each(|path| self.remove_file_from_stage(path))
    }


    pub fn set_current_head(&mut self, set_head_to: &str) -> Result<(),Errors> {
        if !self.branch_heads.contains_key(set_head_to) {
            return Err(Errors::ErrStr(format!("Repository doesn't have the branch {}.", set_head_to)))
        }
        self.current_head = Some(set_head_to.to_string());
        self.save()
    }

    pub fn get_current_head_alias(&self) -> Option<&str> {
        self.current_head.as_deref()
    }
}


// ------ pub mod fns ------
pub fn init(opt_path: Option<&str>) -> Result<String, Errors> {
    let wd = match opt_path {
        Some(path) => path.to_string(),
        None => get_wd_path()
    };

    let paths = RepoPaths::new(&wd);
    if !is_path_valid(&paths.root) { // bypass recreating dirs if .dvcs already exists
        create_dir(&paths.files)?;
        create_dir(&paths.revs)?;
    }

    if !is_path_valid(&paths.repos) { // bypassing reinitializing repos if repos already exists
        let new_repo = Repo {
            current_head: None, // Some("main".to_string())
            branch_heads: HashMap::new(),
            paths: paths,
            stage: Stage::new(),
            // remote_head: None,
        };
        new_repo.save()?;
        Ok(format!("Successfully created new repository at {}", wd))
    } else {
        Ok(format!("Repository already exists at {}", wd))
    }
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
        None => Err(Errors::Errstatic("Directory untracked, fail to locate repository"))
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

    fn save_file_to_repo(&self, file: &ItemInfo) -> Result<&'static str, Errors>{
        let src = path_compose(&self.paths.wd, &file.get_file_wd_path());
        let dest = match file.get_file_id() {
            Some(id) => path_compose(&self.paths.files, id),
            None => return Ok("Not a file")
        };

        if is_path_valid(&dest) {
            return Ok("Exact file exists in repository");
        }

        match copy_file(&src, &dest) {
            Ok(_) => Ok("Successfully saved to repository"),
            Err(e) => Err(e)
        }
    }

}

// ------ private mod fns ------
pub (super) fn check_wd(wd_path: &str) -> Option<String> {
    if is_path_valid(&path_compose(wd_path, ".dvcs")) {
        return Some(wd_path.to_string());
    } else {
        match get_parent_name(wd_path) {
            Some(parent) => check_wd(&parent),
            None => None
        }
    }
}

pub (super) fn get_rel_path(abs_path: &str) -> Option<String> {
    let wd = get_name(&check_wd(abs_path)?)?; // search for the shallowest parent dir name that has .dvcs in it
    let rel_path = abs_path.rsplit_once(&wd)?.1.trim_start_matches(['/', '\\']);
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
        let compare_content = read_file_as_string(&path_compose(&matching_pool_path, &sha_id)).unwrap_or_default();
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
        use std::{fs, time::SystemTime};
        // might have cuncurrency issues when running tests in batch or use cargo test. Test one by one in order should work.
        // running individual tests multiple times could also fail because of certain duplication prevention

        static TEST_PATH: &str = "vc_test";


        fn get_test_paths()-> RepoPaths {
            let wd = get_wd_path();
            RepoPaths::new(&path_compose(&wd, TEST_PATH))
        }


        // 1.
        #[test]
        fn test_init_load() {
            let paths = get_test_paths();
            // delete_dir(&paths.root);
            clear_dir(&paths.wd, Vec::new());
            print!("{}", &paths.revs);
            assert!(init(Some(&paths.wd)).is_ok());
            assert!(fs::read_dir(paths.revs).is_ok());

            assert!(load(&paths.wd).is_ok());
            create_dir(&path_compose(&paths.wd, "test_dir"));

            assert!(load(&path_compose(&paths.wd, "test_dir")).is_ok());

        }

        // 2.
        #[test]
        fn test_add_stage_commit() -> Result<(), Errors> {
            let paths = get_test_paths();
            let mut repo = load(&paths.wd)?;
            repo.clear_stage()?;

            let sub_dir = path_compose(&paths.wd, "nested");
            if !is_path_valid(&sub_dir) {
                create_dir(&sub_dir)?;
            }

            // writing files
            let file_path_1 = path_compose(&paths.wd, "test_file1.txt");
            let file_path_nested = path_compose(&sub_dir, "test_file_nested.txt");
            // println!("debug print\nfile_path_1: {}",file_path_1);
            write_file(&file_path_1, &format!("test file root\n{:?}", SystemTime::now()))?;
            write_file(&file_path_nested, &format!("test file nested\n{:?}", SystemTime::now()))?;

            repo.add_file(&file_path_1)?;
            assert_eq!(repo.stage.get_add().len(), 1);
            repo.commit("test add commit")?;
            assert!(repo.stage.is_empty());

            repo.add_file(&file_path_nested)?;
            repo.add_files(&vec![file_path_1, file_path_nested])?;
            assert_eq!(repo.stage.get_add().len(), 2);
            repo.commit("test add commit 2")
        }

        // 3.
        #[test]
        fn test_branching() -> Result<(), Errors> {
            let paths = get_test_paths();
            let mut repo = load(&paths.wd)?;
            let file_path_1 = path_compose(&paths.wd, "branching_test.txt");
            write_file(&file_path_1, "branching test")?;
            match &repo.current_head {
                Some(alias) => {
                    if alias != "main" {
                        repo.set_current_head("main")?
                    }
                },
                None => ()
            };
            repo.add_file(&file_path_1)?;
            repo.commit("test branching")?;
            let head = repo.get_current_head()?;

            let new_head_alias = "branching-test";
            repo.new_head(new_head_alias,head.get_id().unwrap())?;
            assert_eq!(repo.branch_heads.len(), 2);
            repo.set_current_head(new_head_alias)?;
            assert_eq!(&repo.current_head, &Some(new_head_alias.to_string()));
            assert_eq!(&repo.branch_heads.get(new_head_alias), &repo.branch_heads.get("main"));

            write_file(&file_path_1, "test branching")?;
            repo.add_file(&file_path_1)?;
            repo.commit("committed on test branch")?;
            assert!(&repo.branch_heads.get(new_head_alias) != &repo.branch_heads.get("main"));
            repo.set_current_head("main")?;
            Ok(())
        }

        
        // 4.
        #[test]
        fn test_get_file_content()-> Result<(), Errors> {
            let paths = get_test_paths();
            let mut repo = load(&paths.wd)?;
            let head = repo.get_current_head()?;
            let manifest = head.get_manifest();
            for (k, v) in manifest.iter() {
                if v.is_file() {
                    repo.get_file_content(v);
                }
            };
            Ok(())
        }

    }