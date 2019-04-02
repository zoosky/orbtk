use std::rc::Rc;

use dces::prelude::Entity;

use crate::event::EventHandler;

use crate::properties::*;

pub use self::build_context::*;
pub use self::context::*;
pub use self::message::*;
pub use self::state::*;
pub use self::template::*;
pub use self::widget_container::*;

mod build_context;
mod context;
mod message;
mod state;
mod template;
mod widget_container;

/// Adds the given `pseudo_class` to the css selector of the given `widget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Some(selector) = widget.try_get_mut::<Selector>() {
        selector.0.pseudo_classes.insert(String::from(pseudo_class));
        selector.0.set_dirty(true);
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `widget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Some(selector) = widget.try_get_mut::<Selector>() {
        selector.0.pseudo_classes.remove(pseudo_class);
        selector.0.set_dirty(true);
    }
}

/// The `Widget` trait is used to define a new widget.
pub trait Widget: Template {
    /// Creates a new widget.
    fn create() -> Self;

    /// Builds the widget and returns the template of the widget.
    fn build(self, context: &mut BuildContext) -> Entity;

    /// Inerts a new event handler.
    fn insert_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self;

    /// Returns the state of the widget.
    fn state(&self) -> Option<Rc<State>> {
        None
    }

    /// Appends a child ot the widget.
    fn child(self, child: Entity) -> Self;
}