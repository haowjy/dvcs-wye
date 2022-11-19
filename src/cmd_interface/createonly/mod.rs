pub fn clone<'a>(wd:&'a str, remote:&'a str) -> Result<&'a str, &'a str> {
    unimplemented!(); //TODO
}

pub fn checkout<'a>(wd:&'a str, rev:&'a str) -> Result<&'a str, &'a str> {
    unimplemented!(); //TODO
}

pub fn new_checkout<'a>(wd:&'a str, rev:&'a str) -> Result<&'a str, &'a str> {
    unimplemented!(); //TODO
}

pub fn pull<'a>(wd:&'a str, remote:&'a str, head:Option<&'a str>) -> Result<&'a str, &'a str> {
    unimplemented!(); //TODO
}

pub fn push<'a>(wd:&'a str, remote:&'a str, head:Option<&'a str>) -> Result<&'a str, &'a str> {
    unimplemented!(); //TODO
}

#[cfg(test)]
mod tests {
    use super::*;
}
