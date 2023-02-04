use cursive::traits::*;
use cursive::Vec2;
use cursive::{Printer};

use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

mod controller;

fn main() {
    // As usual, create the Cursive root
    let mut siv = cursive::default();

    // Setup full duplex channel communications...
    let cb_sink = siv.cb_sink().clone();

    let (tx_control, rx_view) = mpsc::channel();
    let (tx_view, rx_control) = mpsc::channel();
    let (tx_dialog, rx_dialog) = mpsc::channel();

    siv.set_user_data::<mpsc::Sender<char>>(tx_view);

    // We want to refresh the page even when no input is given.
    siv.add_global_callback('X', |s| s.quit());

    let to_control_keys = ['i', 'j','k','l',
                           'w','a','s','d','W','A','S','D',
                           'q'];
    for key in to_control_keys {
        siv.add_global_callback(key, move |s| {send_key_to_control(s,key)});
    }

    // Generate data in a separate thread.
    thread::spawn(move || {
        controller::controller_main(&tx_control,
                                    &rx_control, &tx_dialog, cb_sink);
    });

    // And sets the view to read from the other end of the channel.

    //siv.add_layer(WorldView::new(3, rx_view).full_screen());
    /*siv.add_layer(DialogView::new(5, rx_dialog,
                                    cursive::view::Offset {Absolute(0),Absolute(0)})
                                .full_width()
                                .fixed_height(10)
                                .with_name("Dialog"));
    */
    siv.add_layer(
        cursive::views::LinearLayout::vertical()
            .child(WorldView::new(5, rx_view)
                                .full_width()
                                .full_height())
            .child(DialogView::new(10, rx_dialog)
                                .full_width()
                                .fixed_height(10)
                                .with_name("Dialog")) );

    siv.set_fps(30);
    siv.run();
}

fn send_key_to_control (s: &mut cursive::Cursive, c: char) {

    let key_sender = match s.user_data::<mpsc::Sender<char>>() {
        Some(present_sender) => present_sender,
        None => panic!("Missing cursive user data: mpsc tx handle"),
    };

    match key_sender.send(c) {
        Ok(()) => (),
        Err(_) => panic!("Failed key send: mpsc tx handle"),
    };

}

struct WorldView {
    // We'll use a ring buffer
    buffer: VecDeque<controller::CoordItem>,
    // Receiving end of the stream
    rx_view: mpsc::Receiver<controller::CoordItem>,
}

impl WorldView {
    // Creates a new view with the given buffer size
    fn new(size: usize,
           rx_view: mpsc::Receiver<controller::CoordItem>) -> Self {
        let mut buffer = VecDeque::new();
        buffer.resize(size, controller::CoordItem::new());
        WorldView { buffer, rx_view }
    }

    // Reads available data from the stream into the buffer
    fn update(&mut self) {
        // Add each available line to the end of the buffer.
        while let Ok(line) = self.rx_view.try_recv() {
            self.buffer.push_back(line);
            self.buffer.pop_front();
        }
    }
}

impl View for WorldView {
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the end of the buffer
        for (_i, line) in
            self.buffer.iter().rev().take(printer.size.y).enumerate()
        {
            if line.get_y() < printer.size.y && line.get_x() < printer.size.x //Does not like overly large numbers
            {
                printer.print((line.get_x(), line.get_y()), &line.get_c().to_string());
            }
        }
    }
}


struct DialogView {
    buffer: VecDeque<String>,
    rx_text: mpsc::Receiver<String>,
}

impl DialogView {
    fn new(size: usize, rx_text: mpsc::Receiver<String>
            ) -> Self {
        let mut buffer = VecDeque::new();
        buffer.resize(size, String::new());
        DialogView { buffer, rx_text }
    }

    fn update(&mut self) {
        while let Ok(line) = self.rx_text.try_recv() {
            self.buffer.push_back(line);
            self.buffer.pop_front();
        }
    }
}

impl View for DialogView {
    fn layout(&mut self, _:Vec2) {
        self.update()
    }

    fn draw(&self, printer: &Printer) {
        for (i, line) in
            self.buffer.iter().rev().take(printer.size.y).enumerate()
        {
            printer.print((0, printer.size.y - 1 - i), line);
        }
    }


}

