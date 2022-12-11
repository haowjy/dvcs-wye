
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
    message: Option<String>, // to be added
    pub (super) manifest: HashMap<String, ItemInfo>,  // Hashmap<K: wd_relative_path, V: ItemInfo: file content id and metadata id
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
            message: None,
            manifest: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> Option<&str> {
        self.rev_id.as_deref()
    }

    pub fn get_parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }
    
    pub fn get_parent_id2(&self) -> Option<&str> {
        self.parent_id2.as_deref()
    }

    pub fn get_manifest(&self) -> &HashMap<String, ItemInfo> {
        &self.manifest
    }

    pub fn get_log(&self) -> HashMap<&'static  str, String> {
        let mut log = HashMap::new();
        log.insert("id", self.rev_id.clone().unwrap_or_default());
        log.insert("user", self.user_id.clone().unwrap_or("Unknown".to_string()));
        log.insert("time", self.get_date_time());
        log.insert("message", self.message.clone().unwrap_or_default());
        log
    }

    // NOTE: current vc doesn't track files moving from one subdirectory to another, 
    pub (super) fn add_file(&mut self, abs_path: &str) -> Result<(), Errors> {
        let new_entry = retrieve_info(abs_path)?;
        // if self.manifest.contains_key(new_entry.get_file_wd_path()) {
        // }

        self.manifest.insert(new_entry.get_file_wd_path().to_string(), new_entry);
        
        Ok(())
    }

    pub (super) fn remove_file(&mut self, abs_path: &str) -> Result<ItemInfo, Errors>{
        let entry = retrieve_info(abs_path)?;
        let entry_loc_path = entry.get_file_wd_path();
        if !self.manifest.contains_key(entry_loc_path) {
            return Err(Errors::Errstatic("Unable to remove untracked file"));
        }
        self.manifest.remove(entry_loc_path).ok_or(Errors::Errstatic("Unable to remove untracked file"))
    } 

    pub (super) fn update_time(&mut self) -> &Self {
        self.time_stamp = SystemTime::now();
        self
    }

    // pub (super) fn store_files(&mut self, path: &str) -> Result<(), Errors> {
    //     self.manifest.iter().try_for_each(|_, info|{
    //         info.save_to_repo()?

    //         // info.save_to_repo()?;
            
    //     });
    // //         
    //     Ok(())
    // }

    pub (super) fn gen_id(&mut self, rev_path: &str) -> Result<String, Errors> {

        self.parent_id = self.rev_id.clone(); // "inheritance"
        self.rev_id = None;

        let id = checked_sha(&serialize(&self.clone())?, &rev_path);
        self.rev_id = Some(id.clone());
        Ok(id)
    }

    pub(super) fn set_user(&mut self, user_info: &str) -> &Self {
        self.user_id = Some(user_info.to_string());
        self
    }

    pub(super) fn set_message(&mut self, msg: &str) -> &Self {
        self.message = Some(msg.to_string());
        self
    }

    pub(super) fn set_parent_2(&mut self, parent_2: &str) -> &Self {
        self.parent_id2 = Some(parent_2.to_string());
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

