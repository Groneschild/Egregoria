use yakui::widgets::{List, Pad, StateResponse, TextBox};
use yakui::{
    align, center, colored_box_container, column, constrained, pad, row, use_state, Alignment,
    Constraints, CrossAxisAlignment, MainAxisAlignment, MainAxisSize, Response, Vec2,
};

use common::descriptions::{BuildingGen, CompanyKind};
use engine::meshload::MeshProperties;
use engine::wgpu::RenderPass;
use engine::{set_cursor_icon, CursorIcon, Drawable, GfxContext, Mesh, SpriteBatch};
use geom::Matrix4;
use goryak::{
    background, button_primary, button_secondary, center_width, checkbox_value, combo_box,
    debug_constraints, debug_size, dragvalue, is_hovered, labelc, on_background, on_secondary,
    outline_variant, scroll_vertical, secondary, secondary_container, set_theme, stretch_width,
    use_changed, CountGrid, Draggable, MainAxisAlignItems, RoundRect, Theme,
};

use crate::companies::Companies;
use crate::State;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Inspected {
    None,
    Company(usize),
}

#[derive(Clone)]
pub enum Shown {
    None,
    Error(String),
    Model((Mesh, MeshProperties)),
    Sprite(SpriteBatch),
}

pub struct Gui {
    pub companies: Companies,
    pub inspected: Inspected,
    pub shown: Shown,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            companies: Companies::new().expect("could not load companies.json"),
            inspected: Inspected::None,
            shown: Shown::None,
        }
    }
}

impl State {
    pub fn gui_yakui(&mut self) {
        row(|| {
            self.explorer();
            self.model_properties();
            self.properties();
        });
    }

    fn explorer(&mut self) {
        let mut off = use_state(|| 300.0);
        constrained(
            Constraints::loose(Vec2::new(off.get(), f32::INFINITY)),
            || {
                colored_box_container(background(), || {
                    let mut l = List::column();
                    l.cross_axis_alignment = CrossAxisAlignment::Stretch;
                    l.show(|| {
                        let mut l = List::row();
                        l.item_spacing = 5.0;
                        l.main_axis_alignment = MainAxisAlignment::Center;
                        Pad::all(5.0).show(|| {
                            l.show(|| {
                                if button_primary("Dark theme").clicked {
                                    set_theme(Theme::Dark);
                                }
                                if button_primary("Light theme").clicked {
                                    set_theme(Theme::Light);
                                }
                            });
                        });
                        scroll_vertical(|| {
                            let mut l = List::column();
                            l.cross_axis_alignment = CrossAxisAlignment::Stretch;
                            l.show(|| {
                                let companies_open = use_state(|| false);
                                Self::explore_item(0, "Companies".to_string(), || {
                                    companies_open.modify(|x| !x);
                                });
                                if self.gui.companies.changed && button_primary("Save").clicked {
                                    self.gui.companies.save();
                                }
                                if companies_open.get() {
                                    for (i, comp) in self.gui.companies.companies.iter().enumerate()
                                    {
                                        Self::explore_item(4, comp.name.to_string(), || {
                                            self.gui.inspected = Inspected::Company(i);
                                        });
                                    }
                                }
                            });
                        });
                    });
                });
            },
        );
        resizebar_vert(&mut off, false);
    }

    fn explore_item(indent: usize, name: String, on_click: impl FnOnce()) {
        let mut p = Pad::ZERO;
        p.left = indent as f32 * 4.0;
        p.top = 4.0;
        p.show(|| {
            if button_secondary(name).clicked {
                on_click();
            }
        });
    }

    fn model_properties(&mut self) {
        let mut l = List::column();
        l.main_axis_alignment = MainAxisAlignment::End;
        l.cross_axis_alignment = CrossAxisAlignment::Stretch;
        l.show(|| {
            colored_box_container(background(), || {
                column(|| {
                    labelc(on_background(), "Model properties");
                    match &self.gui.shown {
                        Shown::None => {
                            labelc(on_background(), "No model selected");
                        }
                        Shown::Error(e) => {
                            labelc(on_background(), e.clone());
                        }
                        Shown::Model((_, props)) => {
                            row(|| {
                                column(|| {
                                    labelc(on_background(), "Vertices");
                                    labelc(on_background(), "Triangles");
                                    labelc(on_background(), "Materials");
                                    labelc(on_background(), "Textures");
                                    labelc(on_background(), "Draw calls");
                                });
                                column(|| {
                                    labelc(on_background(), format!("{}", props.n_vertices));
                                    labelc(on_background(), format!("{}", props.n_triangles));
                                    labelc(on_background(), format!("{}", props.n_materials));
                                    labelc(on_background(), format!("{}", props.n_textures));
                                    labelc(on_background(), format!("{}", props.n_draw_calls));
                                });
                            });
                        }
                        Shown::Sprite(_sprite) => {
                            labelc(on_background(), "Sprite");
                        }
                    }
                });
            });
        });
    }

