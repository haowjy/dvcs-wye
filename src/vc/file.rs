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
    pub fn get_content(&self) -> Option<String> { // get cached content
        let repo_storage_dir = get_repo_storage_dir()?;
        match &self.content_id { 
            Some(id) => read_file_as_string(&path_compose(&repo_storage_dir, &id)).ok(),
            None => None
        }
    }

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
        let repo_storage_dir = path_compose(wd, ".dvcs/files");
        let repo_storage_file = path_compose(&repo_storage_dir, self.content_id.as_ref()?);
        copy_file(&repo_storage_file, &wd_file_path).ok()
    }
    pub (super) fn save_to_repo(&mut self) -> Option<()> {
        let wd_root = get_wd_root()?;
        let repo_storage_dir = path_compose(&wd_root, ".dvcs/files");

        let content = read_file_as_string(&path_compose(&wd_root, self.loc_in_wd.as_ref()?)).ok()?;
        let mut new_id = checked_sha(&content, &repo_storage_dir);
        let storage_path = path_compose(&repo_storage_dir, &new_id);
        if !is_path_valid(&storage_path) {
            write_file(&storage_path, &content);
        }
        self.content_id = Some(new_id);
        Some(())
    }

}

// fn resolve_id_conflict(sha_id: &str, pool_path) -> Option<String> {
//     let repo_storage_path = path_compose(pool_path, sha_id);
//     if !is_path_valid(repo_storage_path) {
//         return None;
//     }
//     if content_    
// } 
pub (super) fn retrieve_info(abs_path: &str) -> Option<ItemInfo> {
    if !is_path_valid(abs_path) {
        println!("invalid path: {abs_path}"); // might replace with more organized error handling
        return None;
    }
    let rel_path = get_rel_path(abs_path)?;
    let meta = get_metadata(abs_path).ok()?; // *** ERROR HANDLING LATER
    let mut t = EntryType::Other;
    if meta.is_dir() {
        t = EntryType::Dir;
    }
    else if meta.is_file() {
        t = EntryType::Afile;
    }
        
    let info = ItemInfo {
        name: get_name(&rel_path),
        loc_in_wd: Some(rel_path),
        content_id: None,
        entry_type: t,
    };

    Some(info)
}

fn get_repo_storage_dir() -> Option<String> {
    let wd_root = get_wd_root()?;
    Some(path_compose(&wd_root, ".dvcs/files"))
}

#[cfg(test)]
mod tests {
    use super::*;

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
