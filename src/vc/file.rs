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

#[derive(Debug, Serialize, Deserialize)] 
pub struct FileInfo {
    file_name: Option<String>, // last component of path. Can be directory?
    loc_in_wd: Option<String>, // wd relative path
    content_id: Option<String>, // sha_id, only make an id when cached in repos (usually via commit)
    // metadata: Option<FileMetaData>,
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

impl FileInfo {
    pub fn get_content(&self) -> Option<String> { // get cached content
        match &self.content_id { 
            Some(id) => read_file_as_string(path_compose(STORAGE_DIR, id).as_str()).ok(),
            None => None
        }
        
    }

    pub fn get_file_wd_path(&self) -> Option<String> {
        self.loc_in_wd.clone()
    }

    pub fn get_cached_metadata(&self) -> Option<fs::Metadata> {
        match &self.content_id { 
            Some(id) => get_metadata(path_compose(STORAGE_DIR, id).as_str()).ok(),
            None => None
        }
    }




    // pub fn get
    // pub (crate) fn make_file(&self, &wd:&str) -> io::Result() {
    //     let path_to_file = path_compose(wd, self.loc_in_wd);
    //     copy_file()
    //     write_file(path_to_file, )
    // }
}

pub (super) fn retrieve_info(wd_file_path: &str) -> Option<FileInfo> {
    if !is_path_valid(wd_file_path) {
        return None
    };

    let info = FileInfo {
        file_name: get_name(wd_file_path),
        loc_in_wd: Some(wd_file_path.to_string()),
        content_id: None,
    };

    Some(info)

    // let content_id match read_file_as_string(&wd_file_path).ok() {
    //     Some(x) => sha(x), 
    //     None => None
    // };
    // let metadata = FileMetadata {
        
    // }
    // FileInfo {
    //     loc_in_wd: wd_file_path.to_string();
    //     content_id: content_id,
    //     metadata: fs::metadata(wd_file_path),
    // }

}

#[cfg(test)]
mod tests {
    
    // #[test]
    // fn test_get_content(&self) -> {
    //     FileInfo
    // }
}
