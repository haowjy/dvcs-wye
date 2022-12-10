
#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
use std::time::SystemTime;
// external crates:
// use petgraph::graphmap::DiGraphMap;
// use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

use crate::dsr::*;
use crate::ui::Errors;
use crate::vc::repository::*;
use crate::vc::file::*;

static REV_DIR: &str =".dvcs/revs"; //relative path hard coded, might change to use lazy_static macro 


#[derive(Debug, Clone, Serialize, Deserialize)] // might impl Drop trait for save safety
pub struct Rev {
    rev_id: Option<String>,
    parent_id: Option<String>, 
    parent_id2: Option<String>,
    user_id: Option<String>,
    time_stamp: SystemTime,
    manifest: HashMap<String, ItemInfo>,  // Hashmap<K: wd_relative_path, V: ItemInfo: file content id and metadata id
    // path_self: String,
}



impl Rev {
    // *** deserialize not implemented, use serde_json for now
    pub fn from(path: &str) -> Result<Rev, Errors> {
        match serde_json::from_str(&read_file_as_string(path)?){
            Ok(rev) => Ok(rev),
            Err(e) => Err(Errors::ErrSerde(e)),
        }
        // rev.path_self = path.to_string();
        // Some(rev)
    }

    // *** serialize not implemented, use serde_json for now
    pub (super) fn save(&self, rev_path: &str) -> Result<(), Errors> { //need to figure out destructor
        match &self.rev_id {
            Some(id) => write_file(&path_compose(rev_path, &id), &serialize(&self)?),
            None => Err(Errors::ErrStr("Unable to save repo without id".to_string()))
        }
    }

    pub (super) fn new() -> Rev {
        Rev {
            rev_id: None,
            parent_id: None,
            parent_id2:None,
            user_id: None,
            time_stamp: SystemTime::now(),
            manifest: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> Option<&str> {
        self.rev_id.as_deref()
    }

    pub fn get_parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    pub fn get_manifest(&self) -> &HashMap<String, ItemInfo> {
        &self.manifest
    }

    pub fn get_log(&self) -> HashMap<&'static  str, String> {
        let log = HashMap::new();
        log.insert("id", self.rev_id.unwrap_or_default());
        log.insert("user", self.user_id.unwrap_or("Unknown".to_string()));
        log.insert("time", self.get_date_time());
        log
    }

    // NOTE: current vc doesn't track files moving from one subdirectory to another, 
    pub (super) fn add_file(&mut self, abs_path: &str) -> Result<(), Errors> {
        Err(Errors::ErrUnknown) // *** TO BE IMPL

    // pub (super) fn add_file(&mut self, staged_add: ItemInfo) -> Result<(), Errors> {
    //     let wd_path = staged_add.get_file_wd_path();
    //     self.manifest.
    // }
    //     let rel_path = get_rel_path(abs_path).ok_or(Errors::ErrUnknown)?; // change later
    //     if self.manifest.contains_key(&rel_path) {
    //         return None;
    //     }

    //     let new_entry:ItemInfo = retrieve_info(abs_path)?;
    //     self.manifest.insert(rel_path, new_entry);
    //     return Some(());
    }

    pub (super) fn remove_file(&mut self, abs_path: &str) -> Option<()> {None} // *** to be impl 

    pub (super) fn update_time(&mut self) -> &Self {
        self.time_stamp = SystemTime::now();
        self
    }

    // *** To be Changed
    pub (super) fn store_files(&mut self, path: &str) -> Result<(), Errors> {
        for (_, info) in self.manifest.iter_mut() {
            ()
            // info.save_to_repo()?;
            
        }
    //         
        Ok(())
    }

    pub (super) fn gen_id(&mut self) -> Result<String, Errors> {
        let wd = get_wd_root()?;
        let revs_dir = path_compose(&path_compose(&wd, ".dvcs"), "revs");

        self.parent_id = self.rev_id.clone(); // "inheritance"
        self.rev_id = None;

        let id = checked_sha(&serialize(&self.clone())?, &revs_dir);
        self.rev_id = Some(id.clone());
        Ok(id)
    }

    pub(super) fn set_user(&mut self, user_info: &str) -> &Self {
        self.user_id = Some(user_info.to_string());
        self
    }
    
    fn get_date_time(&self) -> String{
        DateTime::<Local>::from(self.time_stamp).to_string()
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

