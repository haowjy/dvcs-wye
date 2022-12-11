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


// static STORAGE_DIR: &str =".dvcs/files"; //relative path hard coded, might change to use lazy_static macro 

// trait Info {
//     loc_in_wd: String,
//     content_id: String,
// }

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
    pub fn get_content(&self) -> Result<String, Errors> { // get cached content
        
        match &self.entry {
            File(id) => {
                let repo_storage_dir = get_repo_storage_dir().ok_or(Errors::ErrUnknown)?;
                read_file_as_string(&path_compose(&repo_storage_dir, &id))
            },
            _ => Err(Errors::ErrStr(format!("{}Not a File", self.name)))
        }
    }

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
    // trial only, might move to repos
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


    // // MOVING TO repository
    // pub (super) fn save_to_repo(&mut self) -> Result<(), Errors>{ 
    // }

}

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
    let wd_root = get_wd_root()?;
    let repo_storage_dir = path_compose(&wd_root, ".dvcs/files");

    let content = read_file_as_string(file_path)?;

    Ok(checked_sha(&content, &repo_storage_dir))
}

fn get_repo_storage_dir() -> Option<String> {
    let wd_root = get_wd_root().ok()?;
    Some(path_compose(&wd_root, ".dvcs/files"))
}



#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn serialization() {
    //     let info = retrieve_info("/Users/yiyangw/Documents/dvcs_test/folder2"); // external file, only works on dev local machine 
    //     let json_str = serde_json::to_string(&info).unwrap();
    //     println!("{}", json_str)
    // }

    // #[test]
    // fn relative_path_info_retrieval() {
    //     let info = retrieve_info("./src/vc/file.rs").unwrap();
    //     assert_eq!(&info.name.unwrap(), "file.rs");
    // }
}
