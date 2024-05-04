use yakui::widgets::List;
use yakui::{
    image, reflow, Alignment, Color, CrossAxisAlignment, Dim2, MainAxisAlignment, MainAxisSize, Vec2
};

use goryak::{image_button, mincolumn, padxy, primary};
use simulation::map::LanePatternBuilder;

use crate::newgui::hud::toolbox::updown_value;
use crate::newgui::roadbuild::{RoadBuildResource, Snapping};
use crate::newgui::textures::UiTextures;
use crate::uiworld::UiWorld;

pub fn roadbuild_properties(uiw: &UiWorld) {
    let mut state = uiw.write::<RoadBuildResource>();

    padxy(0.0, 10.0, || {
        let mut l = List::row();
        l.main_axis_alignment = MainAxisAlignment::Center;
        l.cross_axis_alignment = CrossAxisAlignment::Center;
        l.item_spacing = 10.0;
        l.show(|| {
            let c = primary().lerp(&Color::WHITE, 0.3);
            let active = (c, c.with_alpha(0.7));
            let default = (Color::WHITE.with_alpha(0.3), Color::WHITE.with_alpha(0.5));

            mincolumn(0.0, || {
                let (snapping_none, snapping_grid, snapping_angel) = match state.snapping {
                    Snapping::None =>           {(active, default, default)},
                    Snapping::SnapToGrid =>     {(default, active, default)},
                    Snapping::SnapToAngle =>    {(default, default, active)},
                };

                if image_button(
                    uiw.read::<UiTextures>().get("snap_notting"),
                    Vec2::new(25.0, 25.0),
                    snapping_none.0,
                    snapping_none.1,
                    primary(),
                    "no snapping",
                ).clicked { state.snapping = Snapping::None; }

                if image_button(
                    uiw.read::<UiTextures>().get("snap_grid"),
                    Vec2::new(25.0, 25.0),
                    snapping_grid.0,
                    snapping_grid.1,
                    primary(),
                    "snap to grid",
                )
                .clicked { state.snapping = Snapping::SnapToGrid; }
    
                if image_button(
                    uiw.read::<UiTextures>().get("snap_angle"),
                    Vec2::new(25.0, 25.0),
                    snapping_angel.0,
                    snapping_angel.1,
                    primary(),
                    "snap to angle",
                )
                .clicked { state.snapping = Snapping::SnapToAngle; }
            });

            // Road elevation
            updown_value(&mut state.height_offset, 2.0, "m");

            // image name, label, builder
            let builders: &[(&str, &str, LanePatternBuilder)] = &[
                ("roadtypes_street", "Street", LanePatternBuilder::new()),
                (
                    "roadtypes_street_1way",
                    "Street one-way",
                    LanePatternBuilder::new().one_way(true),
                ),
                (
                    "roadtypes_avenue",
                    "Avenue",
                    LanePatternBuilder::new().n_lanes(2).speed_limit(13.0),
                ),
                (
                    "roadtypes_avenue_1way",
                    "Avenue one-way",
                    LanePatternBuilder::new()
                        .n_lanes(2)
                        .one_way(true)
                        .speed_limit(13.0),
                ),
                (
                    "roadtypes_drive",
                    "Drive",
                    LanePatternBuilder::new()
                        .parking(false)
                        .sidewalks(false)
                        .speed_limit(13.0),
                ),
                (
                    "roadtypes_drive_1way",
                    "Drive one-way",
                    LanePatternBuilder::new()
                        .parking(false)
                        .sidewalks(false)
                        .one_way(true)
                        .speed_limit(13.0),
                ),
                (
                    "roadtypes_highway",
                    "Highway",
                    LanePatternBuilder::new()
                        .n_lanes(3)
                        .speed_limit(25.0)
                        .parking(false)
                        .sidewalks(false),
                ),
                (
                    "roadtypes_highway_1way",
                    "Highway one-way",
                    LanePatternBuilder::new()
                        .n_lanes(3)
                        .speed_limit(25.0)
                        .parking(false)
                        .sidewalks(false)
                        .one_way(true),
                ),
                (
                    "roadtypes_rail",
                    "Rail",
                    LanePatternBuilder::new().rail(true),
                ),
                (
                    "roadtypes_rail_1way",
                    "Rail one-way",
                    LanePatternBuilder::new().rail(true).one_way(true),
                ),
            ];

            for (icon, label, builder) in builders {
                let mut l = List::column();
                l.main_axis_size = MainAxisSize::Min;
                l.show(|| {
                    let is_active = &state.pattern_builder == builder;
                    let (default_col, hover_col) = if is_active {
                        let c = Color::WHITE.adjust(0.5);
                        (c, c)
                    } else {
                        (Color::WHITE, Color::WHITE.with_alpha(0.7))
                    };
                    if image_button(
                        uiw.read::<UiTextures>().get(icon),
                        Vec2::new(64.0, 64.0),
                        default_col,
                        hover_col,
                        primary(),
                        *label,
                    )
                    .clicked
                    {
                        state.pattern_builder = *builder;
                    }

                    if is_active {
                        reflow(Alignment::CENTER_LEFT, Dim2::pixels(0.0, 32.0), || {
                            image(
                                uiw.read::<UiTextures>().get("select_triangle_under"),
                                Vec2::new(64.0, 10.0),
                            );
                        });
                    }
                });
            }
        });
    });
}
