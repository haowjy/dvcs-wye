pub struct FileDiff {
    // TODO
}

pub fn file_diff(content1:&str, content2:&str) -> FileDiff {
    unimplemented!(); //TODO
}

pub struct Conflict {
    // TODO
}

pub fn conflict_find(diff1:FileDiff, diff2:FileDiff) -> Option<Conflict> {
    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {
    use super::*;
}