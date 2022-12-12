use std::collections::HashMap;
use std::time::SystemTime;
// external crates:
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

use crate::dsr::*;
use crate::ui::Errors;
use crate::vc::repository::*;
use crate::vc::file::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rev {
    rev_id: Option<String>,
    parent_id: Option<String>, 
    parent_id2: Option<String>,
    user_id: Option<String>,
    time_stamp: SystemTime,
    message: Option<String>,
    pub (super) manifest: HashMap<String, ItemInfo>,  // Hashmap<K: wd_relative_path, V: ItemInfo: file content id and metadata id
}


impl Rev {
    // ------ pub Rev methods ------
    pub fn from(path: &str) -> Result<Rev, Errors> {
        match serde_json::from_str(&read_file_as_string(path)?){
            Ok(rev) => Ok(rev),
            Err(e) => Err(Errors::ErrSerde(e)),
        }
        // rev.path_self = path.to_string();
        // Some(rev)
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

    // ------ private Rev methods ------

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

    pub (super) fn save(&self, rev_path: &str) -> Result<(), Errors> {
        match &self.rev_id {
            Some(id) => write_file(&path_compose(rev_path, &id), &serialize(&self)?),
            None => Err(Errors::ErrStr("Unable to save repo without id".to_string()))
        }
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


#[cfg(test)]
mod tests {
    use super::*;

    // 1.
    #[test]
    fn test_log() {
        let mut rev = Rev::new();
        let log = rev.get_log();
        assert!(log.get("id").is_some());
        assert!(log.get("time").is_some());
        assert_eq!(log.get("message").unwrap(), "");

        let new_msg = "some message";
        rev.set_message(new_msg);
        let log = rev.get_log();
        assert_eq!(log.get("message").unwrap(), new_msg);

    }

    // 2.
    #[test]
    fn test_gets() {
        let mut rev = Rev::new();
        rev.rev_id = Some("sha_string".to_string());
        rev.parent_id = Some("sha_string_parent".to_string());
        assert_eq!(rev.get_id(), Some("sha_string"));
        assert_eq!(rev.get_parent_id(), Some("sha_string_parent"));
        rev.get_id(); // assure the struct is intact after get 
    }

    // 3.
    #[test]
    #[should_panic]
    fn test_time_update() {
        // might fail if elapse between two System::now() is not captured, but the update works
        let mut rev = Rev::new();

        let time = rev.time_stamp.clone();
        assert_eq!(&time, &rev.time_stamp); // won't panic
        
        rev.update_time();
        assert_eq!(time, rev.time_stamp); // panic
    }

    // 4.
    #[test]
    fn file_add_remove() -> Result<(), Errors> {
        let mut rev = Rev::new();
        let file_path = "./vc_test/test_file1.txt"; // relative path is fine
        rev.add_file(file_path)?;
        assert_eq!(rev.manifest.len(), 1);

        rev.remove_file(file_path)?;
        assert_eq!(rev.manifest.len(), 0);
        Ok(())
    }

    // 5.
    #[test]
    fn test_gen_id_parent_update() -> Result<(), Errors> {
        let mut rev = Rev::new();
        let rev_path = "./vc_test/.dvcs/revs";
        let id1 = rev.gen_id(rev_path)?;
        assert!(&rev.parent_id.is_none());

        let id2 = rev.gen_id(rev_path)?;
        assert!(&id1 != &id2);
        assert_eq!(&rev.parent_id.unwrap(), &id1);

        Ok(())
    }

    // 6.
    #[test]
    fn test_save() -> Result<(), Errors> {
        let mut rev = Rev::new();
        let rev_path = "./vc_test/.dvcs/revs";
        let id = rev.gen_id(rev_path)?;
        rev.save(rev_path)?;
        assert!(is_path_valid(&path_compose(rev_path, &id)));
        Ok(())
    }
}

