use crux_core::{
    macros::effect,
    render::{render, RenderOperation},
    App, Command,
};
use facet::Facet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Facet)]
#[repr(C)]
pub enum ComponentEvent {
    Increment,
    Decrement,
    Reset,
}

#[effect(facet_typegen)]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComponentModel {
    count: isize,
}

#[derive(Clone, Debug, Serialize, Deserialize, Facet)]
pub struct ComponentViewModel {
    pub count: String,
}

#[derive(Default)]
pub struct Component;

impl App for Component {
    type Event = ComponentEvent;
    type Model = ComponentModel;
    type ViewModel = ComponentViewModel;
    type Capabilities = (); // will be deprecated, so use unit type for now
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &(), // will be deprecated, so prefix with underscore for now
    ) -> Command<Effect, ComponentEvent> {
        match event {
            ComponentEvent::Increment => model.count += 1,
            ComponentEvent::Decrement => model.count -= 1,
            ComponentEvent::Reset => model.count = 0,
        }

        render()
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ComponentViewModel {
            count: format!("Count is: {}", model.count),
        }
    }
}
