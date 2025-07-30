

pub struct LeftSession{
    sessions:Vec<String>,
}

impl LeftSession{
    pub fn new()->Self{
        Self{
            sessions: Vec::new(),
        }
    }
}