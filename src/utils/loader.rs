pub fn open<'a>(path: &'a str) -> std::io::Result<tsplib::Instance> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Cursor;

    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let instance = tsplib::parse(Cursor::new(&data))?;
    Ok(instance)
}
