use cursive::traits::*;
use cursive::Vec2;
use cursive::{Cursive, Printer};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct CoordItem {
    x: usize,
    y: usize,
    c: char,
}

impl CoordItem {

    fn new() -> CoordItem {
        let new_item = CoordItem {x: 0, y: 0, c: ' '};
        return new_item
    }
}

fn main() {
    // As usual, create the Cursive root
    let mut siv = cursive::default();

    // Setup full duplex channel communications...
    let cb_sink = siv.cb_sink().clone();

    let (tx_control, rx_view) = mpsc::channel();
    let (tx_view, rx_control) = mpsc::channel();

    siv.set_user_data::<mpsc::Sender<char>>(tx_view);

    // We want to refresh the page even when no input is given.
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('h', |s| {send_key_to_control(s,'h')});
    siv.add_global_callback('j', |s| {send_key_to_control(s,'j')});
    siv.add_global_callback('k', |s| {send_key_to_control(s,'k')});
    siv.add_global_callback('l', |s| {send_key_to_control(s,'l')});

    // Generate data in a separate thread.
    thread::spawn(move || {
        controller_main(&tx_control, &rx_control,cb_sink);
    });

    // And sets the view to read from the other end of the channel.
    siv.add_layer(BufferView::new(1, rx_view).full_screen());
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

fn controller_main(tx_control: &mpsc::Sender<CoordItem>,
                   rx_control: &mpsc::Receiver<char>,
                   cb_sink: cursive::CbSink) {
    let mut x = 8;
    let mut y = 8;
    let mut key_press: [char; 8] = ['\0'; 8];
    let mut key_ind = 0;

    loop {

        while let Ok(key_p) = rx_control.try_recv() {
            key_press[key_ind] = key_p;
            key_ind += 1;
        }

        match key_press[key_ind] {
            'h' => x -= 1,
            'j' => y += 1,
            'k' => y -= 1,
            'l' => x += 1,
             _  => (),
        }


        let line = CoordItem {x: x, y: y, c: '@'};

        if tx_control.send(line).is_err() {
            return; //Break out of loop when other side fails
        }
        cb_sink.send(Box::new(Cursive::noop)).unwrap();
        thread::sleep(Duration::from_millis(30));
        key_press[key_ind] = '\0';
        key_ind = 0;

    }
}

// Let's define a buffer view, that shows the last lines from a stream.
struct BufferView {
    // We'll use a ring buffer
    buffer: VecDeque<CoordItem>,
    // Receiving end of the stream
    rx_view: mpsc::Receiver<CoordItem>,
}

impl BufferView {
    // Creates a new view with the given buffer size
    fn new(size: usize, rx_view: mpsc::Receiver<CoordItem>) -> Self {
        let mut buffer = VecDeque::new();
        buffer.resize(size, CoordItem::new());
        BufferView { buffer, rx_view }
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

impl View for BufferView {
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the end of the buffer
        for (_i, line) in
            self.buffer.iter().rev().take(printer.size.y).enumerate()
        {
            if line.y < printer.size.y && line.x < printer.size.x //Does not like overly large numbers
            {
                printer.print((line.x, line.y), &line.c.to_string());
            }
        }
    }
}
