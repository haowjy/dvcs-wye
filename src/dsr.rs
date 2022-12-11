use glob::glob;
use std::{fs, env};
use std::fs::{Metadata, ReadDir};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use users::{get_user_by_uid, get_current_uid};

use crate::ui::Errors;
use crate::vc::repository;
use crate::vc::revision::Rev;

// ==================================
//        PRIVATE FUNCTIONS
// ==================================

// function to create custom error enums
fn new_error(kind: ErrorKind, message: &str) -> Errors {
    return Errors::ErrIo(Error::new(kind, message));
}

// returns an iterator by reading a directory
fn read_dir(path: &str) -> Result<ReadDir, Errors> {
    match fs::read_dir(path) {
        Ok(it) => return Ok(it),
        Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"read_dir: unknown error when reading directory")),
    }
}

// check if a file is in an directory or a sub directory
fn is_in(path: &str, file: &str) -> bool {
    match glob(&format!("{}/**/{}", path, file)) {
        Ok(p) => {
            for entry in p {
                match entry {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        },
        Err(_) => return false,
    }
    false
}

// check if the file/dir name contains forbidden character(s)
fn is_name_valid(name: &str) -> bool {
    let os = env::consts::OS;
    match os {
        "linux" => {
            if name.contains('/') {
                return false
            }
        }, "macos" => {
            if name.contains(':') {
                return false
            }
        }, "windows" => {
            if name.contains('<') || name.contains('>') || name.contains(':') ||
               name.contains('"') || name.contains('/') || name.contains('\\') ||
               name.contains('|') || name.contains('?') || name.contains('*') {
                return false
            }
        }, _ => {
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
pub fn create_dir(path: &str) -> Result<(), Errors> {
    let folder_name = get_name(path).unwrap();
    if is_path_valid(path) {
        return Err(new_error(ErrorKind::AlreadyExists, &format!("create_dir: directory({}) already exist", folder_name)));
    } else if !is_name_valid(&folder_name) {
        return Err(new_error(ErrorKind::InvalidInput, &format!("create_dir: name({}) contains forbidden character(s)", folder_name)));
    }

    match fs::create_dir_all(path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"create_dir: unknown error when creating directory")),
    }
}

// 4. Remove a directory at this path, after removing all its contents.
// USEAGE: delete_dir("folder1/will_delete");
pub fn delete_dir(path: &str) -> Result<(), Errors> {
    let folder_name = get_name(path).unwrap();
    if !is_path_valid(path) {
        return Err(new_error(ErrorKind::NotFound, &format!("delete_dir: directory({}) does not exist", folder_name)));
    } else if !is_name_valid(&folder_name) {
        return Err(new_error(ErrorKind::InvalidInput, &format!("delete_dir: name({}) contains forbidden character(s)", folder_name)));
    }

    match fs::remove_dir_all(path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"delete_dir: unknown error when deleting directory")),
    }
}

// 5. Copies items and folders within a source path to a destination path
//    * This is a recursive method, i.e. it copies the items within nested folders.
// USEAGE: copy_dir("f1/f2/srcs", "f2/f3/dest");
pub fn copy_dir(src: &str, dest: &str) -> Result<(), Errors> {
    create_dir(dest)?;
    for entry in read_dir(src)? {
        match entry {
            Ok(entry) => {
                let entry = entry;
                let entry_path = entry.path();
                let entry_name = entry.file_name();
                let raw_path = entry_path.to_str().unwrap();
        
                let mut new_addr = dest.to_owned();
                let file_name = entry_name.to_str().unwrap();
                new_addr = path_compose(&new_addr, file_name);
                if entry_path.is_dir() {
                    copy_dir(raw_path, &new_addr)?;
                } else {
                    copy_file(raw_path, &new_addr)?;
                }
            }, Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"copy_dir: unable to read entries from dir")),
        }
    }
    Ok(())
}

// 6. Delete items selectively in a directory, any items or folders name
//     inside 'ignore' vector will not be deleted.
// * This function does not check ignored files in sub-directories, use clear_dir_adv() if needed.
// USEAGE: clear_dir("folder1/folder2", vec!["hi.txt", "rust.rs", "folder3"]);
pub fn clear_dir(path: &str, ignore: Vec<&str>) -> Result<(), Errors> {
    for entry in read_dir(path)? {
        match entry {
            Ok(entry) => {
                let entry = entry;
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
            }, Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"clear_dir: unable to read entries from dir")),
        }
    }
    Ok(())
}

