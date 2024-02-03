use std::sync::atomic::Ordering;
use std::time::Instant;

use yakui::widgets::{List, Pad};
use yakui::{column, reflow, spacer, Alignment, CrossAxisAlignment, Dim2, MainAxisSize};

use goryak::{
    blur_bg, button_primary, button_secondary, constrained_viewport, on_primary_container,
    on_secondary_container, padxy, secondary_container, textc, Window,
};
use simulation::economy::Government;
use simulation::Simulation;

use crate::gui::{ExitState, Gui};
use crate::inputmap::{InputAction, InputMap};
use crate::uiworld::{SaveLoadState, UiWorld};

pub fn menu_bar(gui: &mut Gui, uiworld: &UiWorld, sim: &Simulation) {
    profiling::scope!("hud::menu_bar");

    reflow(Alignment::TOP_LEFT, Dim2::ZERO, || {
        constrained_viewport(|| {
            column(|| {
                blur_bg(secondary_container().with_alpha(0.5), 0.0, || {
                    padxy(5.0, 5.0, || {
                        let mut l = List::row();
                        l.item_spacing = 10.0;
                        l.cross_axis_alignment = CrossAxisAlignment::Center;

                        l.show(|| {
                            gui.windows.menu();
                            save_window(gui, uiworld);
                            textc(
                                on_primary_container(),
                                format!("Money: {}", sim.read::<Government>().money),
                            );
                        });
                    });
                });
                spacer(1);
            });
        });
    });
}

fn save_window(gui: &mut Gui, uiw: &UiWorld) {
    let mut slstate = uiw.write::<SaveLoadState>();
    if slstate.saving_status.load(Ordering::SeqCst) {
        textc(on_secondary_container(), "Saving...");
    } else if button_primary("Save").show().clicked {
        slstate.please_save = true;
        gui.last_save = Instant::now();
        uiw.save_to_disk();
    }

    let mut estate = uiw.write::<ExitState>();

    match *estate {
        ExitState::NoExit => {}
        ExitState::ExitAsk | ExitState::Saving => {
            uiw.window(
                Window {
                    title: "Exit Menu",
                    pad: Pad::all(15.0),
                    radius: 10.0,
                },
                |uiw| {
                    let mut estate = uiw.write::<ExitState>();
                    *estate = ExitState::NoExit;
                },
                |_, uiw, _sim| {
                    let mut slstate = uiw.write::<SaveLoadState>();
                    let mut estate = uiw.write::<ExitState>();
                    let mut l = List::column();
                    l.item_spacing = 5.0;
                    l.main_axis_size = MainAxisSize::Min;
                    l.show(|| {
                        if let ExitState::Saving = *estate {
                            textc(on_secondary_container(), "Saving...");
                            if !slstate.please_save && !slstate.saving_status.load(Ordering::SeqCst)
                            {
                                std::process::exit(0);
                            }
                            return;
                        }
                        if button_secondary("Save and exit").show().clicked {
                            if let ExitState::ExitAsk = *estate {
                                slstate.please_save = true;
                                *estate = ExitState::Saving;
                            }
                        }
                        if button_secondary("Exit without saving").show().clicked {
                            std::process::exit(0);
                        }
                        if button_secondary("Cancel").show().clicked {
                            *estate = ExitState::NoExit;
                        }
                    });
                },
            );

            if uiw
                .read::<InputMap>()
                .just_act
                .contains(&InputAction::Close)
            {
                *estate = ExitState::NoExit;
            }
        }
    }

    match *estate {
        ExitState::NoExit => {
            if button_secondary("Exit").show().clicked {
                *estate = ExitState::ExitAsk;
            }
        }
        ExitState::ExitAsk => {
            if button_secondary("Save and exit").show().clicked {
                if let ExitState::ExitAsk = *estate {
                    slstate.please_save = true;
                    *estate = ExitState::Saving;
                }
            }
        }
        ExitState::Saving => {
            textc(on_secondary_container(), "Saving...");
        }
    }
}
