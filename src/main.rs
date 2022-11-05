use cursive::event::Event::Refresh;
use cursive::views::Dialog;
use cursive::views::TextContent;
use cursive::views::TextView;
use cursive::views::OnEventView;

use std::clone;


fn refesh_no(no: &mut TextContent, incr: &mut u64)
{
    *incr = *incr + 1;
    no.set_content(incr.to_string());

    //let new_no: u64 = (*text).source().parse::<u64>().unwrap() + 1;
    //no.set_content(new_no.to_string()); //This was a fucking nightmare
}

fn main() {
    let mut siv = cursive::default();
    let mut txt_view = TextContent::new("0");
    let mut txt_count = 0;

    refesh_no(&mut txt_view, &mut txt_count);

    let mut update_view = OnEventView::new(
                            TextView::new_with_content(txt_view.clone()));

    siv.add_layer(Dialog::around(update_view)
                 .title("Cursive")
                 .button("Quit", |n| n.quit()));

    refesh_no(&mut txt_view, &mut txt_count);

    siv.set_fps(10);
    siv.set_event
    siv.run();
}
