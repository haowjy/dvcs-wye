use std::{fs, io, env};
use std::fs::Metadata;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use users::{get_user_by_uid, get_current_uid};

use crate::vc::file::ItemInfo;
use crate::vc::revision::Rev;

// ==================================
//        PRIVATE FUNCTIONS
// ==================================

// check if the file/dir name contains forbidden character(s)
fn is_name_valid(name: &str) -> bool {
    let os = env::consts::OS;
    match os {
        "linux" => {
            if name.contains('/') {
                eprintln!("error: Name({}) contains forbidden ASCII character(s)", name);
                return false
            }
        }, "macos" => {
            if name.contains(':') {
                eprintln!("error: Name({}) contains forbidden ASCII character(s)", name);
                return false
            }
        }, "windows" => {
            if name.contains('<') || name.contains('>') || name.contains(':') ||
               name.contains('"') || name.contains('/') || name.contains('\\') ||
               name.contains('|') || name.contains('?') || name.contains('*') {
                eprintln!("error: Name({}) contains forbidden ASCII character(s)", name);
                return false
            }
        }, _ => {
            //eprintln!("WARN<DSR-01>: Operating System: <{}> is not supported", os);
            return true
        },
    };
    true
}

// ==================================
//        PUBLIC FUNCTIONS
// ==================================

// 3. Create a directory and directory within recursively if missing
//     If you would like to create a hidden folder, add a . in front
//      of the folder name, i.e. "folder1/folder2/.git"
// USEAGE: create_dir("folder1/folder2/folder3/.hidden_folder");
pub fn create_dir(path: &str) -> io::Result<()> {
    let folder_name = get_name(path).unwrap();
    if is_path_valid(path) {
        eprintln!("warning: Directory <{}> has already created", folder_name);
    } else if !is_name_valid(&folder_name) {
        return Err(Error::new(ErrorKind::Unsupported, "error: Invalid directory name format"));
    }
    fs::create_dir_all(path)
}

// 4. Remove a directory at this path, after removing all its contents.
// USEAGE: delete_dir("folder1/will_delete");
pub fn delete_dir(path: &str) -> io::Result<()> {
    let folder_name = get_name(path).unwrap();
    if !is_path_valid(path) {
        eprintln!("error: Directory <{}> has already deleted", folder_name);
        return Err(Error::new(ErrorKind::Unsupported, "Directory does not exist"));
    } else if !is_name_valid(&folder_name) {
        return Err(Error::new(ErrorKind::Unsupported, "Invalid directory name format"));
    }
    fs::remove_dir_all(path)
}

// 5. Copies items and folders within a source path to a destination path
//    * This is a recursive method, i.e. it copies the items within nested folders.
// USEAGE: copy_dir("f1/f2/srcs", "f2/f3/dest");
pub fn copy_dir(src: &str, dest: &str) -> io::Result<()> {
    create_dir(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let raw_path = entry_path.to_str().unwrap();

        let mut new_addr = dest.to_owned();
        let file_name = entry_name.to_str().unwrap();
        new_addr.push('/');
        new_addr.push_str(file_name);
        if entry_path.is_dir() {
            copy_dir(raw_path, &new_addr)?;
        } else {
            copy_file(raw_path, &new_addr)?;
        }
    }
    Ok(())
}

// 6. Delete items selectively in a directory, any items or folders name
//     inside 'ignore' vector will not be deleted.
// USEAGE: clear_dir("folder1/folder2", vec!["hi.txt", "rust.rs", "folder3"]);
pub fn clear_dir(path: &str, ignore: Vec<&str>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let raw_path = entry_path.to_str().unwrap();
        if !ignore.contains(&entry_name.to_str().unwrap()) {
            if entry_path.is_dir() {
                delete_dir(raw_path)?;
            } else {
                delete_file(raw_path)?;
            }
        }
    }
    Ok(())
}

