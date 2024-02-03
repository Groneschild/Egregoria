use ordered_float::OrderedFloat;
use yakui::{reflow, Alignment, Color, Dim2, Vec2};

use simulation::map_dynamic::ElectricityFlow;
use simulation::Simulation;

use crate::gui::{Gui, UiTextures};
use crate::newgui::hud::menu::menu_bar;
use crate::newgui::hud::time_controls::time_controls;
use crate::newgui::hud::toolbox::new_toolbox;
use crate::newgui::windows::Windows;
use crate::uiworld::UiWorld;

mod menu;
mod time_controls;
mod toolbox;
pub mod windows;

impl Gui {
    /// Root GUI entrypoint
    pub fn render_newgui(&mut self, uiworld: &mut UiWorld, sim: &Simulation) {
        profiling::scope!("hud::render");
        self.auto_save(uiworld);

        if self.hidden {
            return;
        }

        yakui::column(|| {
            self.power_errors(uiworld, sim);
            new_toolbox(uiworld, sim);
            menu_bar(self, uiworld, sim);
            time_controls(self, uiworld, sim);
            Windows::finish(self, uiworld, sim);
        });
    }

    fn power_errors(&mut self, uiworld: &UiWorld, sim: &Simulation) {
        profiling::scope!("hud::power_errors");
        let map = sim.map();
        let flow = sim.read::<ElectricityFlow>();

        let no_power_img = uiworld.read::<UiTextures>().get_yakui("no_power");

        for network in map.electricity.networks() {
            if !flow.blackout(network.id) {
                continue;
            }

            let mut buildings_with_issues = Vec::with_capacity(network.buildings.len());

            for &building in &network.buildings {
                let Some(b) = map.get(building) else {
                    continue;
                };

                let center = b.obb.center();

                let pos = center.z(b.height
                    + 20.0
                    + 1.0 * f32::cos(uiworld.time_always() + center.mag() * 0.05));
                let (screenpos, depth) = uiworld.camera().project(pos);

                let size = 10000.0 / depth;

                buildings_with_issues.push((screenpos, size));
            }

            buildings_with_issues.sort_by_key(|x| OrderedFloat(x.1));

            for (screenpos, size) in buildings_with_issues {
                reflow(
                    Alignment::TOP_LEFT,
                    Dim2::pixels(screenpos.x - size * 0.5, screenpos.y - size * 0.5),
                    || {
                        let mut image =
                            yakui::widgets::Image::new(no_power_img, Vec2::new(size, size));
                        image.color = Color::WHITE.with_alpha(0.7);
                        image.show();
                    },
                );
            }
        }
    }
}
