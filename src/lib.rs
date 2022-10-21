use druid::{Widget, widget::Label, Data, WidgetExt};

#[derive(Data, Clone)]
struct CardedData {
    amount: i32,
}

pub fn build_ui() -> impl Widget<()> {
    Label::new("Hello Carded World!!").center()
}