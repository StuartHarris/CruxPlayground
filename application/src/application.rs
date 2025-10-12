use component::{ComponentEvent, ComponentModel, ComponentViewModel};
use crux_core::{macros::effect, render::RenderOperation, App, Command};
use facet::Facet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Facet)]
#[repr(C)]
pub enum Event {
    Component(#[facet(namespace = "component_crux")] ComponentEvent),
}

#[effect(facet_typegen)]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    component: ComponentModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, Facet)]
pub struct ViewModel {
    #[facet(namespace = "component_crux")]
    component: ComponentViewModel,
    pub title: String,
}

#[derive(Default)]
pub struct Application {
    component: component::Component,
}

impl App for Application {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = ();
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &(),
    ) -> Command<Effect, Event> {
        match event {
            Event::Component(event) => self
                .component
                .update(event, &mut model.component, &())
                .map_effect(|effect| match effect {
                    component::Effect::Render(request) => Effect::Render(request),
                })
                .map_event(Event::Component),
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            component: self.component.view(&model.component),
            title: "Application".to_string(),
        }
    }
}