// 6.1 A more advanced clear_dir() which checks ignored files in sub-directories, sub-directories
//      that contains ignored files will not be deleted, SEE test functiontest_6_clear_dir_adv() at the end of the code
pub fn clear_dir_adv(path: &str, ignore: Vec<&str>) -> Result<(), Errors> {
    for entry in read_dir(path)? {
        match entry {
            Ok(entry) => {
                let entry = entry;
                let entry_path = entry.path();
                let entry_name = entry.file_name();
                let raw_path = entry_path.to_str().unwrap();

                if entry_path.is_dir() && !ignore.contains(&entry_name.to_str().unwrap()) {
                    let mut isin = false;
                    for file in &ignore {
                        if is_in(raw_path, file) {
                            isin = true;
                            clear_dir_adv(raw_path, ignore.clone())?;
                            break;
                        }
                    }
                    if isin == false {
                        delete_dir(raw_path)?;
                    }
                } else if entry_path.is_file() {
                    if !ignore.contains(&entry_name.to_str().unwrap()) {
                        delete_file(raw_path)?;
                    }
                }
            }, Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"clear_dir: unable to read entries from dir")),
        }
    }
    Ok(())
}

// 7. This function will create a file if it does not exist,
//     and will truncate it if it does
// USEAGE: create_file("folder1/hello_world.py");
pub fn create_file(path: &str) -> Result<(), Errors> {
    let file_name = get_name(path).unwrap();
    if is_path_valid(path) {
        return Err(new_error(ErrorKind::AlreadyExists, &format!("create_file: file({}) already exist", file_name)));
    } else if !is_name_valid(&file_name) {
        return Err(new_error(ErrorKind::InvalidInput, &format!("create_file: name({}) contains forbidden character(s)", file_name)));
    }

    match fs::File::create(path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"create_file: unknown error when creating file")),
    }
}

// 8. Removes a file from the filesystem.
// USEAGE: delete_dir("folder1/hello_world.py");
pub fn delete_file(path: &str) -> Result<(), Errors> {
    let file_name = get_name(path).unwrap();
    if !is_path_valid(path) {
        return Err(new_error(ErrorKind::NotFound, &format!("delete_file: file({}) does not exist", file_name)));
    } else if !is_name_valid(&file_name) {
        return Err(new_error(ErrorKind::InvalidInput, &format!("delete_file: name({}) contains forbidden character(s)", file_name)));
    }

    match fs::remove_file(path) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"delete_file: unknown error when creating file")),
    }
}

// 9. Copies the contents of one file to another.
// USEAGE: copy_file("folder1/hello_world.py", "folder2/hello_world.py");
pub fn copy_file(src: &str, dest: &str) -> Result<(), Errors> {
    let file_name = get_name(dest).unwrap();
    if src.eq(dest) {
        return Err(new_error(ErrorKind::AlreadyExists, &"copy_file: src & dest are the same"));
    } else if !is_path_valid(src) {
        return Err(new_error(ErrorKind::NotFound, &"copy_file: source file does not exist"));
    } else if !is_name_valid(&file_name) {
        return Err(new_error(ErrorKind::InvalidInput, &format!("copy_file: source file name({}) contains forbidden character(s)", file_name)));
    }

    match fs::copy(src, dest) {
        Ok(_) => Ok(()),
        Err(_) => Err(new_error(ErrorKind::UnexpectedEof, &"copy_file: unknown error when copying file")),
    }
}

// 10. Write content to a file given a path
// USEAGE: write_file("folder1/hello_world.py", "print(\"hello world!\")");
pub fn write_file(path: &str, content: &str) -> Result<(), Errors> {
    let file_name = get_name(path).unwrap();
    /*
    if !is_path_valid(path) {
        return Err(new_error(ErrorKind::NotFound, &&format!("write_file: file({}) does not exist", file_name)));
    } else
    */
    if !is_name_valid(&file_name) {
        return Err(new_error(ErrorKind::InvalidInput, &format!("write_file: name({}) contains forbidden character(s)", file_name)));
    }

    match fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(_) => Err(new_error(ErrorKind::UnexpectedEof, &"write_file: unknown error when writing file")),
    }
}

