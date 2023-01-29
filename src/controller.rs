use cursive::Cursive;

use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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


pub fn controller_main(tx_control: &mpsc::Sender<CoordItem>,
                       rx_control: &mpsc::Receiver<char>,
                       tx_dialogue: &mpsc::Sender<String>,
                       cb_sink: cursive::CbSink) {
    let mut x = 8;
    let mut y = 8;
    let mut x_t = 8;
    let mut y_t = 8;
    let mut key_press: [char; 8] = ['\0'; 8];
    let mut key_ind = 0;
    let mut key_ready = false;

    loop {

        while let Ok(key_p) = rx_control.try_recv() {
            key_press[key_ind] = key_p;
            key_ind += 1;
            key_ready = true;

        }

        loop {
            if key_ready == false {
                break;
            }
            match key_press[key_ind] {
                'j' => {x -= 1; x_t -= 1},
                'k' => {y += 1; y_t += 1},
                'i' => {y -= 1; y_t -= 1},
                'l' => {x += 1; x_t += 1},

                'a' => x_t -= 1,
                's' => y_t += 1,
                'w' => y_t -= 1,
                'd' => x_t += 1,

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


        let player = CoordItem {x: x, y: y, c: '@'};
        let target = CoordItem {x: x_t, y: y_t, c: 'X'};

        if tx_control.send(target).is_err() {
            return; //Break out of loop when other side fails
        }
        if tx_control.send(player).is_err() {
            return; //Break out of loop when other side fails
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


