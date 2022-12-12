use std::collections::{VecDeque, HashMap, HashSet};

use crate::ui::{Errors, Errors::*};
use crate::vc::{repository::{Repo}, revision::{Rev}};
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

// 1. file_diff
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
    fn force_end_with_newline(content: String) -> String {
        if content.ends_with("\n"){
            content
        } else {
            content + "\n"
        }
    }
    pub fn new(diff1: FileDiff, diff2: FileDiff) -> Self {
        
        let origin = Self::force_end_with_newline(diff1.clone().origin_content);
        let mod1 = Self::force_end_with_newline(diff1.clone().mod_content);
        let mod2 = Self::force_end_with_newline(diff2.clone().mod_content);

        let merge_res = merge(&origin,&mod1,&mod2);
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

// 2. conflict_find
pub fn conflict_find(diff1: FileDiff, diff2: FileDiff) -> Result<FileConflict, Errors> {
    if diff1.origin_content != diff2.origin_content { // not same origin
        return Err(Errstatic("conflict_find: diff1 and diff2 have different original files"));
    }
    Ok(FileConflict::new(diff1, diff2))
}

// 3. find_unmerged
// Uses diffy crate to find if there is a conflict in the file
pub fn find_unmerged<'a>(content: String) -> Result<(), String> {
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

// ---------- PRIVATE FOR LCA ----------
fn get_all_rev_ancestors<'a>(repo: &'a Repo, rev: Rev) -> Result<Vec<Rev>, Errors> {
    let mut revs:Vec<Rev> = Vec::new();

    let mut q = VecDeque::new();
    revs.push(rev.clone());
    q.push_back(rev);

    while !q.is_empty() {
        let cur_rev = q.pop_front().unwrap();
        let parents = vec![cur_rev.get_parent_id(), cur_rev.get_parent_id2()];
        for parent in parents {
            if parent.is_none() { continue; }
            let p = parent.unwrap().to_string();
            let parent_rev = repo.get_rev(p.as_str())?; // NOTE: getting parent_rev from repo will return erorr if it fails

            revs.push(parent_rev.clone());
            q.push_back(parent_rev);
        }
    }
    return Ok(revs);
}

// count the number of indegrees of each revision
fn count_indegrees<'a>(revs_anc:Vec<Rev>) -> Result<HashMap<String, i32>, Errors> {

    let mut indegrees:HashMap<String, i32> = HashMap::new();

    for rev in revs_anc.clone() {
        indegrees.insert(rev.get_id().unwrap().to_string(), 0);
    }
    
    for rev in revs_anc {
        let p1 = rev.get_parent_id();
        if p1.is_some() {
            let p1 = p1.unwrap().to_string();
            indegrees.entry(p1).and_modify(|e| *e += 1).or_insert(1);
        }

        let p2 = rev.get_parent_id2();
        if p2.is_some() {
            let p2 = p2.unwrap().to_string();
            indegrees.entry(p2).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    return Ok(indegrees);
}

// Topological sort of revisions
fn get_rev_topo(repo: &Repo, rev: Rev) -> Result<Vec<String>, Errors> {
    let mut ordering = Vec::new();

    let revs_anc = get_all_rev_ancestors(repo, rev.clone())?;

    // indegrees
    let mut indegrees = count_indegrees( revs_anc)?;

    // queue with 0 indegrees (no parents, so should be just the first rev)
    let mut queue = VecDeque::new();
    for (rev_id, degree) in indegrees.clone() {
        if degree == 0 {
            queue.push_back(rev_id);
        }
    }

    while let Some(rev_id) = queue.pop_front() {
        let rev = repo.get_rev(rev_id.as_str())?;
        ordering.push(rev_id);

        let parents = vec![rev.get_parent_id(), rev.get_parent_id2()];
        for parent in parents {
            if parent.is_none() { continue; }
            let p = parent.unwrap().to_string();
            indegrees.entry(p.clone()).and_modify(|e| *e -= 1);

            if indegrees.get(p.as_str()).unwrap() == &0 {
                queue.push_back(p);
            }
        }
    }

    Ok(ordering)
    
}

// ---------- END PRIVATE FOR LCA ----------

// 4. find_rev_lca
// LCA of two nodes in a DAG
pub fn find_rev_lca<'a>(repo: &'a Repo, rev1: Rev, rev2: Rev) -> Result<Rev, Errors> {
    // Creates 2 topo sortings of the DAGs that are somewhat connected
    // Then, find the first common node in the 2 topo sortings
    // Ex:
    //      0
    //     1 2      (0 -> 1, 0 -> 2)
    //    3  (4)    (1 -> 3, 2 -> 4)
    //      5  [8]  (3 -> 5, 4->5, 4 -> 8)
    //   [6] 7      (5 -> 6, 5 -> 7)
    // between 6 and 8
    // topo6 = [6, 5, 3, 4, 1, 2, 0]
    // topo8 = [8, 4, 2, 0]
    // LCA = 4

    let topo_rev1 = get_rev_topo(repo, rev1)?;
    let rev1_hashset:HashSet<String> = topo_rev1.iter().cloned().collect();
    
    let topo_rev2 = get_rev_topo(repo, rev2)?;
    
    for rev_id in topo_rev2 {
        if rev1_hashset.contains(rev_id.as_str()) {
            return repo.get_rev(rev_id.as_str());
        }
    }
    return Err(Errstatic("No LCA found"));

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_help::*, vc::repository};
    
    #[test]
    fn test_file_diff_1() {
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
    fn test_conflict_find_2() {
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
    fn test_find_unmerged_3() {
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

    #[test]
    fn test_find_rev_lca_4() {
        let cwd = "./a_test_repo/";

        remove_git_and_init(cwd);

        let rev1_id = create_files_and_commit_ab1(cwd);
        let rev2_id = write_create_files_and_commit_abc2(cwd);

        let repo = repository::load(cwd).unwrap();
        let rev1 = repo.get_rev(rev1_id.as_str()).unwrap();
        let rev2 = repo.get_rev(rev2_id.as_str()).unwrap();

        let lca = find_rev_lca(&repo, rev1, rev2).unwrap();
        assert_eq!(lca.get_id(), Some(rev1_id.as_str())); // lca is rev1
    }
}