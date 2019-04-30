pub struct Command {
    pub cmd_type: String,
    pub cmd_content: String
}

impl Command {
    fn set_type(&mut self, x: String) { 
        self.cmd_type = x; 
    }

    fn set_content(&mut self, x: String) { 
        self.cmd_content = x; 
    }

    fn default() -> Command {
        Command { cmd_type: String::from(""), cmd_content: String::from("") }
    }
}

pub fn cmd(comm: &str) -> Command {
    let mut order: Command = Command::default();
    let v: Vec<&str> = comm.split("{split}").collect();

    order.set_type(v[0].to_string());
    order.set_content(v[1].to_string());

    order
}