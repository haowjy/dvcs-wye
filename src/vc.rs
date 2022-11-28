pub mod repository;
pub mod revision;
pub mod file;

#[cfg(test)]
mod tests {
    // use::super::*;
    use crate::vc::repository::*;
    use crate::vc::revision::*;
    use crate::vc::file::*;
    use crate::dsr::*;

    // VC_test_1
    #[test]
    fn init_load_test() {
        init().unwrap();
        let wd = get_wd_path();
        
        let mut repo = load(&wd).unwrap();
        let path = "src/vc.rs";
        let abs_path = get_abs_path(&path).unwrap();
        repo.add_file(&abs_path).unwrap(); //ok as long as the path contains the wd root
        repo.commit().unwrap(); // problematic, returns None

        delete_dir("./.dvcs");
    }


    // let rev = repo.get_current_head().unwrap();
    // assert_eq!(rev.get_parent_id(), None);

    // // VC_test_2
    // #[test]
    // fn trace_test() {
    //     let rev1 = Revision::new();
    //     let rev2 = Revision::from(rev1.id());
    //     assert_eq!(rev2.parent_id(), rev1.id())
    // }


    // // VC_test_3
    // #[test]
    // fn load_test() {
    //     let mut repo = Repository::new();
    //     let rev1 = Revision::new();
    //     repo.commit(&rev1);
    //     //with private fn: repo.save() equivalent

    //     let repo2 = Repository::load("WD/of/this/repo");
    //     assert_eq!(repo2.get_current_head(), rev1.get_id());
    // }

    // // VC_test_4
    // #[test]
    // fn retrival_test() {
    //     let mut repo = Repository::new();
    //     let rev1 = Revision::new();
    //     repo.commit(&rev1);
    //     let rev2 = repo.get_rev(rev1.get_id()).unwrap();
    //     assert_eq!(rev2, rev1);
    // }

    // // VC_test_5
    // #[test]
    // fn log_test() {
    //     let mut repo = Repository::new();
    //     let mut stage = Revision::new();
    //     stage.add_file("some_file.txt");
    //     let rev1 = repo.commit(&rev1).unwrap();
    //     let rev2 = Revision::from(rev1.get_id());

    //     rev2.add_file("some_other_file.txt");
    //     repo.commit(&rev2)
    //     assert_eq!(repo.get_log().unwrap().len(), 2);
    // }

    // // VC_test_6
    // #[test]
    // fn file_test() {
    //     let mut stage = Revision::new();
    //     stage.add_file("some_file.txt");
    //     stage.add_file("some_other_file.txt");
    //     stage.remove_file("some_file.txt");
    //     let files = stage.get_files();
    //     let file = files["some_file.txt"];
    //     let content = file.get_content();
    //     let path = file.get_WD_path();
    //     assert_eq(content, ""); // assuming the added file is empty
    //     assert_eq!(path, ""); // assuming the file is at base 
    //     assert_eq!(files.len(), 1)    
    // }

}

