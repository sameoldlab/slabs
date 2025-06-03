use gtk::{prelude::*, Orientation};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::{RelmApp, SimpleComponent};

const APP_ID: &str = "supply.same.Slabs";

#[derive(Debug)]
enum Messages {}
struct Model {
    workspace: String,
    window: String,
    time: String,
}

#[relm4::component]
impl SimpleComponent for Model {
    type Init = u8;
    type Input = Messages;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            init_layer_shell: (),
            set_layer: Layer::Top,
            auto_exclusive_zone_enable: (),
            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Right, true),
            set_anchor: (Edge::Top, true),

            gtk::Box {
                set_orientation: Orientation::Horizontal,
                gtk::Label {
                    #[watch]
                    set_label: &model.workspace
                },
                gtk::Label {
                    #[watch]
                    set_label: &model.window
                },
                gtk::Label {
                    #[watch]
                    set_label: &model.time
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Model {
            workspace: "".to_string(),
            window: "".to_string(),
            time: "".to_string(),
        };
        let widgets = view_output!();
        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {}
    }
}

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<Model>(0);
}
