use std::collections::hash_map::RandomState;
use std::ops::{Range, RangeFrom};
use druid::{AppLauncher, Env, LocalizedString, Widget, WidgetExt, WindowDesc, Data, Lens, ImageBuf};
use druid::kurbo::Arc;
use druid::platform_menus::win::file::new;
use druid::text::TextComponent;
use druid::widget::{Align, Flex, Label, LabelText, TextBox, prelude::*, FillStrat, Image, Checkbox, CrossAxisAlignment, RadioGroup, SizedBox};
use druid::widget::LabelText::Static;
use image::GenericImageView;
use druid::piet::InterpolationMode;
use druid::text::ParseFormatter;
use random_str as random;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 300.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    password: String,
}

pub fn main(){
    // describe the main window
    let main_window = WindowDesc::new(build_admin_home_widgets())
        .title(WINDOW_TITLE)
        .window_size((700.0, 500.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
        password: "****".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_admin_home_widgets() -> impl Widget<HelloState>{
    // let png_data = ImageBuf::from_data(include_bytes!("./images/r1.jpg")).unwrap();
    let church_name = Label::new(LocalizedString::new("Moi University Main Altar"));
    let user_name_label = Label::new(LocalizedString::new("User Name"));
    let password_label = Label::new(LocalizedString::new("Password"));
    let user_name = TextBox::new()
        .with_placeholder("User Name")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);
    let password = TextBox::new()
        .with_placeholder("Password")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::password);
    let horizontal_layout_labels = Flex::column()
        .with_child(user_name_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password_label);
    let horizontal_layout_text_boxes = Flex::column()
        .with_child(user_name)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password);

    let form_rows = Flex::row()
        .with_child(horizontal_layout_labels)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(horizontal_layout_text_boxes);
    let flex_layout = Flex::column()
        .with_child(church_name)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(form_rows);
    Align::centered(flex_layout)
}