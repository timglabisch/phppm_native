#[derive(Debug)]
pub struct Config {
    pub working_directory : String,
    pub host : String,
    pub port : usize,
    pub workers : usize
}
