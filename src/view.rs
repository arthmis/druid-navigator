use std::sync::Arc;

use druid::{
    lens,
    widget::{
        Button, Click, Container, Controller, ControllerHost, Flex, Image, Label, List, SizedBox,
        TextBox,
    },
    Color, Command, Env, Event, EventCtx, ImageBuf, Lens, LensExt, Selector, Target, Widget,
    WidgetExt,
};

use crate::{
    app_state_derived_lenses,
    navigator::{Navigator, ViewController},
    // AppState, AppStateLens, AppStateLensBuilder, Contact, View,
    AppState,
    Contact,
    View,
};

pub fn contacts() -> Box<dyn Widget<AppState>> {
    // let back_button = Button::new("Back").on_click(|event, data: &mut AppState, env| {});
    // let next_button: ControllerHost<Button<AppState>, Click<AppState>> = Button::new("Next")
    //     .on_click(|event: &mut EventCtx, data: &mut AppState, env: &Env| {
    //         data.nav_state.add_view(View {
    //             name: "Second".to_string(),
    //         });
    //     });
    // let new_lens: AppStateLens<
    //     app_state_derived_lenses::app_name,
    //     app_state_derived_lenses::nav_state,
    //     app_state_derived_lenses::contacts,
    // > = AppStateLensBuilder::new()
    //     .nav_state(AppState::nav_state)
    //     .contacts(AppState::contacts)
    //     .build();
    let list =
        List::new(|| {
            let image = Image::new(ImageBuf::empty());
            let name_text = Label::new(
                |(views, contact, _, _): &(Arc<Vec<View>>, Contact, Option<usize>, usize),
                 _env: &_| { format!("{}", contact.name) },
            )
            .with_text_color(Color::BLACK);
            let email_text = Label::new(
                |(views, contact, _, _): &(Arc<Vec<View>>, Contact, Option<usize>, usize),
                 _env: &_| { format!("{}", contact.email) },
            )
            .with_text_color(Color::BLACK);
            let details = Flex::column().with_child(name_text).with_child(email_text);
            let layout = Flex::row().with_child(image).with_child(details);
            let layout = layout.on_click(|event, data, env| {
                let new_views = Arc::make_mut(&mut data.0);
                new_views.push(View::new("contact_details".to_string()));
                data.0 = Arc::new(new_views.to_owned());
                data.2 = Some(data.3);
                event.submit_command(Command::new(TEST, data.3, Target::Auto));
            });

            layout.background(Color::GRAY).fix_size(300., 300.)
        });
    // .lens(AppState::contacts);
    // .lens(new_lens);
    let layout = Flex::row()
        .with_flex_child(list, 1.0)
        .must_fill_main_axis(true)
        .expand_width();

    Box::new(Container::new(layout).background(Color::WHITE))
}

pub fn contact_details() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|event, data: &mut AppState, env| {
        data.nav_state.pop_view();
    });

    let name = Label::new(|data: &AppState, _env: &Env| {
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
    let edit_button = Button::new("Edit").on_click(|_event, data: &mut AppState, _| {
        dbg!("clicking edit button");
        let views = Arc::make_mut(&mut data.nav_state);
        views.push(View::new("contact_edit".to_string()));
        data.nav_state = Arc::new(views.to_owned());
    });
    let layout = Flex::column()
        .with_child(back_button)
        .with_child(name)
        .with_child(email)
        .with_child(age)
        .with_child(favorite_food)
        .with_child(edit_button)
        .must_fill_main_axis(true)
        .expand_height();

    // let lens = lens::Identity.index(0).in_arc();
    // child.lens(lens.get(&contacts));
    Box::new(
        Container::new(layout)
            .background(Color::GRAY)
            .controller(DetailsController),
    )
}
pub fn contact_edit() -> Box<dyn Widget<AppState>> {
    let back_button = Button::new("Back").on_click(|event, data: &mut AppState, env| {
        data.nav_state.pop_view();
    });
    let name_textbox = TextBox::new().lens(AppState::app_name);
    // let email_textbox = TextBox::new().lens(AppState::app_name);
    // let age_textbox = TextBox::new().lens(AppState::app_name);
    // let favorite_food_textbox = TextBox::new().lens(AppState::app_name);
    let layout = Flex::column()
        .with_child(back_button)
        .with_child(name_textbox)
        .must_fill_main_axis(true)
        .expand_height();

    Box::new(
        Container::new(layout)
            .background(Color::WHITE)
            .controller(DetailsController),
    )
}
const TEST: Selector<usize> = Selector::new("TEST");
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
        match event {
            Event::Command(selector) if selector.is(TEST) => {
                let index = selector.get(TEST).unwrap().to_owned();
                dbg!(&index);
                // let contacts = data.contacts.clone();
                let contacts = data.clone();
                // let lens = lens::Identity.index(index).in_arc();
                // child.lens(lens.get(&contacts));
                // child.lens(ContactLens { index });
                // dbg!(event);
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

pub fn navigator() -> impl Widget<AppState> {
    Navigator::new("contacts".to_string(), Box::new(contacts))
        .with_view_builder("contact_details".to_string(), contact_details)
        .with_view_builder("contact_edit".to_string(), contact_edit)
    // .with_view_builder("Second".to_string(), || Box::new(contact_details()))
    // .background(Color::WHITE)
}
