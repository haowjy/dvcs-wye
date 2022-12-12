#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
// use std::time::SystemTime;
// use std::fs;
// external crates:

use serde::{Serialize, Deserialize};
use crate::ui::Errors;
use crate::vc::repository::*;
use crate::dsr::*;


#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq)] 
pub struct ItemInfo {
    name: String, // last component of path. Can be directory?
    loc_in_wd: String, // wd RELATIVE path
    entry: EntryType
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq)] 
pub enum EntryType {
    File(String),// String: content SHA id
    Dir, 
    Other 
}

use EntryType::{File, Dir, Other};

impl ItemInfo {
// ------ ItemInfo methods ------
    pub fn get_file_name(&self) -> &str {
        &self.name
    }

    pub fn get_file_wd_path(&self) -> &str {
        &self.loc_in_wd
    }

    pub fn is_file(&self) -> bool {
        match &self.entry {
            File(_) => true,
            _ => false
        }
    }
    
    pub fn get_file_id(&self) -> Option<&str> {
        match &self.entry {
            File(id) => Some(&id),
            _ => None
        }
    }

    // new pub fn, assisting make_wd
    pub fn make_file(&self, wd:&str) -> Result<(), Errors> {    
        let wd_file_path = path_compose(wd, &self.loc_in_wd);
        let repo_storage_dir = path_compose(wd, ".dvcs/files");
        match &self.entry {
            File(id) => {
                let repo_storage_file = path_compose(&repo_storage_dir, &id);
                copy_file(&repo_storage_file, &wd_file_path)
            },
            Dir => {
                if !is_path_valid(&wd_file_path) {
                    create_dir(&wd_file_path)
                } else {
                    Ok(())
                }

            },
            Other => Ok(()) // bypassing Other
        }
    }

}

// ------ mod functions ------
pub fn retrieve_info(abs_path: &str) -> Result<ItemInfo, Errors> {
    let rel_path = get_rel_path(abs_path).ok_or(Errors::ErrStr(format!("Cannot find the proper working directory path for file {abs_path}")))?;

    let meta = get_metadata(abs_path)?; 
    let mut entry = EntryType::Other;
    if meta.is_dir() {
        entry = EntryType::Dir;
    }
    else if meta.is_file() {
        let id = get_id(abs_path)?;
        entry = EntryType::File(id);
    }
    
    Ok(ItemInfo {
        name: get_name(&rel_path).ok_or(Errors::ErrStr(format!("Fail to get file name from path {abs_path}")))?,
        loc_in_wd: rel_path,
        entry: entry
    })
}

fn get_id(file_path: &str) -> Result<String, Errors> {
    let wd_root = check_wd(file_path).ok_or(Errors::ErrStr(format!("unable to locate a repository in {}", file_path)))?;
    let repo_storage_dir = path_compose(&wd_root, ".dvcs/files");

    let content = read_file_as_string(file_path)?;

    Ok(checked_sha(&content, &repo_storage_dir))
}




#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_dir_file() {
        let new_dir = "vc_test/file_test_dir";
        if !is_path_valid(new_dir) {
            create_dir(new_dir);
        };
        let new_file = path_compose(new_dir, "file_test.txt");
        write_file(&new_file, "file tests");
    }

    // 1.
    #[test]
    fn test_serialize_deserialize() -> Result<(), Errors>{
        make_test_dir_file();
        let tracked_file_path = "./vc_test/file_test_dir/file_test.txt"; 

        let info = retrieve_info(tracked_file_path)?; // relative path is fine
        let json_str = serialize(&info).unwrap();
        assert!(json_str.contains("name"));
        let re_info:ItemInfo = serde_json::from_str(&json_str).unwrap();

        assert_eq!(re_info.get_file_id(), info.get_file_id());
        Ok(())
    }


    // 2.
    #[test]
    fn test_abs_rel_path_info_retrieval() -> Result<(), Errors> {
        make_test_dir_file();
        let rel_path = "./vc_test/file_test_dir/file_test.txt";
        let abs_path = path_compose(&get_wd_path(), "vc_test/file_test_dir/file_test.txt");
        let rel_info = retrieve_info(rel_path)?;
        let abs_info = retrieve_info(&abs_path)?;
    
        assert_eq!(&rel_info.name, "file_test.txt");
        assert_eq!(&abs_info.name, "file_test.txt");
        Ok(())
    }


    // 3.
    #[test]
    fn test_dir() -> Result<(), Errors> {
        let new_dir = "vc_test/file_test_dir";
        if !is_path_valid(new_dir) {
            create_dir(new_dir)?;
        };
        let info = retrieve_info(new_dir)?;
        assert_eq!(&info.name, "file_test_dir");
        assert_eq!(&info.entry, &EntryType::Dir);
        assert!(!info.is_file());
        assert!(info.get_file_id().is_none());
        Ok(())
    }

    // 4.
    #[test]
    fn test_get() -> Result<(), Errors> {
        make_test_dir_file();
        let tracked_file_path = "./vc_test/file_test_dir/file_test.txt"; 
        let info = retrieve_info(tracked_file_path)?;

        assert!(info.get_file_id().is_some());
        assert_eq!(info.get_file_name(), &info.name);
        assert_eq!(info.get_file_wd_path(), "file_test_dir/file_test.txt");
        Ok(())
    }


    // 5.
    #[test]
    fn test_make_file_dir() -> Result<(), Errors> { // bypassing make_file for File(String) type since it requires a source in repo (cannot formulate without calling repo)
        make_test_dir_file();
        let tracked_dir = "./vc_test/file_test_dir"; 
        let mut info = retrieve_info(&tracked_dir)?;
        info.loc_in_wd.push_str("/make_new");
        let wd = "./vc_test";
        info.make_file(&wd)
    }


    // 6.
    #[test]
    #[should_panic]
    fn test_make_file() {
        make_test_dir_file();
        let tracked_file_path = "./vc_test/file_test_dir/file_test.txt"; 
        let info = retrieve_info(tracked_file_path).unwrap();
        let wd = "./vc_test";
        info.make_file(wd).unwrap() // should panic bc file_test is not in .dvcs/files
    }

}
