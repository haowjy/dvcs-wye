#![allow(unused_variables)]
#![allow(dead_code)]
//TODO: remove this
use crate::ui::{Errors, Errors::*};
use diffy::{create_patch, merge};

#[derive(PartialEq,Debug,Clone)]
pub enum FileDiffType {
    Added,
    Deleted,
    Modified,
    Unchanged,
}

#[derive(PartialEq,Debug,Clone)]
pub struct FileDiff {
    origin_content:String,
    mod_content:String,
    patch: String,
    diff_type: FileDiffType,
}

impl FileDiff {
    pub fn new(origin_content: String, mod_content: String) -> Self {
        let diff_type = if origin_content.is_empty() {
            FileDiffType::Added
        } else if mod_content.is_empty() {
            FileDiffType::Deleted
        } else if origin_content.clone() != mod_content.clone() {
            FileDiffType::Modified
        } else {
            FileDiffType::Unchanged
        };


        let patch = create_patch(&origin_content, &mod_content).to_string();
        
        return Self {
            origin_content,
            mod_content,
            patch,
            diff_type,
        }
    }

    pub fn get_original(&self) -> String {
        self.origin_content.clone()
    }

    pub fn get_modified(&self) -> String {
        self.mod_content.clone()
    }

    pub fn get_patch(&self) -> String {
        self.patch.clone()
    }

    pub fn to_string(&self) -> String {
        self.patch.to_string()
    }

    pub fn get_diff_type(&self) -> &FileDiffType {
        &self.diff_type
    }

    pub fn is_diff(&self) -> bool {
        &self.diff_type != &FileDiffType::Unchanged
    }
}

// result: ok -> no diff, err -> diff
pub fn file_diff(origin_content: String, mod_content: String) -> FileDiff {
    FileDiff::new(origin_content, mod_content)
}

#[derive(PartialEq,Debug,Clone)]
pub struct FileConflict {
    origin_content: String,
    diff1: FileDiff,
    diff2: FileDiff,
    pub merged_content: String,
    pub is_conflict: bool,
}

impl FileConflict {
    pub fn new(diff1: FileDiff, diff2: FileDiff) -> Self {

        let merge_res = merge(&diff1.origin_content,& diff1.mod_content,& diff2.mod_content);
        let is_conflict = merge_res.is_err();
        
        let merged_content = match merge_res {
            Ok(merged_content) => merged_content,
            Err(merged_content) => merged_content,
        };

        Self {
            origin_content: diff1.clone().origin_content,
            diff1,
            diff2,
            merged_content: merged_content,
            is_conflict,
        }
    }

    pub fn get_content(&self) -> String {
        self.merged_content.clone()
    }

    pub fn is_conflict(&self) -> bool {
        self.is_conflict
    }

}

// results: Ok is running fine -> error -> conflict, ok -> no conflict
// first error is failed to run conflict_find 
pub fn conflict_find(diff1: FileDiff, diff2: FileDiff) -> Result<FileConflict, Errors> {
    if diff1.origin_content != diff2.origin_content { // not same origin
        return Err(Errstatic("conflict_find: diff1 and diff2 have different original files"));
    }
    Ok(FileConflict::new(diff1, diff2))
}

