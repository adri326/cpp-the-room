#![allow(unused_variables)]

use super::*;

pub fn nothing_touch(room: &mut Room) {
    println!("Nothing happens!");
}

pub fn door_locked_touch(room: &mut Room) {
    println!("This door is locked!")
}

pub fn switch_off_touch(room: &mut Room) {
    println!("You turned the switch on!");
}

pub fn room_1_switch_touch(room: &mut Room) {
    if room.objects[1].status=="Off" {
        println!("As you turn the switch on, the lights turns on!");
        if room.objects.len() < 3 {
            room.objects.push(Object::new("Crate", 32, nothing_touch));
            room.objects[2].set_desc("A simple wood crate. It is locked.");
            println!("You now see new items in the room!");
            room.set_name("A simple, small room.");
        }
        room.set_desc("The lights are turned on.");
    } else {
        println!("The lights turns off!");
        room.set_desc("The lights are turned off.");
    }
}
