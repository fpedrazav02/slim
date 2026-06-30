#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        let mut buffer = Vec::new();
        let item = String::from("Hellow World\r\n");
        buffer.push(item);
        Self { lines: buffer }
    }
}
