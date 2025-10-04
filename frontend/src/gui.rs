use bevy::prelude::*;
use bevy_egui::*;

pub fn gui_plugin(app: &mut App) {
    app
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, ui_menu_bar)
        ;
}

fn ui_menu_bar(
    mut contexts: EguiContexts,
    mut rom_event: MessageWriter<crate::ch8_plugin::LoadRomMessage>,
) {
    egui::TopBottomPanel::top("menu_bar").show(contexts.ctx_mut().unwrap(), |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("Emulator", |ui| {
                if ui.button("Load ROM").clicked() {
                    let current_dir = std::env::current_dir().unwrap();
                    let res = rfd::FileDialog::new()
                        .set_directory(&current_dir)
                        .pick_files();

                    if let Some(v) = res {
                        println!("selected: {v:?}");
                        rom_event.write(crate::ch8_plugin::LoadRomMessage(v[0].clone()));
                    }
                }
            });
        });
    });
}