// 7. This function will create a file if it does not exist,
//     and will truncate it if it does
// USEAGE: create_file("folder1/hello_world.py");
pub fn create_file(path: &str) -> io::Result<()> {
    if is_path_valid(path) {
        eprintln!("error: file <{:?}> has already created!", get_name(path));
    }
    match fs::File::create(path) {
        Ok(_) => {
            return Ok(());
        }, Err(_) => {
            eprintln!("error: failed to create file at {}", path);
            return Err(Error::new(ErrorKind::Other, "create_file(): unknown error"));
        },
    }
}

// 8. Removes a file from the filesystem.
// USEAGE: delete_dir("folder1/hello_world.py");
pub fn delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

// 9. Copies the contents of one file to another.
// USEAGE: copy_file("folder1/hello_world.py", "folder2/hello_world.py");
pub fn copy_file(src: &str, dest: &str) -> io::Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}

// 10. 
// USEAGE: 
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

// 11. 
// USEAGE: 
pub fn read_file_as_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// 13. 
// USEAGE: 
pub fn is_path_valid(path: &str) -> bool {
    if Path::new(path).is_dir() {
        return true
    } else if Path::new(path).is_file() {
        return true
    }
    false
}

// 14. Takes in Revision struct (vc/revision.rs/Rev), copy
//      its contents to the current working directory
/* ===================== EXPERIMENTAL - UNKNOWN BEHAVIOUR ===================== */
pub fn make_wd(rev: &Rev) -> io::Result<()> {
    let wd_path = get_wd_path();
    clear_dir(&wd_path, vec![".git"])?;
    for (path, item) in rev.get_manifest() {
        create_file(path)?;
        let content = item.get_content();
        write_file(&path, &content.unwrap())?;
    }
    Ok(())
}

// 15. Returns a string to the current working directory
// USEAGE: get_wd_path()
pub fn get_wd_path() -> String {
    env::current_dir().unwrap().into_os_string().into_string().unwrap()
}

// 16.
pub fn path_compose(path1: &str, path2: &str) -> String {
    //let path = format!("{}{}", path1, path2);
    //path
    let mut path = PathBuf::new();
    path.push(path1);
    path.push(path2); 
    path.into_os_string().into_string().unwrap()
}

// 17. get the last portion of a path, e.g ".git/a/b/c" => "c"
pub fn get_name(path: &str) -> Option<String> {
    let path = Path::new(path);
    let name = path.file_name();
    let name_to_string = name.unwrap().to_owned().into_string().unwrap();
    return Some(name_to_string)
}

// 18. returns existing std::fs::Metadata struct
pub fn get_metadata(path: &str) -> io::Result<Metadata> {
    let attr = fs::metadata(path)?;
    return Ok(attr)
}

// 19. .git/a/b/c -> .git/a/b
pub fn get_parent_name(path: &str) -> Option<String> {
    let mut path = PathBuf::from(path); 
    path.pop();
    let rt = path.into_os_string().into_string();
    match rt {
        Ok(p) => return Some(p),
        Err(_) => return None,
    }
}

// 20. get user id and user name, as a tuple
pub fn get_user() -> (u32, String) {
    let user_id = get_current_uid();
    let user = get_user_by_uid(user_id);

    let user_name = user.unwrap().name().to_os_string().into_string().unwrap();
    return (user_id, user_name);
}


#[cfg(test)]
mod tests_dsr {
    use crate::dsr::*;

    #[test]
    fn test_13_is_path_valid() {
        let abs_path = "/Users/elio/Documents/GitHub/dvcs-wye/src/dsr.rs";
        println!("1: {}", is_path_valid(&abs_path));
        let abs_path = "/Users/elio/Documents/Code/UR453/!.txt";
        println!("2: {}", is_path_valid(&abs_path));
        let abs_path = "/Users/elio/Documents/Code/UR453/?.txt";
        println!("2: {}", is_path_valid(&abs_path));
    }

    #[test]
    fn test_19_get_parent_name() {
        let path = "this/that/those/test/hello_world.txt";
        let parent = get_parent_name(&path);
        println!("Parent Name {:?}", parent);
        let parent = get_parent_name(&parent.unwrap());
        println!("Parent Name {:?}", parent);
    }

    #[test]
    fn test_20_get_user() {
        let user = get_user();
        println!("(id, name): {:?}", user);
    }
}