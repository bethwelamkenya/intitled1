mod database;
mod member;
mod admin_home;

use std::collections::hash_map::RandomState;
use std::ops::{Range, RangeFrom};
use druid::{AppLauncher, Env, LocalizedString, Widget, WidgetExt, WindowDesc, Data, Lens, ImageBuf, theme, Color, RenderContext, TextAlignment, RoundedRectRadii, FontFamily};
use druid::kurbo::{Arc, Affine, Point, Rect, Shape};
use druid::platform_menus::win::file::new;
use druid::text::TextComponent;
use druid::widget::{Align, Flex, Label, LabelText, TextBox, prelude::*, FillStrat, Image, Checkbox, CrossAxisAlignment, RadioGroup, SizedBox, Painter, Button, Axis, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition, ViewSwitcher, MainAxisAlignment, Split, BackgroundBrush};
use druid::widget::LabelText::Static;
use image::GenericImageView;
// use druid::im::Vector;
use druid::piet::InterpolationMode;
use druid::text::ParseFormatter;
use crate::TextAlignment::Center;
use std::time::Duration;
use random_str as random;


const VERTICAL_WIDGET_SPACING: f64 = 30.0;
const TEXT_BOX_WIDTH: f64 = 300.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Log In");
const WINDOW_TITLE_ADMIN: LocalizedString<AdminState> = LocalizedString::new("Admin Home");
// const PAINTER: Painter<AdminState> = Painter::new(|ctx, _: &AdminState, env| {
//     let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
//     ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
//     if ctx.is_hot() {
//         ctx.stroke(bounds, &Color::grey(0.3), 0.2);
//     }
//     if ctx.is_active() {
//         ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
//     }
// });

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    password: String,
    button: bool,
}

#[derive(Clone, Data, Lens)]
struct AdminState {
    id: String,
    name: String,
    email: String,
    reg_no: String,
    number: String,
    school: String,
    year: String,
    department: String,
    residence: String,
    attendance_id: String,
    date: String,
    // button: bool,
}

struct AdminStateWidget {
    state: AdminState,
}

impl AdminStateWidget {
    fn new(state: AdminState) -> Self {
        AdminStateWidget { state }
    }
}

impl Widget<AdminState> for AdminStateWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AdminState, env: &Env) {
        // handle events here
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AdminState, env: &Env) {
        // handle lifecycle events here
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AdminState, data: &AdminState, env: &Env) {
        // handle updates here
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AdminState, env: &Env) -> Size {
        // layout the widget and return its size
        Size::ZERO
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AdminState, env: &Env) {
        let bounds = paint_ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
        paint_ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
        if paint_ctx.is_hot() {
            paint_ctx.stroke(bounds, &Color::grey(0.3), 0.2);
        }
        if paint_ctx.is_active() {
            paint_ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    }
}