    fn properties(&mut self) {
        match self.gui.inspected {
            Inspected::None => {}
            Inspected::Company(i) => {
                properties_container(|| {
                    let comp = &mut self.gui.companies.companies[i];

                    let label = |name: &str| {
                        pad(Pad::all(3.0), || {
                            labelc(on_background(), name.to_string());
                        });
                    };

                    fn dragv(v: &mut impl Draggable) {
                        Pad::all(5.0).show(|| {
                            stretch_width(|| {
                                dragvalue().show(v);
                            });
                        });
                    }

                    label("Name");
                    text_inp(&mut comp.name);

                    label("Kind");
                    let mut selected = match comp.kind {
                        CompanyKind::Store => 0,
                        CompanyKind::Factory { .. } => 1,
                        CompanyKind::Network => 2,
                    };

                    if combo_box(&mut selected, &["Store", "Factory", "Network"], 150.0) {
                        match selected {
                            0 => comp.kind = CompanyKind::Store,
                            1 => comp.kind = CompanyKind::Factory { n_trucks: 1 },
                            2 => comp.kind = CompanyKind::Network,
                            _ => unreachable!(),
                        }
                    }

                    label("Building generator");
                    let mut selected = match comp.bgen {
                        BuildingGen::House => unreachable!(),
                        BuildingGen::Farm => 0,
                        BuildingGen::CenteredDoor { .. } => 1,
                        BuildingGen::NoWalkway { .. } => 2,
                    };

                    if combo_box(
                        &mut selected,
                        &["Farm", "Centered door", "No walkway"],
                        150.0,
                    ) {
                        match selected {
                            0 => comp.bgen = BuildingGen::Farm,
                            1 => {
                                comp.bgen = BuildingGen::CenteredDoor {
                                    vertical_factor: 1.0,
                                }
                            }
                            2 => {
                                comp.bgen = BuildingGen::NoWalkway {
                                    door_pos: geom::Vec2::ZERO,
                                }
                            }
                            _ => unreachable!(),
                        }
                    }

                    label("Recipe");
                    label(" ");

                    let recipe = &mut comp.recipe;

                    label("complexity");
                    dragv(&mut recipe.complexity);

                    label("storage_multiplier");
                    dragv(&mut recipe.storage_multiplier);

                    label("consumption");
                    label(" ");

                    for (name, amount) in recipe.consumption.iter_mut() {
                        label(name);
                        dragv(amount);
                    }

                    label("production");
                    label(" ");
                    for (name, amount) in recipe.production.iter_mut() {
                        label(name);
                        dragv(amount);
                    }

                    label("n_workers");
                    dragv(&mut comp.n_workers);

                    label("size");
                    dragv(&mut comp.size);

                    label("asset_location");
                    text_inp(&mut comp.asset_location);

                    label("price");
                    dragv(&mut comp.price);

                    label("zone");
                    let mut v = comp.zone.is_some();
                    center_width(|| checkbox_value(&mut v));

                    if v != comp.zone.is_some() {
                        if v {
                            comp.zone = Some(Default::default());
                        } else {
                            comp.zone = None;
                        }
                    }

                    if let Some(ref mut z) = comp.zone {
                        label("floor");
                        text_inp(&mut z.floor);

                        label("filler");
                        text_inp(&mut z.filler);

                        label("price_per_area");
                        dragv(&mut z.price_per_area);
                    }
                });
            }
        }
    }
}

fn properties_container(children: impl FnOnce()) {
    let mut off = use_state(|| 350.0);
    resizebar_vert(&mut off, true);
    constrained(
        Constraints::loose(Vec2::new(off.get(), f32::INFINITY)),
        || {
            colored_box_container(background(), || {
                align(Alignment::TOP_CENTER, || {
                    Pad::balanced(5.0, 20.0).show(|| {
                        RoundRect::new(10.0)
                            .color(secondary_container())
                            .show_children(|| {
                                Pad::all(8.0).show(|| {
                                    CountGrid::col(2)
                                        .main_axis_size(MainAxisSize::Min)
                                        .main_axis_align_items(MainAxisAlignItems::Center)
                                        .show(children);
                                });
                            });
                    });
                });
            });
        },
    );
}

/// A horizontal resize bar.
pub fn resizebar_vert(off: &mut Response<StateResponse<f32>>, scrollbar_on_left_side: bool) {
    colored_box_container(outline_variant(), || {
        let last_val = use_state(|| None);
        let mut hovered = false;
        let d = yakui::draggable(|| {
            constrained(Constraints::tight(Vec2::new(5.0, f32::INFINITY)), || {
                hovered = *is_hovered();
            });
        })
        .dragging;
        let delta = d
            .map(|v| {
                let delta = v.current.x - last_val.get().unwrap_or(v.current.x);
                last_val.set(Some(v.current.x));
                delta
            })
            .unwrap_or_else(|| {
                last_val.set(None);
                0.0
            });
        off.modify(|v| {
            if scrollbar_on_left_side {
                v - delta
            } else {
                v + delta
            }
            .clamp(100.0, 600.0)
        });

        let should_show_mouse_icon = d.is_some() || hovered;
        use_changed(should_show_mouse_icon, || {
            set_colresize_icon(should_show_mouse_icon);
        });
    });
}

fn text_inp(v: &mut String) {
    center(|| {
        let mut t = TextBox::new(v.clone());
        t.fill = Some(secondary());
        t.style.color = on_secondary();
        if let Some(x) = t.show().into_inner().text {
            *v = x;
        }
    });
}

impl Drawable for Shown {
    fn draw<'a>(&'a self, gfx: &'a GfxContext, rp: &mut RenderPass<'a>) {
        match self {
            Shown::None | Shown::Error(_) => {}
            Shown::Model((mesh, _)) => mesh.draw(gfx, rp),
            Shown::Sprite(sprite) => sprite.draw(gfx, rp),
        }
    }

    fn draw_depth<'a>(
        &'a self,
        gfx: &'a GfxContext,
        rp: &mut RenderPass<'a>,
        shadow_cascade: Option<&Matrix4>,
    ) {
        match self {
            Shown::None | Shown::Error(_) => {}
            Shown::Model((mesh, _)) => mesh.draw_depth(gfx, rp, shadow_cascade),
            Shown::Sprite(sprite) => sprite.draw_depth(gfx, rp, shadow_cascade),
        }
    }
}

fn set_colresize_icon(enabled: bool) {
    if enabled {
        set_cursor_icon(CursorIcon::ColResize);
    } else {
        set_cursor_icon(CursorIcon::Default);
    }
}