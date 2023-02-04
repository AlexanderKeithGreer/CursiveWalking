use cursive::Cursive;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod entity;

#[derive(Clone)]
pub struct CoordItem {
    x: usize,
    y: usize,
    c: char,
}

impl CoordItem {

    pub fn new() -> CoordItem {
        let new_item = CoordItem {x: 0, y: 0, c: ' '};
        return new_item
    }

    pub fn get_x(&self) -> usize {
        return self.x
    }

    pub fn get_y(&self) -> usize {
        return self.y
    }

    pub fn get_c(&self) -> char {
        return self.c
    }
}

fn send_description(tx_dialogue: &mpsc::Sender<String>,
                    check_entity: &entity::EntityBase,
                    x:u64, y:u64) -> bool {
    let mut set_to_abort: bool = false;
    if check_entity.is_at(x, y) {
        if let Some(desc_str) = check_entity.yield_desc() {
            if tx_dialogue.send(desc_str).is_err() {
                set_to_abort = true;
            }
        }
    }
    set_to_abort
}

pub fn controller_main(tx_control: &mpsc::Sender<CoordItem>,
                       rx_control: &mpsc::Receiver<char>,
                       tx_dialogue: &mpsc::Sender<String>,
                       cb_sink: cursive::CbSink) {
    let mut x_t = 8;
    let mut y_t = 8;
    let mut key_press: [char; 8] = ['\0'; 8];
    let mut key_ind = 0;
    let mut key_ready = false;

    let mut debug_in_str: String;
    let mut debug_cnt_str;

    let croc: entity::EntityBase =
        entity::EntityBase::new( 10, 4, '<',
            Some("This is a crocodile. Her teeth are sharp.".to_string()) );

    let rock: entity::EntityBase =
        entity::EntityBase::new( 10, 5, 'o', None );

    let player: entity::EntityBase =
        entity::EntityBase::new( 10, 10, '@',
        Some("This is you. You're going to meet God.".to_string()) );

    let mut entities: [entity::EntityBase; 3] = [player, croc, rock];

    loop {

        debug_in_str = String::from("debug:");
        debug_cnt_str = 0;

        while let Ok(key_p) = rx_control.try_recv() {
            key_press[key_ind] = key_p;
            key_ind += 1;
            key_ready = true;
            debug_cnt_str += 1;
            debug_in_str.push(key_p);
        }

        if debug_cnt_str >= 2 {
            if tx_dialogue.send(debug_in_str).is_err() {
                return; //Break out of loop when other side fails
                }
        }

        loop {
            if key_ready == false {
                break;
            }
            match key_press[key_ind] {
                'j' => {entities[0].mv(-1,0); x_t -= 1},
                'k' => {entities[0].mv(0,1);  y_t += 1},
                'i' => {entities[0].mv(0,-1); y_t -= 1},
                'l' => {entities[0].mv(1,0);  x_t += 1},

                'a' => x_t -= 1,
                's' => y_t += 1,
                'w' => y_t -= 1,
                'd' => x_t += 1,
                'A' => x_t -= 5,
                'S' => y_t += 5,
                'W' => y_t -= 5,
                'D' => x_t += 5,

                'q' => for ent in &entities {
                            if send_description(tx_dialogue, ent, x_t, y_t) {
                                return; //Fault, return...
                            }
                        }

                _  => (),
            }

            key_press[key_ind] = '\0';

            if key_ind == 0 {
                key_ready = false;
                break;
            } else {
                key_ind -= 1;
            }
        }

        let target = CoordItem {x: x_t as usize, y: y_t as usize, c: 'X'};

        if tx_control.send(target).is_err() {
            return; //Break out of loop when other side fails
        }
        for ent in &entities {
            if tx_control.send(ent.to_coord_item()).is_err() {
                return; //Break out of loop when other side fails
            }
        }

        if x_t == 1 && y_t == 1 {
            if tx_dialogue.send("Update...".to_string()).is_err() {
                return; //Break out of loop when other side fails
            }
            x_t = 2
        }

        cb_sink.send(Box::new(Cursive::noop)).unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}


