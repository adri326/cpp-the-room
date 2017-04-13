use std::io::{self, Write};
pub mod objects;
pub mod story;

pub struct Room {
    name: String,
    desc: String,
    objects: Vec<Object>
}

impl Room {
    fn new(name: &str) -> Room {
        Room {
            name: String::from(name),
            desc: String::new(),
            objects: Vec::new()
        }
    }

    fn set_desc(&mut self, desc: &str) {
        self.desc = String::from(desc);
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn long_desc(&self) {
        println!("{} {}", self.name, self.desc);
        println!("Objects:");
        for x in &self.objects {
            print!(" - ");
            x.short_desc();
        }
    }
}

pub struct Object {
    name: String,
    desc: String,
    status: String,
    weight: i16,
    show_status: bool,
    touch: Vec<fn(&mut Room)>
}

impl Object {
    fn new(name: &str, weight: i16, touch: fn(&mut Room)) -> Object {
        Object {
            name: String::from(name),
            desc: String::new(),
            status: String::new(),
            weight: weight,
            show_status: false,
            touch: vec!(touch)
        }
    }
    fn set_desc(&mut self, desc: &str) {
        self.desc = String::from(desc);
    }
    fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
        self.show_status = true;
    }
    fn short_desc(&self) {
        if self.show_status {
            println!("{} ({})", self.name, self.status);
        }
        else {
            println!("{}", self.name);
        }
    }
    fn long_desc(&self) {
        if self.show_status {
            println!("{} ({}): {};  Weight: {}", self.name, self.status, self.desc, self.weight);
        }
        else {
            println!("{}: {};  Weight: {}", self.name, self.desc, self.weight);
        }
    }
}

pub fn clear_screen() -> bool {
    std::process::Command::new("cls").status().or_else(|_| std::process::Command::new("clear").status()).unwrap().success()
}



fn main() {
    let mut rooms = Vec::<Room>::new();
    unsafe { init(&mut rooms); }
    clear_screen();
    story::main(&mut rooms);
}

unsafe fn init(rooms: &mut Vec<Room>) {
    rooms.push(Room::new("A dark room..."));
    rooms[0].set_desc("The lights are turned off and you feel the presence of objects around you.");
    rooms[0].objects.push(Object::new("Door", 1000, objects::door_locked_touch));
    rooms[0].objects[0].set_desc("An iron door");
    rooms[0].objects[0].set_status("Locked");
    rooms[0].objects.push(Object::new("Switch", 2, objects::room_1_switch_touch));
    rooms[0].objects[1].set_desc("A switch, perhaps there to activate the lights");
    rooms[0].objects[1].set_status("Off");
}

pub fn user_input() -> String {
    println!("");
    print!("> ");
    let mut input = String::new();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	println!("");
	input.pop();
	return input;
}