// 11. Return the content of a file as String
// USEAGE: read_file_as_string("folder1/hello_world.py") -> "print("hello world!")"
pub fn read_file_as_string(path: &str) -> Result<String, Errors> {
    let file_name = get_name(path).unwrap();
    if !is_path_valid(path) {
        return Err(new_error(ErrorKind::NotFound, &&format!("read_file_as_string: file({}) does not exist", file_name)));
    }

    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(_) => Err(new_error(ErrorKind::UnexpectedEof, &"read_file_as_string: unknown error when reading file")),
    }
}

// 13. Return a boolean that whether a path is a directory or a file, or neither
// USEAGE: is_path_valid("folder1/hello_world.py")
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
pub fn make_wd(rev: &Rev, wd_path: &str) -> Result<(), Errors> {
    clear_dir(&wd_path, vec![".dvcs"])?;
    let repo = repository::load(wd_path)?;
    for (path, item) in rev.get_manifest() {
        create_file(path)?;
        let content = repo.get_file_content(item)?;
        write_file(&path, &content)?;
    }
    Ok(())
}

// 15. Returns a string to the current working directory
// USEAGE: get_wd_path()
pub fn get_wd_path() -> String {
    env::current_dir().unwrap().into_os_string().into_string().unwrap()
}

// 16. Concatenate two path into one
// USEAGE: path_compose("folder1", "hi.txt") -> "folder1/hi.txt"
pub fn path_compose(path1: &str, path2: &str) -> String {
    let mut path = PathBuf::new();
    path.push(path1);
    path.push(path2); 
    path.into_os_string().into_string().unwrap()
}

// 17. get the last portion of a path, e.g ".git/a/b/c" => "c"
pub fn get_name(path: &str) -> Option<String> {
    let path = Path::new(path);
    let name = path.file_name();
    match name {
        Some(n) => return Some(n.to_owned().into_string().unwrap()),
        None => return None,
    }
}

// 18. returns existing std::fs::Metadata struct
pub fn get_metadata(path: &str) -> Result<Metadata, Errors> {
    let file_name = get_name(path).unwrap();
    if !is_path_valid(path) {
        return Err(new_error(ErrorKind::NotFound, &&format!("get_metadata: file({}) does not exist", file_name)));
    }

    match fs::metadata(path) {
        Ok(attr) => Ok(attr),
        Err(_) => Err(new_error(ErrorKind::UnexpectedEof, &"get_metadata: unknown error when getting metadata")),
    }
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

// 21.
// USAGE: let mut list_files = vec![];
//        get_files("path1/path2", &mut list_files);
pub fn get_files(path: &str, ignore: Vec<&str>, list: &mut Vec<String>) -> Result<(), Errors> {
    match fs::read_dir(path) {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(entry) => {
                        let entry = entry;
                        let entry_path = entry.path();
                        let entry_name = entry.file_name();
                        let raw_path = entry_path.to_str().unwrap();
                        if !ignore.contains(&entry_name.to_str().unwrap()) {
                            if entry_path.is_dir() {
                                get_files(raw_path, ignore.clone(), list)?;
                            } else {
                                list.push(raw_path.to_string());
                            }
                        }
                    },
                    Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"get_files: unable to read entries from dir")),
                }
            }
        },
        Err(_) => return Err(new_error(ErrorKind::UnexpectedEof, &"get_files: unable to read entries from dir")),
    }
    Ok(())
}

// ====================================================
//                  TESTING FUNCTIONS
//        * all tests perform in dsr_tests folder
// ====================================================

#[cfg(test)]
mod tests_dsr {
    use crate::dsr::*;

    #[allow(unused_must_use)]
    fn setup_test_space() {
        create_dir("dsr_test/folder1/folder2/folder3");
        create_dir("dsr_test/folderA/folderB");

        create_file("dsr_test/README.md");
        create_file("dsr_test/folder1/hi.txt");
        create_file("dsr_test/folder1/also_hi.txt");
        create_file("dsr_test/folder1/first_layer.rs");
        create_file("dsr_test/folder1/folder2/another_hi.txt");
        create_file("dsr_test/folder1/folder2/second_layer.rs");
        create_file("dsr_test/folder1/folder2/folder3/third_layer.rs");
        create_file("dsr_test/folderA/cfile.cpp");
        create_file("dsr_test/folderA/folderB/python.py");
    }

