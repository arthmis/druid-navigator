#![allow(warnings)]
use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    process::Child,
    rc::Rc,
    sync::Arc,
};

use druid::{widget::prelude::*, Selector, WidgetPod};

use crate::{AppState, View};

// this view struct will handle the building of the ui
// pub struct View<T, W> {
//     ui_builder: Box<dyn Fn(&T, &Env) -> W + 'static>,
// }

// impl<T, W> View<T, W> {
//     pub fn new(ui_builder: Box<Fn(&T, &Env) -> W>) -> Self {
//         Self { ui_builder }
//     }
//     pub fn build_ui(&self, data: &T, env: &Env) -> W {
//         (self.ui_builder.as_mut())(data, env)
//     }
// }

// impl<T, W> Widget<T> for View<T, W> {
//     fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
//         self.ui.event(ctx, event, data, env)
//     }

//     fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
//         self.ui.lifecycle(ctx, event, data, env)
//     }

//     fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
//         self.ui.update(ctx, old_data, data, env)
//     }

//     fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
//         self.ui.layout(ctx, bc, data, env)
//     }

//     fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
//         self.ui.paint(ctx, data, env)
//     }
// }

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
    views: Vec<WidgetPod<AppState, Box<dyn Widget<AppState>>>>,
    // _data: PhantomData<T>,
}

// pub type ViewBuilder<T> = Box<dyn Fn() -> Box<dyn Widget<T>>>;

impl Navigator {
    // pub fn new<W: Widget<T> + 'static>(
    pub fn new(ui_builder: impl Fn() -> Box<dyn Widget<AppState> + 'static>) -> Self {
        let mut views = Vec::new();
        // let current_view = Box::new((ui_builder)());
        let current_view = (ui_builder)();
        let current_view = WidgetPod::new(current_view);
        views.push(current_view);
        Self {
            views,
            // _data: PhantomData,
        }
    }
    // pub fn add_view(&self, data: &T, widget: W) {
    // pub fn add_view(&mut self, widget: impl Fn() -> Box<dyn Widget<T> + 'static>) {
    pub fn add_view(&mut self, view: View) {
        let new_view = (view.ui_builder)();
        let widget = WidgetPod::new(new_view);
        self.views.push(widget);
    }
}
pub trait ViewController {
    // fn add_view<W: Data>(&mut self, widget: impl Fn() -> Box<dyn Widget<W> + 'static>);
    fn add_view(&mut self, view: AppState);

    fn current_view(&self) -> &AppState;
    fn current_view_mut(&self) -> &mut AppState;
    fn build_view(&self) -> WidgetPod<AppState, Box<dyn Widget<AppState>>>;
}

pub type ViewState = Arc<Vec<View>>;

impl<V: ViewController + Data> Widget<V> for Navigator {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut V, env: &Env) {
        // self.current_view.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &V, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            // ctx.children_changed();
            dbg!("widget added");
        }
        // self.views
        //     .last_mut()
        //     .unwrap()
        //     .lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &V, data: &V, env: &Env) {
        if !old_data.same(data) {
            dbg!("data changed");
        }
        dbg!("data is same");
        // self.views.last_mut().unwrap().update(ctx, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &V, env: &Env) -> Size {
        Size::new(0.0, 0.0)
        // self.views.last_mut().unwrap().layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &V, env: &Env) {
        dbg!("painting");
        // todo!()
        // self.views.last_mut().unwrap().paint(ctx, data, env)
    }
}
