#![allow(warnings)]
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    hash::Hash,
    marker::PhantomData,
    process::Child,
    rc::Rc,
    sync::Arc,
    unreachable,
};

use druid::{widget::prelude::*, Point, Selector, WidgetPod};

use crate::AppState;

pub const POP_VIEW: Selector<()> = Selector::new("Pop view");
// Navigator will contain a vec of views, these will be pushed and popped
// depending on whatever buttons or other navigational tools are used
// whenever a new view is asked for a command is sent with a function
// that will build the new View
// from here the navigator will call the builder and then push this new widget
// onto the VecDeque and then repaint and do whatever it needs to do to display it
// I still have to work out how to send data to the new views if necessary
// or send data back to views when a supposed back button is clicked
// Navigator will need to someway to keep track of the state of views so it
// knows when to update the views or change to a new one depending on state changes
pub struct Navigator<T, H> {
    state: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
    views: HashMap<H, Box<dyn Fn() -> Box<dyn Widget<T>>>>,
}

impl<T: Data, H: View> Navigator<T, H> {
    pub fn new(name: H, ui_builder: impl Fn() -> Box<dyn Widget<T>> + 'static) -> Self {
        let mut views = Vec::new();
        let current_view = (ui_builder)();
        let current_view = WidgetPod::new(current_view);
        views.push(current_view);
        let mut this = Self {
            state: views,
            views: HashMap::new(),
        };
        match this.views.insert(name, Box::new(ui_builder)) {
            Some(_) => unreachable!("Map should be empty at this point"),
            None => {}
        }
        this
    }
    pub fn with_view_builder(
        mut self,
        name: H,
        view_builder: impl Fn() -> Box<dyn Widget<T>> + 'static,
    ) -> Self {
        match self.views.insert(name, Box::new(view_builder)) {
            // This can change in the future
            Some(_) => panic!("Views should never update. They should be set at navigator creation and never change."),
            None => {},
        }
        self
    }
    pub fn push_view(&mut self, view: H) {
        let ui_builder = self.views.get(&view).unwrap();
        let new_view = (ui_builder)();
        let widget = WidgetPod::new(new_view);
        self.state.push(widget);
    }
    pub fn pop_view(&mut self) {
        if self.state.len() == 1 {
            return;
        }
        self.state.pop().unwrap();
    }
}
pub trait ViewController<T: Hash + PartialEq + Eq + Clone> {
    fn add_view(&mut self, view: T);
    fn pop_view(&mut self);
    fn current_view(&self) -> &T;
    fn len(&self) -> usize;
}
pub trait View: Hash + PartialEq + Eq + Clone + fmt::Debug {}

impl<H: View, T: Data + ViewController<H>> Widget<T> for Navigator<T, H> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if event.should_propagate_to_hidden() {
            for view in self.state.iter_mut() {
                view.event(ctx, event, data, env);
            }
            match event {
                Event::Command(selector) if selector.is(POP_VIEW) => {
                    data.pop_view();
                }
                _ => (),
            }
        } else {
            self.state.last_mut().unwrap().event(ctx, event, data, env);
            match event {
                Event::Command(selector) if selector.is(POP_VIEW) => {
                    data.pop_view();
                }
                _ => (),
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if event.should_propagate_to_hidden() {
            for view in self.state.iter_mut() {
                view.lifecycle(ctx, event, data, env);
            }
        } else {
            if let LifeCycle::WidgetAdded = event {
                ctx.children_changed();
            }
            self.state
                .last_mut()
                .unwrap()
                .lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !old_data.same(data) {
            if data.len() > old_data.len() {
                if !(data.current_view() == old_data.current_view()) {
                    self.push_view(data.current_view().clone());
                    ctx.children_changed();
                }
            } else if data.len() < old_data.len() {
                self.pop_view();
                ctx.children_changed();
                self.state.last_mut().unwrap().update(ctx, data, env);
            } else {
                ctx.children_changed();
            }
        } else {
            self.state.last_mut().unwrap().update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let child_size = self.state.last_mut().unwrap().layout(ctx, bc, data, env);
        self.state
            .last_mut()
            .unwrap()
            .set_origin(ctx, data, env, Point::ZERO);
        child_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.state.last_mut().unwrap().paint(ctx, data, env)
    }
}
