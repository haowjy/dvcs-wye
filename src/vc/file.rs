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

    // content_id: Option<String>, // sha_id
    // entry_type: EntryType,
    // metadata: Option<FileMetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq)] 
pub enum EntryType {
    File(String),
    Dir, 
    Other 
}

use EntryType::{File, Dir, Other};

// #[derive(Debug, Serialize, Deserialize)] 
// struct FileMetaData {
//     // created: bool,
//     // is_file: bool, 
//     is_dir: bool,
//     // len: u64,
// }

// impl FileMetaData {
//     fn retrieve(&wd_file_path) -> FileMetaData {
//         let meta = fs::metadata(wd_file_path);
//         FileMetaData {
//             is_dir: meta.is_dir();
//         }
//     }
// }

impl ItemInfo {
    // CHANGE INCOMING: WILL MOVE TO Repo's get_file()
    pub fn get_content(&self) -> Result<String, Errors> { // get cached content
        
        match self.entry {
            File(id) => {
                let repo_storage_dir = get_repo_storage_dir().ok_or(Errors::ErrUnknown)?;
                read_file_as_string(&path_compose(&repo_storage_dir, &id))
            },
            _ => Err(Errors::ErrStr(format!("{}Not a File", self.name)))
        }
    }

    pub fn get_file_wd_path(&self) -> &str {
        &self.loc_in_wd
    }

    // new pub fn, assisting make_wd
    // trial only, might move to repos
    pub fn make_file(&self, wd:&str) -> Result<(), Errors> {     // *** ERROR HANDLING
        let wd_file_path = path_compose(wd, &self.loc_in_wd);
        let repo_storage_dir = path_compose(wd, ".dvcs/files");
        match self.entry {
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
    //     let wd_root = get_wd_root()?;
    //     let repo_storage_dir = path_compose(&wd_root, ".dvcs/files");

    //     copy_file(self.
    //         path_compose(repo_storage_dir, &self.content_id))

    //     let content = read_file_as_string(&path_compose(&wd_root, &self.loc_in_wd))?;


    //     let mut new_id = checked_sha(&content, &repo_storage_dir);
    //     let storage_path = path_compose(&repo_storage_dir, &new_id);

    //     ()
    //     if !is_path_valid(&storage_path) {
    //         write_file(&storage_path, &content);
    //     }
    //     self.content_id = Some(new_id);
    //     Ok(())
    // }

}
// fn resolve_id_conflict(sha_id: &str, pool_path) -> Option<String> {
//     let repo_storage_path = path_compose(pool_path, sha_id);
//     if !is_path_valid(repo_storage_path) {
//         return None;
//     }
//     if content_    
// } 
pub fn retrieve_info(abs_path: &str) -> Result<ItemInfo, Errors> {
    let rel_path = get_rel_path(abs_path).ok_or(Errors::ErrStr(format!("Cannot find the proper repository path for file {abs_path}")))?;

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


pub fn get_id(file_path: &str) -> Result<String, Errors> {
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
