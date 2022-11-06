use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, NamedView, SelectView, TextView};
use cursive::traits::*;
use std::{thread, time};


struct Controller {
    counter : u64,
    ui_root : CursiveRunnable,
    sleep_dur : time::Duration,
}

impl Controller {

    fn run(&mut self) {
        loop {
            self.counter += 1;
            println!("{}...", self.counter);

            self.ui_root.find_name::<TextView>("sole")
                .expect("Ohhhh")
                .set_content(self.counter.to_string());

            thread::sleep(self.sleep_dur);

            if (self.counter == 34)
            {
                break;
            }
        }
    }

}


//Let this be the thread that actually has the view...
fn main() {
    let mut controller = Controller{
        counter: 0,
        ui_root: cursive::default(),
        sleep_dur: time::Duration::from_millis(100),
    };


    controller.ui_root.add_layer(
            NamedView::new("sole", TextView::new("Hello cursive-in-a-struct!")));
    controller.ui_root.set_fps(30);
    controller.run();
    controller.ui_root.run();
}
