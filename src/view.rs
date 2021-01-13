use std::{fmt::Debug, sync::Arc};

use druid::{
    lens,
    widget::{
        Button, Click, Container, Controller, ControllerHost, Flex, Image, Label, LensWrap, List,
        Painter, Scope, ScopePolicy, ScopeTransfer, SizedBox, TextBox,
    },
    Color, Command, Env, Event, EventCtx, ImageBuf, Lens, LensExt, RenderContext, Selector, Target,
    Widget, WidgetExt,
};

use crate::{
    app_state_derived_lenses,
    navigator::{Navigator, ViewController},
    AppState, Contact, View,
};

pub fn contacts() -> Box<dyn Widget<AppState>> {
    let list =
        List::new(|| {
            let image = Image::new(ImageBuf::empty());
            let name_text = Label::new(
                |(views, contact, _, _): &(Arc<Vec<View>>, Contact, Option<usize>, usize),
                 _env: &_| { format!("{}", contact.name) },
            )
            .with_text_color(Color::BLACK)
            .with_text_size(20.);
            let email_text = Label::new(
                |(views, contact, _, _): &(Arc<Vec<View>>, Contact, Option<usize>, usize),
                 _env: &_| { format!("{}", contact.email) },
            )
            .with_text_color(Color::BLACK)
            .with_text_size(20.);
            let details = Flex::column().with_child(name_text).with_child(email_text);
            let layout = Flex::row().with_child(image).with_child(details);
            let layout = layout.on_click(|event, data, env| {
                let new_views = Arc::make_mut(&mut data.0);
                new_views.push(View::new("contact_details".to_string()));
                data.0 = Arc::new(new_views.to_owned());
                data.2 = Some(data.3);
                event.submit_command(Command::new(CONTACT_DETAIL, data.3, Target::Auto));
            });

            layout.background(Painter::new(|ctx, data, env| {
                let is_hot = ctx.is_hot();
                let is_active = ctx.is_active();
                let rect = ctx.size().to_rect();
                let background_color = if is_active {
                    Color::rgb8(0x88, 0x88, 0x88)
                } else if is_hot {
                    Color::rgb8(0xdd, 0xdd, 0xdd)
                } else {
                    Color::WHITE
                };
                ctx.stroke(rect, &background_color, 0.);
                ctx.fill(rect, &background_color);
            }))
        });
    let layout = Flex::row()
        .with_flex_child(list.with_spacing(20.).center(), 1.)
        .must_fill_main_axis(true)
        .expand_width();

    Box::new(Container::new(layout).background(Color::WHITE))
}

pub fn contact_details() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|event, data: &mut AppState, env| {
        data.nav_state.pop_view();
    });

    let name = Label::dynamic(|data: &AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("{}", data.contacts[idx].name)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.0);
    let email = Label::new(|data: &AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("{}", data.contacts[idx].email)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.0);
    let age = Label::new(|data: &AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("{}", data.contacts[idx].age)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.0);
    let favorite_food = Label::new(|data: &AppState, _env: &Env| {
        if let Some(idx) = data.selected {
            format!("{}", data.contacts[idx].favorite_food)
        } else {
            "".to_string()
        }
    })
    .with_text_size(20.0);
    let edit_button = Button::new("Edit").on_click(|event, data: &mut AppState, _| {
        let views = Arc::make_mut(&mut data.nav_state);
        views.push(View::new("contact_edit".to_string()));
        data.nav_state = Arc::new(views.to_owned());
        event.submit_command(Command::new(
            CONTACT_EDIT,
            data.selected.unwrap(),
            Target::Auto,
        ));
    });
    let layout = Flex::column()
        .with_child(back_button)
        .with_child(name)
        .with_child(email)
        .with_child(age)
        .with_child(favorite_food)
        .with_child(edit_button)
        .must_fill_main_axis(true)
        // .center()
        .expand_height();
    let layout = Flex::row()
        .with_flex_child(layout.center(), 1.0)
        .must_fill_main_axis(true)
        .expand_width();

    Box::new(
        Container::new(layout)
            .background(Color::GRAY)
            .controller(DetailsController),
    )
}
pub struct EditLens {}
impl Lens<AppState, Contact> for EditLens {
    fn with<V, F: FnOnce(&Contact) -> V>(&self, data: &AppState, f: F) -> V {
        match data.selected {
            Some(index) => f(&data.contacts[index]),
            None => {
                let contact = Contact::new("".to_string(), "".to_string(), 0, "".to_string());
                f(&contact)
            }
        }
        // f(&data.contacts[data.selected.unwrap()])
    }

    fn with_mut<V, F: FnOnce(&mut Contact) -> V>(&self, data: &mut AppState, f: F) -> V {
        match data.selected {
            Some(index) => {
                let contacts = Arc::make_mut(&mut data.contacts);
                let something = f(&mut contacts[index]);
                // dbg!(&contacts);
                data.contacts = Arc::new(contacts.to_owned());
                // dbg!(&data.contacts);
                something
            }
            None => {
                let mut contact = Contact::new("".to_string(), "".to_string(), 0, "".to_string());
                f(&mut contact)
            }
        }
    }
}

