#![allow(warnings)]
use std::{collections::HashMap, sync::Arc, thread::current};

use druid::{
    widget::{
        Button, Click, Container, ControllerHost, Flex, Label, ListIter, SizedBox, WidgetExt,
    },
    AppLauncher, Color, Command, Env, EventCtx, ImageBuf, Selector, Target, Widget, WidgetPod,
    WindowDesc,
};
use druid::{Lens, LensExt};
use druid_lens_compose::ComposeLens;

mod navigator;
mod view;

use navigator::{Navigator, ViewController};
fn main() {
    let window = WindowDesc::new(view::navigator).title("Navigation");

    let contacts = vec![
        Contact {
            name: "Billy Bob".to_string(),
            email: "Billybob@gmail.com".to_string(),
            image: ImageBuf::empty(),
            favorite_food: "Curry".to_string(),
            age: 39,
        },
        Contact {
            name: "Waka waka".to_string(),
            email: "wakaka@gmail.com".to_string(),
            image: ImageBuf::empty(),
            favorite_food: "Fried Rice".to_string(),
            age: 65,
        },
    ];
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppState {
            app_name: "Navigator".to_string(),
            nav_state: Arc::new(vec![View::new("contacts".to_string())]),
            contacts: Arc::new(contacts),
            selected: None,
        })
        .unwrap();
}

use druid::Data;
#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    app_name: String,
    nav_state: Arc<Vec<View>>,
    contacts: Arc<Vec<Contact>>,
    selected: Option<usize>,
}

impl ListIter<(Arc<Vec<View>>, Contact, Option<usize>, usize)> for AppState {
    fn for_each(
        &self,
        mut cb: impl FnMut(&(Arc<Vec<View>>, Contact, Option<usize>, usize), usize),
    ) {
        for (idx, contact) in self.contacts.iter().enumerate() {
            let nav_state = self.nav_state.clone();
            cb(&(nav_state, contact.clone(), self.selected, idx), idx);
        }
    }

    fn for_each_mut(
        &mut self,
        mut cb: impl FnMut(&mut (Arc<Vec<View>>, Contact, Option<usize>, usize), usize),
    ) {
        let mut any_shared_changed = false;
        for (idx, contact) in self.contacts.iter().enumerate() {
            let mut d = (
                self.nav_state.clone(),
                contact.clone(),
                self.selected.clone(),
                idx,
            );

            cb(&mut d, idx);
            if !any_shared_changed && !self.nav_state.same(&d.0) {
                any_shared_changed = true;
            }
            if any_shared_changed {
                self.nav_state = d.0;
                self.selected = d.2;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.contacts.len()
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct Contact {
    name: String,
    email: String,
    favorite_food: String,
    age: u8,
    image: ImageBuf,
}

// pub struct View<T: Data> {
#[derive(Clone, Debug)]
pub struct View {
    name: String,
}
impl View {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
impl ViewController for Arc<Vec<View>> {
    // fn add_view(&mut self, widget: impl Fn() -> Box<dyn Widget<AppState> + 'static>) {
    fn add_view(&mut self, view: View) {
        let views = Arc::make_mut(self);
        views.push(view);
        let views = Arc::new(views.clone());
        *self = views;
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
}
