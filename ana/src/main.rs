use cursive::{
    event::Event,
    traits::*,
    views::{Dialog, EditView, LinearLayout, ScrollView, SelectView, TextView},
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
        x if x > 1 => {
            if args[1] == "-h" || args[1] == "--help" {
                usage_and_exit();
            }
        }
        _ => usage_and_exit(),
    }

    // load dict from pre-defined filepath
    let mut dict = ana::gram::Dict::new();
    dict.load_from_file("res/words.txt");

    let input: &str = args[1].trim();
    let mut siv = cursive::default();
    siv.set_theme(cursive::theme::Theme::default().with(|theme| {
        use cursive::theme::{BaseColor::*, Color::*, PaletteColor::*};
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

    let input = input.to_string();
    let input_box = EditView::new()
        .on_submit(handle_submit)
        .with_name("input_box")
        .full_width();
    let results = SelectView::<String>::new()
        .item(&input, input.clone())
        .with_name("results")
        .full_height();
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

    run_single_anagrams(&mut siv, &dict);
    siv.run();
}

fn run_single_anagrams(siv: &mut Cursive, dict: &ana::gram::Dict) {
    let mut results_list = siv.find_name::<SelectView<String>>("results").unwrap();
    let input = results_list.selection().unwrap();

    let anagrams = dict.anagrams(&input);
    for anagram in anagrams {
        results_list.add_item(anagram.to_string(), anagram.to_string());
    }
}

fn handle_submit(siv: &mut Cursive, text: &str) {
    let mut results_list = siv.find_name::<SelectView<String>>("results").unwrap();
    let input = results_list.selection().unwrap();
    let lowercase = input.to_ascii_lowercase();
    let sentence: Vec<&str> = lowercase.split(' ').collect();
    let letters: Vec<char> = text.chars().collect();
    let result: String = ana::gram::remove_letters_from_sentence(&sentence[0..], &letters)
        .join(" ")
        .to_ascii_uppercase();
    // TODO: show each letter of text in a different color, if its letters were used in the sentence
    if result != lowercase {
        results_list.add_item(text.to_string() + " -> " + &result, result);
    }
}
