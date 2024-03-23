use std::rc::Rc;

use slint::{ModelExt, ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let model = Rc::new(VecModel::from(vec![
        mod_str {
            text: "Text 1".into(),
            hidden: false,
        },
        mod_str {
            text: "Text 2".into(),
            hidden: true,
        },
        mod_str {
            text: "Text 3".into(),
            hidden: false,
        },
        mod_str {
            text: "Text 4".into(),
            hidden: false,
        },
    ]));
    ui.set_list_mod(ModelRc::from(model.clone()));

    let callb_handle = ui.as_weak();
    let omod = model.clone();
    ui.on_up_mod(move || {
        let callb_handle = callb_handle.unwrap();
        let hide = callb_handle.get_hide();
        let filt_mod = ModelRc::from(omod.clone()).filter(move |fmod| {
            !(hide && fmod.hidden)
        });
        callb_handle.set_list_mod(ModelRc::from(Rc::new(filt_mod)));
    });

    ui.run()
}