fn my_painter() -> Painter<(AdminState)> {
    Painter::new(|paint_ctx, _, env| {
        let bounds = paint_ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
        paint_ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
        if paint_ctx.is_hot() {
            paint_ctx.stroke(bounds, &Color::grey(0.3), 0.2);
        }
        if paint_ctx.is_active() {
            paint_ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    })
}

// impl From<&Painter<()>> for BackgroundBrush<AdminState> {
//     fn from(value: &Painter<()>) -> Self {
//
//     }
//     // your implementation here
// }


//
// struct MyData { is_enabled: bool }
//
// let my_painter = Painter::new(|ctx, data: &MyData, env| {
// let bounds = ctx.size().to_rect();
// if data.is_enabled {
// ctx.fill(bounds, &env.get(ENABLED_BG_COLOR));
// } else {
//
// ctx.fill(bounds, &env.get(DISABLED_BG_COLOR));
// }
// });

// struct CustomPainter;
//
// impl Widget<()> for CustomPainter {
//     fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {}
//
//     fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}
//
//     fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}
//
//     fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
//         Size::ZERO
//     }
//
//     fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
//         let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
//         ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
//         if ctx.is_hot() {
//             ctx.stroke(bounds, &Color::grey(0.3), 0.2);
//         }
//         if ctx.is_active() {
//             ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
//         }
//     }
// }
//
//
// // impl Widget<()> for CustomPainter {
// //     fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &()) {
// //         let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
// //         ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
// //         if ctx.is_hot() {
// //             ctx.stroke(bounds, &Color::grey(0.3), 0.2);
// //         }
// //         if ctx.is_active() {
// //             ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
// //         }
// //     }
// // }
//
// // Define a painter that draws a colored rectangle
// #[derive(Clone, Copy)]
// struct ColoredRectPainter {
//     color: Color,
// }
//
// impl From<ColoredRectPainter> for BackgroundBrush<Color> {
//     fn from(painter: ColoredRectPainter) -> Self {
//         BackgroundBrush::Solid(painter.color)
//     }
// }
//
// // impl From<ColoredRectPainter> for Box<dyn Painter<AdminState>> {
// //     fn from(painter: ColoredRectPainter) -> Self {
// //         Box::new(painter)
// //     }
// // }
// //
// //
// // impl From<ColoredRectPainter> for Box<dyn Painter<AdminState>> {
// //     fn from(painter: ColoredRectPainter) -> Self {
// //         Box::new(painter)
// //     }
// // }
//
//
// impl ColoredRectPainter {
//     fn new(color: Color) -> Self {
//         ColoredRectPainter { color }
//     }
// }
//
// impl Widget<()> for ColoredRectPainter {
//     fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {}
//
//     fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}
//
//     fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}
//
//     fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
//         Size::ZERO
//     }
//     fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
//         let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
//         ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
//         if ctx.is_hot() {
//             ctx.stroke(bounds, &Color::grey(0.3), 0.2);
//         }
//         if ctx.is_active() {
//             ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
//         }
//         // let size = ctx.size();
//         // let rect = size.to_rect();
//         // ctx.fill(rect, &self.color);
//     }
// }

// #[derive(Clone, Copy)]
// struct MyPainter;
//
// impl Painter<AdminState> for MyPainter {
//     fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &MyWidgetData, env: &Env) {
//         let bounds = paint_ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
//         paint_ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
//         if paint_ctx.is_hot() {
//             paint_ctx.stroke(bounds, &Color::grey(0.3), 0.2);
//         }
//         if paint_ctx.is_active() {
//             paint_ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
//         }
//     }
// }


// fn main(){
//     let a = 39;
//     println!("{}", a);
// }

fn main() {
    // let main_window = WindowDesc::new(ui_builder)
    //     .title(LocalizedString::new("My Awesome App"))
    //     .window_size((400.0, 400.0));

    // describe the main window
    // let main_window = WindowDesc::new(build_log_in_widgets())
    //     .title(WINDOW_TITLE)
    //     .window_size((700.0, 500.0));

    // describe the main window
    let main_window = WindowDesc::new(build_admin_home())
        .title(WINDOW_TITLE_ADMIN)
        .window_size((900.0, 700.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "".into(),
        password: "".into(),
        button: false.into(),
    };

    let initial_admin_state = AdminState{
        id: "".to_string(),
        name: "".to_string(),
        email: "".to_string(),
        reg_no: "".to_string(),
        number: "".to_string(),
        school: "".to_string(),
        year: "".to_string(),
        department: "".to_string(),
        residence: "".to_string(),
        attendance_id: "".to_string(),
        date: "".to_string(),
    };

    // start the application
    // AppLauncher::with_window(main_window)
    //     .launch(initial_state)
    //     .expect("Failed to launch application");

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_admin_state)
        .expect("Failed to launch application");
}

// // Define a simple painter that draws a red rectangle
// fn painter(ctx: &mut PaintCtx, _data: &(), _env: &()) {
//     let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
//     ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
//     if ctx.is_hot() {
//         ctx.stroke(bounds, &Color::grey(0.3), 0.2);
//     }
//     if ctx.is_active() {
//         ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
//     }
//     // let size = ctx.size();
//     // let rect = size.to_rect();
//     // ctx.fill(rect, &Color::RED);
// }

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

fn build_log_in_widgets() -> impl Widget<HelloState> {
    // let png_data = ImageBuf::from_data(include_bytes!("./images/r1.jpg")).unwrap();
    let painter = Painter::new(|ctx, _: &HelloState, env| {
        let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
        ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
        if ctx.is_hot() {
            ctx.stroke(bounds, &Color::grey(0.3), 0.2);
        }
        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });
    // let painter1 = Painter::new(|ctx, _: &HelloState, env| {
    //     let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
    //     ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
    //     if ctx.is_hot() {
    //         ctx.stroke(bounds, &Color::grey(0.3), 0.2);
    //     }
    //     if ctx.is_active() {
    //         ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
    //     }
    // });
    // let button1 = Button::from_label(Label::new("Submit").with_text_color(Color::grey(0.7)))
    //     // .background(painter1)
    //     .fix_width(100.0)
    //     .fix_height(50.0);
        // .paint(data: &mut HelloState, _env|() )
        // .set_rounded(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0))
        // .clear_background()
        // .foreground(painter1);
    let button = Label::new("Submit")
        .with_text_size(24.0);
    let church_name = Label::new(LocalizedString::new("Moi University Main Altar")).with_text_size(24.0)
        // .with_text_alignment(Center)
        .with_text_color(Color::grey(0.9));
    let user_name_label = Label::new(LocalizedString::new("User Name")).with_text_size(18.0)
        .with_text_color(Color::grey(0.6));
    let password_label = Label::new(LocalizedString::new("Password")).with_text_size(18.0)
        .with_text_color(Color::grey(0.6));
    let forgot_text_label = Label::new(LocalizedString::new("Forgot Password?")).with_text_size(18.0)
        .with_text_color(Color::grey(0.6));
    let forgot_link_label = Label::new(LocalizedString::new("Reset Password")).with_text_size(18.0)
        .with_text_color(Color::rgb8(0xaa, 0x55, 0xaa))
        .on_click(move |_ctx, data: &mut HelloState, _env| {
            // data.digit()
            // main().main_window.hide;
            admin_home::main;
        });
    let user_name = TextBox::new()
        .with_placeholder("User Name")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);
    let password = TextBox::new()
        .with_placeholder("Password")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::password);
    let horizontal_layout_labels = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(user_name_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password_label);
    let horizontal_layout_text_boxes = Flex::column()
        .with_child(user_name)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(password);
    let button_area = Flex::column()
        .with_child(button)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .fix_width(100.0)
        .fix_height(50.0)
        .background(painter)
        .on_click(move |_ctx, data: &mut HelloState, _env| data.digit());
    let form_rows = Flex::row()
        .with_child(horizontal_layout_labels)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(horizontal_layout_text_boxes);
    let forgot_password_layout = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(forgot_text_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(forgot_link_label);
    let flex_layout = Flex::column()
        .with_child(church_name)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(form_rows)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Align::centered(button_area))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(forgot_password_layout);
    Align::centered(flex_layout)
}

