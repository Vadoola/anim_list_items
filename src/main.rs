use std::rc::Rc;

use slint::{ModelExt, ModelRc, Timer, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let model = Rc::new(VecModel::from(vec![
        mod_str {
            text: "Text 1".into(),
            hidden: true,
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

    let filt_ui = ui.as_weak();
    let filt_mod = ModelRc::from(model.clone()).filter(move |fmod| !(filt_ui.unwrap().get_hide() && fmod.hidden));
    let filt_mod = Rc::new(filt_mod);
    ui.set_list_mod(ModelRc::from(filt_mod.clone()));

    let timer = Timer::default();
    ui.on_up_mod(move || {
        let filt_mod = filt_mod.clone();
        
        timer.start(
            slint::TimerMode::SingleShot,
            std::time::Duration::from_millis(1000),
            move || {
                filt_mod.reset();

            },
        );
    });

    ui.run()
}
