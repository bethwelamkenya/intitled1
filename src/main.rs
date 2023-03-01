mod database;
mod member;

use std::collections::hash_map::RandomState;
use std::ops::{Range, RangeFrom};
use druid::{AppLauncher, Env, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::kurbo::Arc;
use druid::platform_menus::win::file::new;
use druid::text::TextComponent;
use druid::widget::{Align, Flex, Label, LabelText, TextBox};
use druid::widget::LabelText::Static;
use druid::{Data, Lens};
use image::GenericImageView;
use random_str as random;


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    // let main_window = WindowDesc::new(ui_builder)
    //     .title(LocalizedString::new("My Awesome App"))
    //     .window_size((400.0, 400.0));

    let data = 0;
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    // AppLauncher::with_window(main_window)
    //     .launch(data)
    //     .expect("launch failed");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}

fn get_random_number(a : i32, b : i32) -> Range<i32> {
    println!("{}", a + b);
    return Range {start: 20, end: 50};
}

fn random_str_examples(){
    let random_letter = random::get_letter(true, true);
    println!("Random letter: {}", random_letter);

    let random_symbol = random::get_symbol();
    println!("Random symbol: {}", random_symbol);

    let random_number = random::get_int(0, 9);
    println!("Random number: {}", random_number);

    let seven_digits = random::get_int(1000000, 9999999); // 7 digits
    let random_phone_number = format!("+52 343{}", seven_digits);
    println!("Random phone number: {}", random_phone_number);

    let random_password = random::get_string(16, true, true, true, true);
    println!("Random password: {}", random_password);

    let random_bool = random::get_bool();
    println!("Random bool: {}", random_bool);

    let random_char = random::get_char();
    println!("Random char: {}", random_char);
}