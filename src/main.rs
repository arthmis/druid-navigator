use std::{collections::HashMap, sync::Arc, thread::current};

use druid::{
    widget::{
        Button, Click, Container, ControllerHost, Flex, Label, ListIter, ScopeTransfer, SizedBox,
        WidgetExt,
    },
    AppLauncher, Color, Command, Env, EventCtx, ImageBuf, Selector, Target, Widget, WidgetPod,
    WindowDesc,
};
use druid::{Lens, LensExt};

mod example_data;
mod navigator;
mod view;

use navigator::{Navigator, View, ViewController};
fn main() {
    let window = WindowDesc::new(view::navigator).title("Navigation");
    // let window = WindowDesc::new(view::contact_edit).title("Navigation");

    let contacts = example_data::get_data();

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppState {
            app_name: "This is a paragraph about the Navigator.".to_string(),
            nav_state: Arc::new(vec![UiView::new("contacts".to_string())]),
            contacts: Arc::new(contacts),
            selected: None,
        })
        .unwrap();
}

use druid::Data;
#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    app_name: String,
    nav_state: Arc<Vec<UiView>>,
    contacts: Arc<Vec<Contact>>,
    selected: Option<usize>,
}

#[derive(Clone, Data, Lens, Debug)]
pub struct EditState {
    contact: Contact,
    index: usize,
    was_saved: bool,
}

impl EditState {
    pub fn new(data: AppState) -> Self {
        let (contact, index) = if let Some(idx) = data.selected {
            (data.contacts[idx].clone(), idx)
        } else {
            (
                Contact::new("".to_owned(), "".to_owned(), 31, "".to_owned()),
                0,
            )
        };
        Self {
            contact,
            index,
            was_saved: false,
        }
    }
}
pub struct EditTransfer;
impl ScopeTransfer for EditTransfer {
    type In = AppState;

    type State = EditState;

    fn read_input(&self, state: &mut Self::State, inner: &Self::In) {
        // only read data in if the input was saved
        // I don't know if this is correct, can there be data raced???
        if state.was_saved {
            let selected = inner.selected;
            let idx = if let Some(idx) = selected { idx } else { 0 };
            state.contact = inner.contacts[idx].clone();
            state.index = idx;
            state.was_saved = false;
        }
    }

    fn write_back_input(&self, state: &Self::State, inner: &mut Self::In) {
        // also don't know if this will work. Will the save button update the save first
        // before this is called??
        if state.was_saved {
            let contacts = Arc::make_mut(&mut inner.contacts);
            contacts[state.index] = state.contact.clone();
            inner.contacts = Arc::new(contacts.to_owned());
        }
    }
}

// a little special implementation to give the list view all that it needs
impl ListIter<(Arc<Vec<UiView>>, Contact, Option<usize>, usize)> for AppState {
    fn for_each(
        &self,
        mut cb: impl FnMut(&(Arc<Vec<UiView>>, Contact, Option<usize>, usize), usize),
    ) {
        for (idx, contact) in self.contacts.iter().enumerate() {
            let nav_state = self.nav_state.clone();
            cb(&(nav_state, contact.clone(), self.selected, idx), idx);
        }
    }

    fn for_each_mut(
        &mut self,
        mut cb: impl FnMut(&mut (Arc<Vec<UiView>>, Contact, Option<usize>, usize), usize),
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
    age: u32,
    image: ImageBuf,
}
impl Contact {
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        age: u32,
        favorite_food: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let email = email.into();
        // let age = age.into();
        let favorite_food = favorite_food.into();
        Self {
            name,
            email,
            favorite_food,
            age,
            image: ImageBuf::empty(),
        }
    }
}

// This is the View type that Navigator will use. I want this to hold
// a data type that can be hashed which should open navigator to use more types
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UiView {
    name: String,
}
impl UiView {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
impl View for UiView {}

impl ViewController<UiView> for AppState {
    // fn add_view(&mut self, widget: impl Fn() -> Box<dyn Widget<AppState> + 'static>) {
    fn add_view(&mut self, view: UiView) {
        let views: &mut Vec<UiView> = Arc::make_mut(&mut self.nav_state);
        views.push(view);
        let views = Arc::new(views.clone());
        self.nav_state = views;
    }

    fn pop_view(&mut self) {
        let views = Arc::make_mut(&mut self.nav_state);
        views.pop();
        let views = Arc::new(views.clone());
        self.nav_state = views;
    }

    fn current_view(&self) -> &UiView {
        self.nav_state.last().unwrap()
    }

    fn len(&self) -> usize {
        self.nav_state.len()
    }
}
