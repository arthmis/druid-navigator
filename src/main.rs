#![allow(warnings)]
use std::{collections::HashMap, sync::Arc, thread::current};

use druid::{
    widget::{Button, Click, Container, ControllerHost, Flex, SizedBox, WidgetExt},
    AppLauncher, Color, Command, Env, EventCtx, Selector, Target, Widget, WidgetPod, WindowDesc,
};
use druid::{Lens, LensExt};
use navigator::{Navigator, ViewController, ViewState};

mod navigator;
fn main() {
    let window = WindowDesc::new(navigator).title("Navigation");

    AppLauncher::with_window(window)
        .use_simple_logger()
        // .launch("Some data".to_string())
        .launch(AppState {
            view_state: Arc::new(vec![View {
                name: "First".to_string(),
                ui_builder: Box::new(move || Box::new(first_view())),
            }]),
        })
        .unwrap();
}

use druid::Data;
#[derive(Clone, Data, Lens)]
pub struct AppState {
    view_state: Arc<Vec<View>>,
}

// impl AppState {
//     pub fn change_view(&mut self, view: View) {
//         self.add_view(view);
//     }

//     pub fn add_view(&mut self, new_view: View) {
//         let mut views = Arc::make_mut(&mut self.view_state).clone();
//         views.push(new_view);
//         self.view_state = Arc::new(views);
//     }
// }
// pub struct View<T: Data> {
pub struct View {
    name: String,
    ui_builder: Box<dyn Fn() -> Box<dyn Widget<View>>>,
}
impl View {
    pub fn new(name: String, ui_builder: impl Fn() -> Box<dyn Widget<View>>) -> Self {
        Self {
            name,
            ui_builder: Box::new(move || Box::new(ui_builder())),
        }
    }
}
impl ViewController<View> for ViewState<AppState> {
    // fn add_view(&mut self, widget: impl Fn() -> Box<dyn Widget<AppState> + 'static>) {
    fn add_view(&mut self, view: View) {
        // let current_view = (widget)();
        // let current_view = WidgetPod::new(current_view);
        self.push(view);
    }

    fn current_view(&self) -> &View {
        self.last().unwrap()
    }

    fn current_view_mut(&self) -> &mut View {
        self.last_mut().unwrap()
    }

    fn build_view(&self) -> WidgetPod<View, Box<dyn Widget<View>>> {
        let current_view = self.current_view_mut();
        let widget = (current_view.ui_builder)();
        WidgetPod::new(widget)
        // WidgetPod::new((self.ui_builder)())
    }
}

fn first_view() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|event, data, env| {
        dbg!("pressed back button");
    });
    let next_button: ControllerHost<Button<AppState>, Click<AppState>> = Button::new("Back")
        .on_click(|event: &mut EventCtx, data: &mut AppState, env: &Env| {
            dbg!("pressed next view button");
            // dbg!(&data);
            data.view_state.add_view(View {
                name: "Second".to_string(),
                ui_builder: Box::new((move || Box::new(second_view()))),
            });
        });
    let layout: SizedBox<AppState> = Flex::row()
        .with_child(back_button)
        .with_child(next_button)
        .must_fill_main_axis(true)
        .expand_height();

    Box::new(Container::new(layout).background(Color::WHITE))
}

fn second_view() -> impl Widget<AppState> {
    let back_button = Button::new("Back").on_click(|event, data, env| {
        dbg!("pressed back button");
    });
    let next_button = Button::new("Back").on_click(|event, data, env| {
        dbg!("pressed next view button");
    });
    let layout = Flex::row()
        .with_child(back_button)
        .with_child(next_button)
        .must_fill_main_axis(true)
        .expand_height();

    Container::new(layout).background(Color::GRAY)
}

fn navigator() -> impl Widget<AppState> {
    Navigator::new(Box::new(first_view)).lens(AppState::view_state)
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
