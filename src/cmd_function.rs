#![allow(unused_variables)]
#![allow(dead_code)]

use diffy::{create_patch, Patch, merge};

#[derive(PartialEq,Debug,Clone)]
pub struct FileDiff<'a> {
    origin_content:&'a str,
    mod_content:&'a str,
    patch: Patch<'a, str>,
    is_diff: bool,
}

impl <'a> FileDiff<'a> {
    pub fn new(origin_content:&'a str, mod_content:&'a str) -> Self {
        let patch = create_patch(origin_content, mod_content);
        let is_diff = origin_content == mod_content;
        Self {
            origin_content,
            mod_content,
            patch,
            is_diff,
        }
    }

    pub fn get_patch(&self) -> &Patch<'a, str> {
        &self.patch
    }

    pub fn is_diff(&self) -> bool {
        self.is_diff
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_diff() {
        let diff1 = file_diff("Some line of text\nSecond line", "Some line of text\nSecond line\nin a file");
        assert_eq!(diff1.is_diff, true); // diff's is_diff is true
        let diff2 = file_diff("Some line of text\nSecond line", "Some line of text\nSecond line");
        assert_eq!(diff2.is_diff, false); //diff;s is_diff is false


        let diff = file_diff("Some line of text\nSecond line", "Some line of text\nSecond line\n");
        let test_patch = diffy::create_patch("Some line of text\nSecond line", "Some line of text\nSecond line\n");
        assert_eq!(diff, FileDiff{
            origin_content: "Some line of text\nSecond line",
            mod_content: "Some line of text\nSecond line\n",
            patch: test_patch,
            is_diff: true
        }); // diff returns error
    }

    #[test]
    fn test_conflict_find() {
        let content0 = "Some line of text\nSecondLine\n";
        let content1 = "Some line of text\nInsert\nSecondLine\n";
        let content2 = "Some line of text\nSecondLine\nInsertLast";

        let fd01 = file_diff(content0, content1);
        let fd02 = file_diff(content0, content2);

        let conflict0_12 = conflict_find(&fd01, &fd02);

        assert_eq!(conflict_find(&fd01, &fd02).unwrap().is_conflict, false); // not a conflict

        let fd12 = file_diff(content1, content2);
        let conflict01_12 = conflict_find(
            &fd01,
            &fd12
        );
        assert_eq!(conflict01_12, Err("conflict_find: diff1 and diff2 have different original files")); // different origins

        let content3 = "Some line of \nSecondLine\nInsertLast";
        let fd03 = file_diff(content0, content3);
        assert_eq!(conflict_find(&fd01, &fd03).unwrap().is_conflict, true); // is a conflict

    }
}