use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

const APP_ID: &str = "supply.same.Slabs";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.auto_exclusive_zone_enable();

    for anchor in [Edge::Left, Edge::Right, Edge::Top] {
        window.set_anchor(anchor, true);
    }

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
