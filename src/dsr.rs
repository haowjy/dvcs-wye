use std::io::{Error, ErrorKind};
use std::{fs, io, env};
use std::path::{Path, PathBuf};

// ==================================
//        PRIVATE FUNCTIONS
// ==================================

// check if the file/dir name contains forbidden character(s)
fn is_name_valid(name: &str) -> bool {
    let os = env::consts::OS;
    match os {
        "linux" => {
            if name.contains('/') {
                eprintln!("ERR<DSR-02>: Name({}) contains forbidden ASCII character(s)", name);
                return false
            }
        }, "macos" => {
            if name.contains(':') {
                eprintln!("ERR<DSR-03>: Name({}) contains forbidden ASCII character(s)", name);
                return false
            }
        }, "windows" => {
            if name.contains('<') || name.contains('>') || name.contains(':') ||
               name.contains('"') || name.contains('/') || name.contains('\\') ||
               name.contains('|') || name.contains('?') || name.contains('*') {
                eprintln!("ERR<DSR-04>: Name({}) contains forbidden ASCII character(s)", name);
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

// 1. Serialize

// 2. Deserialize

// 3. Create a directory and directory within recursively if missing
//     If you would like to create a hidden folder, add a . in front
//      of the folder name, i.e. "folder1/folder2/.git"
// USEAGE: create_dir("folder1/folder2/folder3/.hidden_folder");
pub fn create_dir(path: &str) -> io::Result<()> {
    let folder_name = get_name(path).unwrap();
    if is_path_valid(path) {
        eprintln!("WARN<DSR-02>: Directory <{}> has already created", folder_name);
    } else if !is_name_valid(&folder_name) {
        return Err(Error::new(ErrorKind::Unsupported, "ERR: Invalid directory name format"));
    }
    fs::create_dir_all(path)
}

// 4. Remove a directory at this path, after removing all its contents.
// USEAGE: delete_dir("folder1/will_delete");
pub fn delete_dir(path: &str) -> io::Result<()> {
    let folder_name = get_name(path).unwrap();
    if !is_path_valid(path) {
        eprintln!("ERR<DSR-05>: Directory <{}> has already deleted", folder_name);
        return Err(Error::new(ErrorKind::Unsupported, "ERR: Directory does not exists"));
    } else if !is_name_valid(&folder_name) {
        return Err(Error::new(ErrorKind::Unsupported, "ERR: Invalid directory name format"));
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
    fs::File::create(path)?;
    Ok(())
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
/* ===================== PSEUDOCODE =====================
pub fn make_wd(rev: &Rev) -> io::Result<()> {
    clear_dir(wd)
    for each F in Rev.manifest {
        path = F.key
        info = F.value

        create_file(path)
        content = info.get_content() from vc/file.rs
        write_file(&path, &content)
    }
}
======================== PSEUDOCODE ===================== */

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
    let mut pieces = path.rsplit('/');
    match pieces.next() {
        Some(p) => {
            let rt: String = p.to_string();
            return Some(rt)
        } None => {
            eprintln!("ERR<DSR-01>: Invalid Path");
            return None
        },
    }
}