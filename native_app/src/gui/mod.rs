use bulldozer::BulldozerResource;
use egregoria::Egregoria;
use inspected_aura::InspectedAura;
use movable::MovableSystem;
use roadbuild::RoadBuildResource;
use roadeditor::RoadEditorResource;

mod bulldozer;
mod follow;
mod inspect;
mod inspected_aura;
mod movable;
mod roadbuild;
mod roadeditor;
mod selectable;
mod topgui;
mod windows;

pub use follow::FollowEntity;

use common::inspect::InspectedEntity;
pub use inspect::*;
pub use topgui::*;

pub fn setup_gui(goria: &mut Egregoria) {
    goria
        .schedule
        .add_system(selectable::selectable_select_system())
        .add_system(selectable::selectable_cleanup_system())
        .add_system(roadbuild::roadbuild_system())
        .add_system(roadeditor::roadeditor_system())
        .add_system(bulldozer::bulldozer_system())
        .add_system(inspected_aura::inspected_aura_system(InspectedAura::new(
            &mut goria.world,
        )))
        .add_system(movable::movable_system(MovableSystem::default()));

    goria.insert(InspectedEntity::default());
    goria.insert(FollowEntity::default());
    goria.insert(Tool::default());

    let s = RoadBuildResource::new(&mut goria.world);
    goria.insert(s);

    let s = RoadEditorResource::new(&mut goria.world);
    goria.insert(s);

    let s = BulldozerResource::new(&mut goria.world);
    goria.insert(s);
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tool {
    Hand,
    RoadbuildStraight,
    RoadbuildCurved,
    RoadEditor,
    Bulldozer,
}

const Z_TOOL: f32 = 0.9;

impl Default for Tool {
    fn default() -> Self {
        Tool::Hand
    }
}