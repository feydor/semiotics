use cursive::{
    event::{Event},
    traits::*,
    views::{
        Dialog, EditView, LinearLayout, ScrollView, SelectView,
        TextView,
    },
    Cursive,
};
use log::LevelFilter;
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
    siv.set_theme(cursive::theme::Theme::default().with(|theme| {
        use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*, BorderStyle::*};
        theme.shadow = false;
        theme.palette[Background] = TerminalDefault;
        theme.palette[Primary] = Dark(Black);
        theme.palette[Secondary] = Dark(Black);
        theme.palette[Tertiary] = Dark(Black);
        theme.palette[TitlePrimary] = Dark(Red);
        theme.palette[TitleSecondary] = Dark(Black);
        theme.palette[Highlight] = Dark(Red);
        theme.palette[HighlightInactive] = Dark(Black);
    }));

    siv.add_global_callback(Event::Key(cursive::event::Key::Esc), |s| s.quit());

    let input_box = EditView::new().on_submit(handle_submit).with_name("input_box").full_width();
    let results = SelectView::<String>::new().item(input.clone(), input.clone()).with_name("results").full_height();
    let scroll_results = ScrollView::new(results);

    siv.add_layer(
        LinearLayout::vertical()
            .child(
                Dialog::around(
                    LinearLayout::vertical()
                        .child(input_box)
                        .child(scroll_results)
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
        ana::gram::remove_letters_from_sentence(&sentence[0..], &letters).join(" ").to_ascii_uppercase();
    if result != lowercase {
        results_list.add_item(result.clone(), result);
    }
}
