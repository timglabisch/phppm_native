pub trait OutputTrait {
    fn is_very_verbose(&self) -> bool;
    fn writeln(&self, &str);
}

pub struct OutputConsole;

impl OutputConsole {
    pub fn new() -> OutputConsole {
        OutputConsole {}
    }
}

impl OutputTrait for OutputConsole {
    fn is_very_verbose(&self) -> bool {
        true
    }

    fn writeln(&self, v : &str) {
        println!("{}", v);
    }
}
