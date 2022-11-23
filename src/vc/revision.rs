

// #[derive(Debug, Clone, Serialize, Deserialize)] 
// pub struct Rev {
//     rev_id: Option<String>,
//     user_id: Option<String>,
//     time_stamp: SystemTime,
//     manifest: HashMap<&str, FileInfo>,  // Hashmap<K: wd_relative_path, V: FileInfo: file content id and metadata id
// }

// pub fn from(path_stage: &str) -> Result<Rev> {
//     let rev = deserialize(read_file(path_stage));
//     match rev {
//         Ok() => Ok(Rev),
//         Err() => Err("error loading the stage") // might need wrapping
//     }
// }
// fn save(&self) -> io::Result() {
//     gen_id
//     write_file(serialize(&self.clone()));
// }



// fn gen_id(&self) -> String {
//     sha(serialize(&self.clone()))
//     pub fn add_file(&mut self, wd_file_path: &str) -> Self {
//         self.manifest
//     }
// }
//     pub fn remove_file(&mut self, wd_file_path:&str) -> Self {

//     }