pub fn contact_edit() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|event, data: &mut AppState, env| {
        data.nav_state.pop_view();
    });
    let name_textbox = TextBox::new();
    let email_textbox = TextBox::new();
    let age_textbox = TextBox::new();
    let favorite_food_textbox = TextBox::new();

    let layout = Flex::column()
        .with_child(name_textbox.lens(Contact::name))
        .with_child(email_textbox.lens(Contact::email))
        .with_child(age_textbox.lens(Contact::age.map(
            |age| age.to_string(),
            |age, age_string| *age = age_string.parse().unwrap(),
        )))
        .with_child(favorite_food_textbox.lens(Contact::favorite_food))
        .must_fill_main_axis(true)
        .lens(EditLens {})
        .center()
        .debug_paint_layout();
    let layout = Flex::column()
        .with_child(back_button)
        .with_flex_child(layout, 1.0)
        .expand_height();
    let container = Container::new(layout).background(Color::WHITE);
    // .controller(EditController);

    Box::new(container)
}
// pub struct ContactScopeTransfer {
//     contact: Contact,
// }

// impl ScopeTransfer for ContactScopeTransfer {
//     type In = AppState;

//     type State = Contact;

//     fn read_input(&self, state: &mut Self::State, inner: &Self::In) {
//         todo!()
//     }

//     fn write_back_input(&self, state: &Self::State, inner: &mut Self::In) {
//         todo!()
//     }
// }
// pub struct ContactScope {
//     contact: Contact,
// }
// impl ScopePolicy for ContactScope {
//     type In = AppState;
//     type Transfer = ContactScopeTransfer;

//     type State = Contact;

//     fn create(self, inner: &Self::In) -> (Self::State, Self::Transfer) {
//         let contact = inner.contacts[inner.selected.unwrap()];
//         (
//             contact,
//             ContactScopeTransfer {
//                 contact: contact.clone(),
//             },
//         )
//     }
// }
pub struct EditController;
impl Controller<AppState, Container<AppState>> for EditController {
    fn event(
        &mut self,
        child: &mut Container<AppState>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(selector) if selector.is(CONTACT_EDIT) => {
                let index = selector.get(CONTACT_EDIT).unwrap().to_owned();
                dbg!(&index);
                // let contacts = data.contacts.clone();
                // let contacts = data.clone();
                // let lens = lens::Identity.index(index).in_arc();
                // child.lens(lens.get(&contacts));
                // child.lens(ContactLens { index });
                // dbg!(event);
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}
const CONTACT_DETAIL: Selector<usize> = Selector::new("contact_detail");
const CONTACT_EDIT: Selector<usize> = Selector::new("contact_edit");
// pub struct ContactLens {
//     index: usize,
// }
// impl Lens<AppState, Contact> for ContactLens {
//     fn with<V, F: FnOnce(&Contact) -> V>(&self, data: &AppState, f: F) -> V {
//         f(&data.contacts[self.index])
//     }

//     fn with_mut<V, F: FnOnce(&mut Contact) -> V>(&self, data: &mut AppState, f: F) -> V {
//         f(&mut data.contacts[self.index])
//     }
// }
pub struct DetailsController;
impl Controller<AppState, Container<AppState>> for DetailsController {
    fn event(
        &mut self,
        child: &mut Container<AppState>,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        // match event {
        //     Event::Command(selector) if selector.is(CONTACT_DETAIL) => {
        //         let index = selector.get(CONTACT_DETAIL).unwrap().to_owned();
        //         dbg!(&index);
        //         // let contacts = data.contacts.clone();
        //         let contacts = data.clone();
        //         // let lens = lens::Identity.index(index).in_arc();
        //         // child.lens(lens.get(&contacts));
        //         // child.lens(ContactLens { index });
        //         // dbg!(event);
        //     }
        //     _ => {}
        // }
        child.event(ctx, event, data, env);
    }

    // fn lifecycle(
    //     &mut self,
    //     child: &mut Container<AppState>,
    //     ctx: &mut druid::LifeCycleCtx,
    //     event: &druid::LifeCycle,
    //     data: &AppState,
    //     env: &Env,
    // ) {
    //     child.lifecycle(ctx, event, data, env)
    // }

    fn update(
        &mut self,
        child: &mut Container<AppState>,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        dbg!(old_data, data);
        child.update(ctx, old_data, data, env);
        // ctx.request_paint();
    }
}

pub fn navigator() -> impl Widget<AppState> {
    Navigator::new("contacts".to_string(), Box::new(contacts))
        .with_view_builder("contact_details".to_string(), contact_details)
        .with_view_builder("contact_edit".to_string(), contact_edit)
    // .with_view_builder("Second".to_string(), || Box::new(contact_details()))
    // .background(Color::WHITE)
}
