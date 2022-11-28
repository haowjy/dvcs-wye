
#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
use std::time::SystemTime;

// external crates:
use petgraph::graphmap::DiGraphMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::dsr::*;
use crate::vc::repository::*;
use crate::vc::file::*;

static REV_DIR: &str =".dvcs/revs"; //relative path hard coded, might change to use lazy_static macro 


#[derive(Debug, Clone, Serialize, Deserialize)] // might impl Drop trait for save safety
pub struct Rev {
    rev_id: Option<String>,
    parent_id: Option<String>,
    user_id: Option<String>,
    time_stamp: SystemTime,
    manifest: HashMap<String, ItemInfo>,  // Hashmap<K: wd_relative_path, V: ItemInfo: file content id and metadata id
    // path_self: String,
}



impl Rev {
    // *** deserialize not implemented, use serde_json for now
    pub fn from(path: &str) -> Option<Rev> {
        serde_json::from_str(&read_file_as_string(path).ok()?).ok()
        // rev.path_self = path.to_string();
        // Some(rev)
    }

    // *** serialize not implemented, use serde_json for now
    pub (super) fn save(&self, rev_path: &str) -> Option<()> { //need to figure out destructor
        match &self.rev_id {
            Some(id) => write_file(&path_compose(rev_path, &id), &serde_json::to_string(&self).ok()?).ok(),
            None => None
        }
    }

    pub (super) fn new() -> Rev {
        Rev {
            rev_id: None,
            parent_id: None,
            user_id: None,
            time_stamp: SystemTime::now(),
            manifest: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> Option<&str> {
        match self.rev_id.as_ref() {
            
            Some(x) => Some(x.as_str()),
            None => None
        }
    }

    pub fn get_parent_id(&self) -> Option<&str> {
        match self.parent_id.as_ref() {
            Some(x) => Some(x.as_str()),
            None => None
        }
    }
    
    pub fn get_manifest(&self) -> &HashMap<String, ItemInfo> {
        &self.manifest
    }

    // NOTE: current vc doesn't track files moving from one subdirectory to another, 
    pub (super) fn add_file(&mut self, abs_path: &str) -> Option<()> {
        let rel_path = get_rel_path(abs_path)?;
        if self.manifest.contains_key(&rel_path) {
            return None;
        }

        let new_entry:ItemInfo = retrieve_info(abs_path)?;
        self.manifest.insert(rel_path, new_entry);
        return Some(());
    }

    pub (super) fn remove_file(&mut self, abs_path: &str) -> Option<()> {None} // *** to be impl 

    pub (super) fn update_time(&mut self) -> &Self {
        self.time_stamp = SystemTime::now();
        self
    }

    pub fn store_files(&mut self, path: &str) -> Option<()> {// *** to be implemented
        for (_, info) in self.manifest.iter_mut() {
            info.save_to_repo()?;
        }
    //         
        Some(())
    }

    pub (super) fn gen_id(&mut self) -> Option<String> {
        let wd = get_wd_root()?;
        let revs_dir = path_compose(&path_compose(&wd, ".dvcs"), "revs");

        self.parent_id = self.rev_id.clone(); // "inheritance"
        self.rev_id = None;

        let id = checked_sha(&serde_json::to_string(&self.clone()).ok()?, &revs_dir);
        self.rev_id = Some(id.clone());
        Some(id)
    }
}


//     pub fn remove_file(&mut self, abs_path:&str) -> Self {
        // self.manifest
    // }
//     }
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize() { // with serde_json
        let rev = Rev::new();
        let json_str = serde_json::to_string(&rev).unwrap();
        // println!("{}", json_str);
        let rev_other: Rev = serde_json::from_str(&json_str).unwrap();
        assert_eq!(rev.time_stamp, rev_other.time_stamp); // since Eq not derived
    }
    
    #[test]
    fn get_data_types() {
        let mut rev = Rev::new();
        rev.rev_id = Some("sha_string".to_string());
        rev.parent_id = Some("sha_string_parent".to_string());
        assert_eq!(rev.get_id(), Some("sha_string"));
        assert_eq!(rev.get_parent_id(), Some("sha_string_parent"));
        rev.get_id(); // assure the struct is intact after get 
    }

    #[test]
    #[should_panic]
    fn time_update() {
        let mut rev = Rev::new();
        let time = rev.time_stamp.clone();
        assert_eq!(&time, &rev.time_stamp); // won't panic
        rev.update_time();
        assert_eq!(time, rev.time_stamp); // panic
    }

    // #[test]
    // fn file_add() {
    //     let mut rev = Rev::new();
    //     let add_result = rev.add_file("./src/vc/repository.rs");
    //     assert_eq!(add_result, Some(()));
    //     let add_result2 = rev.add_file("./src/vc/repository.rs");
    //     assert_eq!(add_result2, None);
    // }
}