    #[allow(unused_must_use)]
    fn clear_test_space() {
        delete_dir("dsr_test");
    }

    #[test]
    fn test_3_create_dir() {
        // success
        match create_dir("dsr_test/create_dir") {
            Ok(_) => println!("create (dsr_test/create_dir) success"),
            Err(e) => println!("error: {:?}", e),
        }
        // failure: already exist
        match create_dir("dsr_test/create_dir") {
            Ok(_) => println!("warning: suppose to fail!"),
            Err(e) => println!("error: {:?}", e),
        }
        // failure: invalid char
        match create_dir("dsr_test/invalid:folder") {
            Ok(_) => println!("warning: suppose to fail!"),
            Err(e) => println!("error: {:?}", e),
        }
    }

    #[test]
    fn test_4_delete_dir() {
        // success
        match delete_dir("dsr_test/create_dir") {
            Ok(_) => println!("delete (dsr_test/create_dir) success"),
            Err(e) => println!("error: {:?}", e),
        }
        // failure: already deleted
        match delete_dir("dsr_test/create_dir") {
            Ok(_) => println!("warning: suppose to fail!"),
            Err(e) => println!("error: {:?}", e),
        }
        // failure: invalid char
        match delete_dir("dsr_test/invalid:folder") {
            Ok(_) => println!("warning: suppose to fail!"),
            Err(e) => println!("error: {:?}", e),
        }
    }

    #[test]
    fn test_7_create_file() {
        // success
        match create_dir("dsr_test/folder/inner") {
            Ok(_) => println!("create (dsr_test/folder) success"),
            Err(e) => println!("error: {:?}", e),
        }
        // success
        match create_file("dsr_test/folder/hi.txt") {
            Ok(_) => println!("create (dsr_test/folder/hi.txt) success"),
            Err(e) => println!("error: {:?}", e),
        }
        // success
        match create_file("dsr_test/folder/python.py") {
            Ok(_) => println!("create (dsr_test/folder/python.py) success"),
            Err(e) => println!("error: {:?}", e),
        }
        // success
        match create_file("dsr_test/folder/inner/rust.rs") {
            Ok(_) => println!("create (dsr_test/folder/inner/rust.rs) success"),
            Err(e) => println!("error: {:?}", e),
        }
        // failure: already exists
        match create_file("dsr_test/folder/inner/rust.rs") {
            Ok(_) => println!("warning: suppose to fail!"),
            Err(e) => println!("error: {:?}", e),
        }
    }

    #[test]
    fn test_5_copy_dir() {
        // success
        match copy_dir("dsr_test/folder", "dsr_test/folder_dup") {
            Ok(_) => println!("copy (dsr_test/folder) success"),
            Err(e) => println!("error: {:?}", e),
        }
    }

    #[test]
    fn test_6_clear_dir_adv() {
        setup_test_space();

        // success
        match clear_dir_adv("dsr_test", vec!["hi.txt", "another_hi.txt", "folderA"]) {
            Ok(_) => println!("clear (dsr_test without hi.txt, another_hi.txt) success"),
            Err(e) => println!("error: {:?}", e),
        }

        //clear_test_space();
    }


    #[test]
    fn test_13_is_path_valid() {
        setup_test_space();
    }

    #[test]
    fn test_21_get_parent_name() {
        setup_test_space();
 
        let mut list = vec![];
        get_files(".", vec![".git", "target", "test_repo"], &mut list);
        println!("{:?}", list);
        clear_test_space();
    }

    #[test]
    fn test_priv_is_in() {
        setup_test_space();

        assert_eq!(is_in("dsr_test", "README.md"), true);
        assert_eq!(is_in("dsr_test", "second_layer.rs"), true);
        assert_eq!(is_in("dsr_test/folder1/folder2", "hi.txt"), false);
        assert_eq!(is_in("dsr_test/folderA", "python.py"), true);
        assert_eq!(is_in("dsr_test/folderA/folderB", "cfile.cpp"), false);

        clear_test_space();
    }
}