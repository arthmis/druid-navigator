#![allow(warnings)]
use std::{collections::HashMap, sync::Arc, thread::current};

use druid::{
    widget::{Button, Click, Container, ControllerHost, Flex, Label, SizedBox, WidgetExt},
    AppLauncher, Color, Command, Env, EventCtx, Selector, Target, Widget, WidgetPod, WindowDesc,
};
use druid::{Lens, LensExt};
use navigator::{Navigator, ViewController};

mod navigator;
fn main() {
    let window = WindowDesc::new(navigator).title("Navigation");

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppState {
            app_name: "Navigator".to_string(),
            nav_state: Arc::new(vec![View::new("First".to_string())]),
        })
        .unwrap();
}

use druid::Data;
#[derive(Clone, Data, Lens)]
pub struct AppState {
    app_name: String,
    nav_state: Arc<Vec<View>>,
}

// pub struct View<T: Data> {
#[derive(Clone, Debug)]
pub struct View {
    name: String,
    // ui_builder: Box<dyn Fn() -> Box<dyn Widget<AppState>>>,
}
impl View {
    // pub fn new(name: String, ui_builder: impl Fn() -> Box<dyn Widget<AppState>> + 'static) -> Self {
    pub fn new(name: String) -> Self {
        // pub fn new(name: String) -> Self {
        Self {
            name,
            // ui_builder: Box::new(ui_builder),
        }
    }
}
impl ViewController for Arc<Vec<View>> {
    // fn add_view(&mut self, widget: impl Fn() -> Box<dyn Widget<AppState> + 'static>) {
    fn add_view(&mut self, view: View) {
        // let current_view = (widget)();
        // let current_view = WidgetPod::new(current_view);
        let views = Arc::make_mut(self);
        views.push(view);
        let views = Arc::new(views.clone());
        *self = views;
        // self.push(view);
    }

    fn pop_view(&mut self) {
        let views = Arc::make_mut(self);
        views.pop();
        let views = Arc::new(views.clone());
        *self = views;
    }

    fn current_view(&self) -> &View {
        self.last().unwrap()
    }

    // fn current_view_mut(&mut self) -> &mut View {
    //     self.last_mut().unwrap()
    // }

    // fn build_view(&self) -> WidgetPod<AppState, Box<dyn Widget<AppState>>> {
    //     let current_view = self.current_view_mut();
    //     let widget = (current_view.ui_builder)();
    //     WidgetPod::new(widget)
    //     // WidgetPod::new((self.ui_builder)())
    // }
}

fn first_view() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|event, data, env| {
        dbg!("pressed back button");
    });
    let next_button: ControllerHost<Button<AppState>, Click<AppState>> = Button::new("Next")
        .on_click(|event: &mut EventCtx, data: &mut AppState, env: &Env| {
            dbg!("pressed next view button");
            data.nav_state.add_view(View {
                name: "Second".to_string(),
            });
        });
    let label = Label::new("First")
        .with_text_size(20.0)
        .with_text_color(Color::BLACK);
    let layout: SizedBox<AppState> = Flex::row()
        .with_child(label)
        .with_child(back_button)
        .with_child(next_button)
        .must_fill_main_axis(true)
        .expand_height();

    Box::new(Container::new(layout).background(Color::WHITE))
}

fn second_view() -> impl Widget<AppState> {
    let back_button = Button::new("Back").on_click(|event, data: &mut AppState, env| {
        dbg!("pressed back button");
        dbg!("pressed next view button");
        data.nav_state.pop_view();
    });
    let next_button = Button::new("Next").on_click(|event, data: &mut AppState, env| {
        dbg!("pressed next view button");
        data.nav_state.add_view(View {
            name: "Second".to_string(),
        });
    });
    let label = Label::new("Second").with_text_size(20.0);
    let layout = Flex::row()
        .with_child(label)
        .with_child(back_button)
        .with_child(next_button)
        .must_fill_main_axis(true)
        .expand_height();

    Container::new(layout).background(Color::GRAY)
}

fn navigator() -> impl Widget<AppState> {
    Navigator::new(Box::new(first_view)).with_view("Second".to_string(), || Box::new(second_view()))
    // .background(Color::WHITE)
}

// fn initial_builder() -> impl Widget<String> {
//     let back_button = Button::new("Back").on_click(|event, data, env| {
//         dbg!("pressed back button");
//     });
//     let next_button = Button::new("Next").on_click(|event, data, env| {
//         dbg!("pressed next view button");
//     });
//     let layout = Flex::row()
//         .with_child(back_button)
//         .with_child(next_button)
//         .must_fill_main_axis(true)
//         .expand_height();

//     Container::new(layout).background(Color::WHITE)
// }
