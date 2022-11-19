pub struct RevDiff {
    // TODO
}

pub fn diff<'a>(rev1_id:Option<&'a str>, rev2_id:&'a str) -> Result<RevDiff, &'a str>{
    unimplemented!(); //TODO
}

pub fn cat<'a>(rev_id:Option<&'a str>, path:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

pub fn add<'a>(path:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

pub fn remove<'a>(path:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

pub fn commit<'a>(message:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

pub fn merge<'a>(rev_id:&'a str) -> Result<&'a str, &'a str>{
    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {
    use super::*;
}