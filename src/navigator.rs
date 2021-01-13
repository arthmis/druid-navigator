#![allow(warnings)]
use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    process::Child,
    rc::Rc,
    sync::Arc,
    unreachable,
};

use druid::{widget::prelude::*, Point, Selector, WidgetPod};

use crate::{AppState, View};

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
pub struct Navigator {
    state: Vec<WidgetPod<AppState, Box<dyn Widget<AppState>>>>,
    views: HashMap<String, Box<dyn Fn() -> Box<dyn Widget<AppState>>>>,
}

impl Navigator {
    pub fn new(name: String, ui_builder: impl Fn() -> Box<dyn Widget<AppState>> + 'static) -> Self {
        let mut views = Vec::new();
        let current_view = (ui_builder)();
        let current_view = WidgetPod::new(current_view);
        views.push(current_view);
        let mut this = Self {
            state: views,
            // views: map,
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
        name: String,
        view_builder: impl Fn() -> Box<dyn Widget<AppState>> + 'static,
    ) -> Self {
        match self.views.insert(name, Box::new(view_builder)) {
            // This can change in the future
            Some(_) => panic!("Views should never update. They should be set at navigator creation and never change."),
            None => {},
        }
        self
    }
    pub fn push_view(&mut self, view: View) {
        let ui_builder = self.views.get(&view.name).unwrap();
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
pub trait ViewController {
    fn add_view(&mut self, view: View);
    fn pop_view(&mut self);
    fn current_view(&self) -> &View;
}

impl Widget<AppState> for Navigator {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        if event.should_propagate_to_hidden() {
            // dbg!("should propagate");
            for view in self.state.iter_mut() {
                view.event(ctx, event, data, env);
            }
        } else {
            self.state.last_mut().unwrap().event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
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

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        if !old_data.same(data) {
            if data.nav_state.len() > old_data.nav_state.len() {
                if !(data.nav_state.last().unwrap().name == old_data.nav_state.last().unwrap().name)
                {
                    self.push_view(data.nav_state.last().unwrap().clone());
                    ctx.children_changed();
                }
            } else if data.nav_state.len() < old_data.nav_state.len() {
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

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        let child_size = self.state.last_mut().unwrap().layout(ctx, bc, data, env);
        self.state
            .last_mut()
            .unwrap()
            .set_origin(ctx, data, env, Point::ZERO);
        child_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.state.last_mut().unwrap().paint(ctx, data, env)
    }
}
