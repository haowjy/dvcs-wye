use std::collections::HashMap;
use std::time::SystemTime;
use std::path::PathBuf; // needs to be moved to DSR

// external crates:
use petgraph::graphmap::DiGraphMap;
use sha2::{Sha256, Digest};
// use serde::{Serialize, Deserialize};

mod dsr;
use crate::dsr::*; //not working?

mod Repository {
    #[derive(Debug)] 
    pub struct Repo {
        current_head: Option<&str>, // sha_rev_id 
        branch_heads: Option<Hashmap<&str, &str>>, // <K=alias, V=sha_rev_id>
        paths: RepoPaths,
        revs: DiGraphMap<_, _>,
    }

    #[derive(Debug)] 
    pub(crate) struct RepoPaths { 
        // wd: &str,// inconsistent types for paths, might need better type representation
        wd: String,
        root: String,
        files: String,
        revs: String,
        head: String, // THE current head
        branch_heads: String,
    }
    
    impl RepoPaths {
        fn new(WD: &str) -> RepoPaths {
            let root = WD.clone().push(".dvcs"); // better to be wrapped in DSR like:
            let root = path_compose(WD, ".dvcs");
            // ideal form: path_compose(&str: component1, &str: component2) -> String { // with PathBuf::from and outputs .to_str} // maybe with Result/Option enum wrapper
            RepoPaths {
                wd: WD.clone(),
                root: root.clone(),
                files: path_compose(root, "files"),
                revs: path_compose(root, "revs"),
                head: path_compose(root, "head"),
                branch_heads: path_compose(root, "branches"),
            }
        }
    }

    impl Repo {

        fn save(&self) -> () { // Result<(), ()> { 
            write_file(self.paths.head, serialize(self.current_head));
            write_file(self.paths.branch_heads, serialize(self.branch_heads));
            // additional writing operations possible
        }
    }

    pub fn init() -> Repo { // Result<(),()>{ // error handling to be impl
        let WD = get_wd_path();
        let paths = RepoPaths::new(WD);
        // ***** error handling needed *****
        // esp: handle running init again with existing repo
        create_dir(&paths.files); // root .dvcs automatically added
        create_dir(&paths.revs);
        // create_file(&paths.branch_heads); 
        // create_file(&paths.head);
        let new_repo = Repo {
            current_head: None,
            branch_heads: None,
            paths: paths,
        }
        new_repo.save();
        return new_repo;
    }

    pub fn load(WD: &str) -> Repo { // Result<Repo, ()>
        let paths = RepoPaths::new(WD);
        let load_repo = Repo {
            current_head: read_file(paths.head), //?
            branch_heads: read_file(paths.branch_heads), //?
            paths: paths,
        }
        return load_repo;
    }

    pub (crate) fn sha<T: AsRef<[u8]> + ?Sized> (data: &T) -> String {
        format!("{:x}", Sha256::digest(data))
    }
    
    // preliminary fn might change later or make a trait
    pub (crate) fn sha_match<'a, T: Clone + Iterator + Iterator<Item=&'a String>> (sha: &'a String, pool: T) -> Vec<&'a String> {
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
}


mod Revision {
    pub struct Rev {
        manifest: HashMap<&str, FileInfo>,  //<K: wd_relative_path, V: FileInfo: file content id and metadata id
        user_id: Option<String>,
        time_stamp: SystemTime,
        
        

    }

    impl Rev {

    }


}

mod File {
    #[derive(Debug, Serialize, Deserialize)] 
    pub struct FileInfo {
        wd_loc: String, 
        content: String, // sha_id
        metadata: FileMetaData, //or String //sha_id?
    }

    #[derive(Debug, Serialize, Deserialize)] 
    struct FileMetaData {

    }

    impl FileInfo {
        fn ()
    }
}