// Uses diffy crate to find if there is a conflict in the file
pub fn find_unmerged<'a>(content: String) -> Result<(), Errors> {
    let mut unmerged_markers = vec!["<<<<<<< ours", "||||||| original", "=======", ">>>>>>> theirs"].into_iter();
    let is_unmerged = content.split("\n").into_iter().try_fold(unmerged_markers.next(), |marker, line| {
        if marker.is_none() {
            return None;
        }
        if line == marker.unwrap() {
            let m = unmerged_markers.next();
            return Some(m);
        }
        Some(marker)
    }).is_none();
    if is_unmerged {
        let i = content.split("\n").into_iter().position(|line| line == "<<<<<<< ours");
        return Err(ErrStr(["File is unmerged at line", &i.unwrap().to_string()].join(" ")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_diff() {
        let diff1 = file_diff("Some line of text\nSecond line".to_string(), "Some line of text\nSecond line\nin a file".to_string());
        assert_eq!(diff1.get_diff_type(), &FileDiffType::Modified); // modified
        let diff2 = file_diff("Some line of text\nSecond line".to_string(), "Some line of text\nSecond line".to_string());
        assert_eq!(diff2.get_diff_type(), &FileDiffType::Unchanged); // no diff, unchanged

        let diff = file_diff("Some line of text\nSecond line".to_string(), "Some line of text\nSecond line\n".to_string());
        let test_patch = diffy::create_patch("Some line of text\nSecond line", "Some line of text\nSecond line\n");
        assert_eq!(diff, FileDiff{
            origin_content: "Some line of text\nSecond line".to_string(),
            mod_content: "Some line of text\nSecond line\n".to_string(),
            patch: test_patch.to_string(),
            diff_type: FileDiffType::Modified
        }); // diff returns error

        let diff_no_origin = file_diff("".to_string(),"Some line of text\nSecond line\n".to_string());
        assert_eq!(diff_no_origin.get_diff_type(), &FileDiffType::Added); // added
        let diff_no_mod = file_diff("Some line of text\nSecond line".to_string(), "".to_string());
        assert_eq!(diff_no_mod.get_diff_type(), &FileDiffType::Deleted); // deleted
    }

    #[test]
    fn test_conflict_find() {
        let content0 = "Some line of text\nSecondLine\n".to_string();
        let content1 = "Some line of text\nInsert\nSecondLine\n".to_string();
        let content2 = "Some line of text\nSecondLine\nInsertLast".to_string();

        let fd01 = file_diff(content0.clone(), content1.clone());
        let fd02 = file_diff(content0.clone(), content2.clone());

        let conflict0_12 = conflict_find(
            fd01.clone(), 
            fd02.clone()
        );
        assert_eq!(conflict0_12.unwrap().is_conflict, false); // not a conflict

        let fd12 = file_diff(content1.clone(), content2.clone());
        let conflict01_12 = conflict_find(
            fd01.clone(),
            fd12.clone()
        );
        assert!(conflict01_12.is_err()); // different origins

        let content3 = "Some line of \nSecondLine\nInsertLast".to_string();
        let fd03 = file_diff(content0.clone(), content3.clone());

        let conflict0_13 = conflict_find(
            fd01.clone(),
            fd03.clone()
        );
        assert_eq!(conflict0_13.unwrap().is_conflict, true); // is a conflict
    }

    #[test]
    fn test_find_unmerged() {
        let content0 = "Some line of text\nSecondLine\n".to_string();
        let content1 = "Some line of text\nInsert\nSecondLine\n".to_string();
        let content2 = "Some line of text\nSecondLine\nInsertLast".to_string();
        let content3 = "Some line of \nSecondLine\nInsertLast".to_string();

        let fd01 = file_diff(content0.clone(), content1.clone());
        let fd02 = file_diff(content0.clone(), content2.clone());
        let conflict_0_12 = conflict_find(
            fd01.clone(),
            fd02.clone()
        ).unwrap();
        assert_eq!(conflict_0_12.is_conflict, false); // not a conflict
        assert!(find_unmerged(conflict_0_12.get_content()).is_ok()); // no unmerged markers
        
        let fd03 = file_diff(content0.clone(), content3.clone());
        let conflict0_13 = conflict_find(
            fd01.clone(),
            fd03.clone()
        ).unwrap();
        assert_eq!(conflict0_13.is_conflict, true); // is a conflict
        
        assert!(find_unmerged(conflict0_13.get_content()).is_err()); // has unmerged markers

        let conf = "<<<<<<< ours\n\
        Some line of text\n\
        Insert\n\
        ||||||| original\n\
        Some line of text\n\
        =======\n\
        Some line of\n\
        SecondLine\n\
        InsertLast".to_string();

        // must EXACTLY match the merge markers:
        // <<< ours
        // ...
        // |||origin 
        // ...
        //=== 
        // ...
        //>>> their
        assert!(find_unmerged(conf).is_ok()); // has unmerged markers
    }
}