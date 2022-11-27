#[allow(dead_code)]
#[allow(unused_imports)]
use std::collections::HashMap;
use std::time::SystemTime;
use std::fs;
// external crates:

use serde::{Serialize, Deserialize};
use crate::vc::repository::*;
use crate::dsr::*;

static STORAGE_DIR: &str =".dvcs/files"; //relative path hard coded, might change to use lazy_static macro 

// trait Info {
//     loc_in_wd: String,
//     content_id: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, Hash)] 
pub struct ItemInfo {
    name: Option<String>, // last component of path. Can be directory?
    loc_in_wd: Option<String>, // wd RELATIVE path
    content_id: Option<String>, // sha_id, only make an id when cached in repos (usually via commit)
    entry_type: EntryType,
    // metadata: Option<FileMetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash)] 
pub enum EntryType {
    Afile,
    Dir, 
    Other 
}


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
    // MOVED TO Repo!
    // pub fn get_content(&self) -> Option<String> { // get cached content
    //     match &self.content_id { 
    //         Some(id) => read_file_as_string(path_compose(STORAGE_DIR, id).as_str()).ok(),
    //         None => None
    //     }
    // }

    pub fn get_file_wd_path(&self) -> Option<String> {
        self.loc_in_wd.clone()
    }

    // pub fn get_cached_metadata(&self) -> Option<fs::Metadata> {
    //     match &self.content_id { 
    //         Some(id) => get_metadata(path_compose(STORAGE_DIR, id).as_str()).ok(),
    //         None => None
    //     }
    // }

    // new pub fn, assisting make_wd
    // trial only, might move to repos
    pub fn make_file(&self, wd:&str) -> Option<()> {
        let wd_file_path = path_compose(wd, &self.loc_in_wd.as_ref()?);
        let repo_storage_path = path_compose(wd, self.content_id.as_ref()?);
        copy_file(&repo_storage_path, &wd_file_path).ok()
    }
}

pub (crate) fn retrieve_info(wd_path: &str) -> Option<ItemInfo> {
    if !is_path_valid(wd_path) {
        println!("invalid path");
        return None;
    }
    let meta = get_metadata(wd_path).ok()?; // *** ERROR HANDLING LATER
    let mut t = EntryType::Other;
    if meta.is_dir() {
        t = EntryType::Dir;
    }
    else if meta.is_file() {
        t = EntryType::Afile;
    }
        
    let info = ItemInfo {
        name: get_name(wd_path),
        loc_in_wd: Some(wd_path.to_string()),
        content_id: None,
        entry_type: t,
    };

    Some(info)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        let info = retrieve_info("/Users/yiyangw/Documents/dvcs_test/folder2"); // external file, only works on dev local machine 
        let json_str = serde_json::to_string(&info).unwrap();
        println!("{}", json_str)
    }

    #[test]
    fn relative_path_info_retrieval() {
        let info = retrieve_info("./src/vc/file.rs").unwrap();
        assert_eq!(&info.name.unwrap(), "file.rs");
    }
}
