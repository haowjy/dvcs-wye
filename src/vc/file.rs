// #[derive(Debug, Serialize, Deserialize)] 
// pub struct FileInfo {
//     loc_in_wd: String, // wd relative path
//     content_id: String, // sha_id
//     metadata: FileMetaData,
// }

// #[derive(Debug, Serialize, Deserialize)] 
// struct FileMetaData {

// }

// impl FileInfo {
//     pub fn get_content(&self) -> io::Result() {
//         let WD = get_wd_path();
//         let paths = RepoPaths(WD);
//         let path_to_file = path_compose(repo_path, self.content_id);
//         read_file_as_string(&path_to_file)
//     }

//     pub (crate) fn make_file(&self, &wd:&str) -> io::Result() {
//         let path_to_file = path_compose(wd, self.loc_in_wd);
//         copy_file()
//         write_file(path_to_file, )
//     }
// }