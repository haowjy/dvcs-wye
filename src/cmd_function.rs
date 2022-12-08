#![allow(unused_variables)]
#![allow(dead_code)]

use diffy::{create_patch, Patch, merge};

#[derive(PartialEq,Debug,Clone)]
pub enum FileDiffType {
    Added,
    Deleted,
    Modified,
    Unchanged,
}

#[derive(PartialEq,Debug,Clone)]
pub struct FileDiff<'a> {
    origin_content:&'a str,
    mod_content:&'a str,
    patch: Patch<'a, str>,
    diff_type: FileDiffType,
}

impl <'a> FileDiff<'a> {
    pub fn new(origin_content: &'a str, mod_content:&'a str) -> Self {
        let diff_type = if origin_content.is_empty() {
            FileDiffType::Added
        } else if mod_content.is_empty() {
            FileDiffType::Deleted
        } else if origin_content != mod_content {
            FileDiffType::Modified
        } else {
            FileDiffType::Unchanged
        };

        let patch = create_patch(origin_content, mod_content);
        
        return Self {
            origin_content: origin_content,
            mod_content: mod_content,
            patch: patch,
            diff_type: diff_type,
        }
    }

    pub fn get_original(&self) -> &'a str {
        self.origin_content
    }

    pub fn get_modified(&self) -> &'a str {
        self.mod_content
    }

    pub fn get_patch(&self) -> &Patch<'a, str> {
        &self.patch
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
pub fn file_diff<'a>(origin_content:&'a str, mod_content:&'a str) -> FileDiff<'a> {
    FileDiff::new(origin_content, mod_content)
}

#[derive(PartialEq,Debug,Clone)]
pub struct FileConflict<'a> {
    origin_content:&'a str,
    diff1: &'a FileDiff<'a>,
    diff2: &'a FileDiff<'a>,
    pub merged_content: String,
    pub is_conflict: bool,
}

impl <'a> FileConflict<'a> {
    pub fn new(diff1: &'a FileDiff<'a>, diff2: &'a FileDiff<'a>) -> Self {

        let merge_res = merge(diff1.origin_content, diff1.mod_content, diff2.mod_content);
        let is_conflict = merge_res.is_err();
        
        let merged_content = match merge_res {
            Ok(merged_content) => merged_content,
            Err(merged_content) => merged_content,
        };

        Self {
            origin_content: diff1.origin_content,
            diff1,
            diff2,
            merged_content: merged_content,
            is_conflict,
        }
    }

    pub fn get_content(&self) -> &str {
        self.merged_content.as_str()
    }

    pub fn is_conflict(&self) -> bool {
        self.is_conflict
    }

}

// results: Ok is running fine -> error -> conflict, ok -> no conflict
// first error is failed to run conflict_find 
pub fn conflict_find<'a>(diff1:&'a FileDiff, diff2:&'a FileDiff) -> Result<FileConflict<'a>, &'a str> {
    if diff1.origin_content != diff2.origin_content { // not same origin
        return Err("conflict_find: diff1 and diff2 have different original files");
    }
    Ok(FileConflict::new(diff1, diff2))
}

// Uses diffy crate to find if there is a conflict in the file
pub fn find_unmerged<'a>(content: &'a str) -> Result<(), String> {
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
        return Err(["File is unmerged at line", &i.unwrap().to_string()].join(" "));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_diff() {
        let diff1 = file_diff("Some line of text\nSecond line", "Some line of text\nSecond line\nin a file");
        assert_eq!(diff1.get_diff_type(), &FileDiffType::Modified); // modified
        let diff2 = file_diff("Some line of text\nSecond line", "Some line of text\nSecond line");
        assert_eq!(diff2.get_diff_type(), &FileDiffType::Unchanged); // no diff, unchanged

        let diff = file_diff("Some line of text\nSecond line", "Some line of text\nSecond line\n");
        let test_patch = diffy::create_patch("Some line of text\nSecond line", "Some line of text\nSecond line\n");
        assert_eq!(diff, FileDiff{
            origin_content: "Some line of text\nSecond line",
            mod_content: "Some line of text\nSecond line\n",
            patch: test_patch,
            diff_type: FileDiffType::Modified
        }); // diff returns error

        let diff_no_origin = file_diff("", "Some line of text\nSecond line\n");
        assert_eq!(diff_no_origin.get_diff_type(), &FileDiffType::Added); // added
        let diff_no_mod = file_diff("Some line of text\nSecond line", "");
        assert_eq!(diff_no_mod.get_diff_type(), &FileDiffType::Deleted); // deleted
    }

    #[test]
    fn test_conflict_find() {
        let content0 = "Some line of text\nSecondLine\n";
        let content1 = "Some line of text\nInsert\nSecondLine\n";
        let content2 = "Some line of text\nSecondLine\nInsertLast";

        let fd01 = file_diff(content0, content1);
        let fd02 = file_diff(content0, content2);

        let conflict0_12 = conflict_find(
            &fd01, 
            &fd02
        );
        assert_eq!(conflict0_12.unwrap().is_conflict, false); // not a conflict

        let fd12 = file_diff(content1, content2);
        let conflict01_12 = conflict_find(
            &fd01,
            &fd12
        );
        assert_eq!(conflict01_12, Err("conflict_find: diff1 and diff2 have different original files")); // different origins

        let content3 = "Some line of \nSecondLine\nInsertLast";
        let fd03 = file_diff(content0, content3);

        let conflict0_13 = conflict_find(
            &fd01,
            &fd03
        );
        assert_eq!(conflict0_13.unwrap().is_conflict, true); // is a conflict
    }

    #[test]
    fn test_patch_find() {
        let content0 = "Some line of text\nSecondLine\n";
        let content1 = "Some line of text\nInsert\nSecondLine\n";
        let content2 = "Some line of text\nSecondLine\nInsertLast";
        let content3 = "Some line of \nSecondLine\nInsertLast";

        let fd01 = file_diff(content0, content1);
        let fd02 = file_diff(content0, content2);
        let conflict_0_12 = conflict_find(
            &fd01,
            &fd02
        ).unwrap();
        assert_eq!(conflict_0_12.is_conflict, false); // not a conflict
        assert_eq!(find_unmerged(conflict_0_12.get_content()), Ok(())); // no unmerged markers
        
        let fd03 = file_diff(content0, content3);
        let conflict0_13 = conflict_find(
            &fd01,
            &fd03
        ).unwrap();
        assert_eq!(conflict0_13.is_conflict, true); // is a conflict
        
        assert_eq!(find_unmerged(conflict0_13.get_content()), Err(String::from("File is unmerged at line 0"))); // has unmerged markers

        let conf = "<<<<<<< ours\n\
        Some line of text\n\
        Insert\n\
        ||||||| original\n\
        Some line of text\n\
        =======\n\
        Some line of\n\
        SecondLine\n\
        InsertLast";

        // must EXACTLY match the merge markers:
        // <<< ours
        // ...
        // |||origin 
        // ...
        //=== 
        // ...
        //>>> their
        assert_eq!(find_unmerged(conf), Ok(())); // has unmerged markers
    }
}