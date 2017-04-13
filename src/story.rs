use super::*;

pub fn main(rooms: &mut Vec<Room>) {
    let active_room = 0;
    let active_object: i16 = 0;
    rooms[active_room].long_desc();
    loop {
        let cmd = user_input();
        treat(cmd, &mut rooms[active_room], active_object);
    }
}

pub fn treat(cmd: String, mut room: &mut Room, active_object: i16) {
    let words: Vec<&str> = cmd.split(" ").collect();
    match words[0].as_ref() {
        "help" => {
            if words.len() > 1 {
                match words[1].as_ref() {
                    "help" => {
                        println!("help [command]: display a help message")
                    }
                    _ => {
                        println!("There is no help for what you asked for")
                    }
                }
            }
            else {
                println!("Available commands:");
                println!(" - help");
                println!(" - poke");
                println!(" - desc");
                println!(" - touch");
            }
        }
        "poke" => {

        }
        "desc" => {
            if words.len() > 1 {
                for x in &mut room.objects {
                    if x.name == words[1].as_ref() {
                        x.long_desc();
                    }
                }
            }
            else {
                room.long_desc();
            }
        }
        "touch" => {
            if words.len() > 1 {
                let mut n = 0;
                let mut o = 100;
                for x in &mut room.objects {
                    if x.name == words[1].as_ref() {
                        o = n;
                    }
                    n += 1;
                }
                if o<100 {
                    (room.objects[o].touch[0])(room);
                }
            }
            else {
                println!("You need something to touch!");
            }
        }
        _ => {}
    }
}