// fn get_painted(){
//     Painter::new(|ctx, _: &AdminState, env| {
//         let bounds = ctx.size().to_rounded_rect(RoundedRectRadii::new(10.0, 10.0, 10.0, 10.0));
//         ctx.fill(bounds, &Color::rgb8(0x3c, 0x35, 0x35));
//         if ctx.is_hot() {
//             ctx.stroke(bounds, &Color::grey(0.3), 0.2);
//         }
//         if ctx.is_active() {
//             ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
//         }
//     })
// }

fn build_admin_home() -> impl Widget<AdminState>{

    // let _paint = get_painted();
    // let painter = ColoredRectPainter::new(Color::rgb8(0x3c, 0x35, 0x35));
    // let my_painter = AdminStateWidget{state: AdminState{
    //     id: "".to_string(),
    //     name: "".to_string(),
    //     email: "".to_string(),
    //     reg_no: "".to_string(),
    //     number: "".to_string(),
    //     school: "".to_string(),
    //     year: "".to_string(),
    //     department: "".to_string(),
    //     residence: "".to_string(),
    //     attendance_id: "".to_string(),
    //     date: "".to_string(),
    // }};
    let member_id_label = Label::new("ID");
    let member_name_label = Label::new("Name");
    let member_email_label = Label::new("Email");
    let member_reg_no_label = Label::new("Reg No");
    let member_number_label = Label::new("Number");
    let member_school_label = Label::new("School");
    let member_year_label = Label::new("Year");
    let member_department_label = Label::new("Department");
    let member_residence_label = Label::new("Residence");

    let member_id = TextBox::new()
        .with_placeholder("ID")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::id);
    let member_name = TextBox::new()
        .with_placeholder("Name")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::name);
    let member_email = TextBox::new()
        .with_placeholder("Email")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::email);
    let member_reg_no = TextBox::new()
        .with_placeholder("Reg No")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::reg_no);
    let member_number = TextBox::new()
        .with_placeholder("Number")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::number);
    let member_school = TextBox::new()
        .with_placeholder("School")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::school);
    let member_year = TextBox::new()
        .with_placeholder("Year")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::year);
    let member_department = TextBox::new()
        .with_placeholder("Department")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::department);
    let member_residence = TextBox::new()
        .with_placeholder("Residence")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AdminState::residence);

    let fetch_members = Label::new("Fetch Members")
        .fix_width(100.0)
        .fix_height(50.0)
        .background(my_painter())
        .on_click(move |_ctx, data: &mut AdminState, _env| {});
    let clear_form = Label::new("Clear Form")
        .fix_width(100.0)
        .fix_height(50.0)
        .background(my_painter())
        .on_click(move |_ctx, data: &mut AdminState, _env| {});
    let insert_member = Label::new("Insert Member")
        .fix_width(100.0)
        .fix_height(50.0)
        .background(my_painter())
        .on_click(move |_ctx, data: &mut AdminState, _env| {});
    let refresh = Label::new("Refresh")
        .fix_width(100.0)
        .fix_height(50.0)
        .background(my_painter())
        .on_click(move |_ctx, data: &mut AdminState, _env| {});
    let delete_member = Label::new("Delete Member")
        .fix_width(100.0)
        .fix_height(50.0)
        .background(my_painter())
        .on_click(move |_ctx, data: &mut AdminState, _env| {});
    let print_data = Label::new("Print Data")
        .fix_width(100.0)
        .fix_height(50.0)
        .background(my_painter())
        .on_click(move |_ctx, data: &mut AdminState, _env| {});

    let member_labels = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(member_id_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_name_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_email_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_reg_no_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_number_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_school_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_year_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_department_label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_residence_label);
    let member_text_boxes = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(member_id)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_name)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_email)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_reg_no)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_number)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_school)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_year)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_department)
        .with_spacer(VERTICAL_WIDGET_SPACING - 9.0)
        .with_child(member_residence);
    let member_details = Flex::row()
        .with_child(member_labels)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(member_text_boxes);
    let buttons_row_1 = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(fetch_members)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(clear_form)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(insert_member);
    let buttons_row_2 = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(refresh)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(delete_member)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(print_data);
    let member_panel = Flex::column()
        .with_child(member_details)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(buttons_row_1)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(buttons_row_2);
    let main_widget = Flex::row()
        .with_child(member_panel)
        .with_spacer(VERTICAL_WIDGET_SPACING);
    Align::centered(main_widget)
}


impl HelloState{
    fn digit(&mut self) {
        if !self.button {
            self.name.clear();
            self.button = true;
            // self.password = "Bethu".to_string();
        }
        self.name  = "Bethu".to_string();
        // let ch = (b'0' + digit) as char;
        // self.value.push(ch);
    }
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