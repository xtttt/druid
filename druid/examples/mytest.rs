use druid::widget::prelude::*;
use druid::widget::{Flex, Label, TextBox};
use druid::{
    theme, AppLauncher, Command, Data, Lens, Notification, Point, Rect, Selector, UnitPoint,
    WidgetExt, WidgetPod, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct HelloState {
    show_popup: bool,
}

struct MyMenu<T> {
    show: Box<dyn Fn(&T, &Env) -> bool>,
    popup: Box<dyn Fn(&T, &Env) -> Box<dyn Widget<T>>>,
    handle: Option<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T> MyMenu<T> {
    fn new(
        show: Box<dyn Fn(&T, &Env) -> bool>,
        popup: Box<dyn Fn(&T, &Env) -> Box<dyn Widget<T>>>,
    ) -> Self {
        MyMenu {
            show,
            popup,
            handle: None,
        }
    }
}

impl<T: Data> Widget<T> for MyMenu<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Some(child) = self.handle.as_mut() {
            child.event(ctx, event, data, env)
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let Some(child) = self.handle.as_mut() {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !(self.show)(old_data, env) && (self.show)(data, env) {
            self.handle = Some(WidgetPod::new((self.popup)(data, env)));
            ctx.children_changed();
        } else if (self.show)(old_data, env) && !(self.show)(data, env) {
            self.handle = None;
            ctx.children_changed();
        } else if let Some(child) = self.handle.as_mut() {
            child.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        match self.handle {
            Some(ref mut child) => {
                let size = child.layout(ctx, bc, data, env);
                child.set_origin(ctx, data, env, Point::ORIGIN);
                size
            }
            None => Size::ZERO,
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Some(ref mut child) = self.handle {
            child.paint(ctx, data, env);
        }
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state: HelloState = HelloState { show_popup: false };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    Flex::column()
        .with_child(
            Flex::column()
                .with_child(
                    Label::new("text1").on_click(|_ctx, data: &mut HelloState, _env| {
                        data.show_popup = !data.show_popup
                    }),
                )
                .with_child(MyMenu::new(
                    Box::new(|data: &HelloState, _| data.show_popup),
                    Box::new(|_, _| Box::new(Label::new("item"))),
                )),
        )
        .with_child(
            Label::new("text2")
                .on_click(|_ctx, data: &mut HelloState, _env| data.show_popup = !data.show_popup),
        )
        .with_child(
            Label::new("text3")
                .on_click(|_ctx, data: &mut HelloState, _env| data.show_popup = !data.show_popup),
        )
        .with_child(
            Label::new("text4")
                .on_click(|_ctx, data: &mut HelloState, _env| data.show_popup = !data.show_popup),
        )
}
