use cursive::{
    event::{Event, EventResult, Key},
    traits::*,
    view::{scroll::Scroller, Scrollable},
    views::{
        Button, Dialog, EditView, LinearLayout, OnEventView, Panel, ScrollView, SelectView,
        TextView,
    },
    Cursive,
};
use log::LevelFilter;
use log::{info, trace, warn};
use std::process;

fn usage_and_exit() {
    println!("program: ana \"sentence\"");
    process::exit(1);
}

fn main() {
    simple_logging::log_to_file("test.log", LevelFilter::Info).expect("simple_logging failed");
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            if args[1] == "-h" || args[1] == "--help" {
                usage_and_exit();
            }
        }
        _ => usage_and_exit(),
    }

    let input: String = args[1].trim().to_string();
    let mut siv = cursive::default();
    
    siv.add_global_callback(Event::Key(cursive::event::Key::Esc), |s| s.quit());

    let input_box = EditView::new().on_submit(handle_submit).with_name("input_box").full_width();
    let results = SelectView::<String>::new().item(input.clone(), input.clone()).with_name("results").full_height();
    let scroll_results = ScrollView::new(results);

    siv.add_layer(
        LinearLayout::vertical()
            .child(
                Dialog::around(
                    LinearLayout::vertical()
                        .child(scroll_results)
                        .child(input_box)
                        .full_width(),
                )
                .title("ana"),
            )
            .child(
                LinearLayout::horizontal()
                    .child(TextView::new("F1: Menu | ").with_name("menu_bar"))
                    .child(TextView::new("").with_name("status_bar"))
                    .full_width(),
            ),
    );

    siv.run();
}

fn siv_popup(siv: &mut Cursive, msg: &str) {
    siv.add_layer(Dialog::text(msg).button("Ret", |s| {s.pop_layer();}));
}

fn handle_submit(siv: &mut Cursive, text: &str) {
    let mut results_list = siv.find_name::<SelectView<String>>("results").unwrap();
    let input = results_list.selection().unwrap();
    let lowercase = input.to_ascii_lowercase();
    let sentence: Vec<&str> = lowercase.split(' ').collect();
    let letters: Vec<char> = text.chars().collect();
    let result: String =
        anars::ana::remove_letters_from_sentence(&sentence[0..], &letters).join("");
    results_list.add_item(result.clone(), result);
}
