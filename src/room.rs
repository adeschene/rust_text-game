
//use

pub struct Room {
    pub name: String,
    pub desc: String,
    //inv
}

impl Room {
    pub fn new(name: String, desc: String) -> Room {
        Room { name, desc }
    }
}

pub fn make_room(code: &str) -> Room {
    match code {
        "R00" => Room::new( "Cell".to_string(),
                            "A dank cell.".to_string() ),
        _     => Room::new( "err_room".to_string(),
                            "error!".to_string() )
    }
}



