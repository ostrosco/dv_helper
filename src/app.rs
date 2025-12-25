use crate::locomotive::{LOCO_LIST, Locomotive, LocomotiveInfo, locomotives};
use crate::order::{Order, OrderModal, OrderModalMode};
use crate::toggle_switch::toggle;
use egui_extras::{Column, TableBuilder};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ConsistManagerApp {
    add_loco_modal_open: bool,
    selected_loco: LocomotiveInfo,

    #[serde(skip)]
    new_order_modal: OrderModal,
    #[serde(skip)]
    edit_order_modal: OrderModal,

    locomotives: Vec<LocomotiveInfo>,
    pub orders: Vec<Order>,
    order_index: Option<usize>,

    total_weight: f32,
    total_length: f32,
    supported_weight_0_deg: u16,
    supported_weight_2_deg: u16,
    supported_weight_rain: u16,
}

impl Default for ConsistManagerApp {
    fn default() -> Self {
        Self {
            add_loco_modal_open: false,
            selected_loco: locomotives()
                .get(&Locomotive::DE2)
                .expect("Locomotive structure is totally borked")
                .clone(),

            new_order_modal: OrderModal::new(OrderModalMode::New),
            edit_order_modal: OrderModal::new(OrderModalMode::Edit),

            locomotives: Vec::new(),
            orders: Vec::new(),
            order_index: None,

            total_weight: 0.0,
            total_length: 0.0,
            supported_weight_0_deg: 0,
            supported_weight_2_deg: 0,
            supported_weight_rain: 0,
        }
    }
}

impl ConsistManagerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    // Recalculates the weight maximums of the current train consist.
    pub fn recalc_loco_limits(&mut self) {
        let weight_0_deg = self
            .locomotives
            .iter()
            .fold(0, |a, l| if l.powered { a + l.zero_grade_t } else { a });
        let weight_2_deg = self
            .locomotives
            .iter()
            .fold(0, |a, l| if l.powered { a + l.two_grade_t } else { a });
        let weight_rain = self
            .locomotives
            .iter()
            .fold(0, |a, l| if l.powered { a + l.rain_grade_t } else { a });
        self.supported_weight_0_deg = weight_0_deg;
        self.supported_weight_2_deg = weight_2_deg;
        self.supported_weight_rain = weight_rain;
    }

    // Recalculates the total weight and length of the current consist.
    pub fn recalc_consist(&mut self) {
        let loco_weight = self.locomotives.iter().fold(0.0, |a, l| a + l.weight);
        let order_weight = self.orders.iter().fold(0.0, |a, o| a + o.weight);
        self.total_weight = loco_weight + order_weight;
        let loco_length = self.locomotives.iter().fold(0.0, |a, l| a + l.length);
        let order_length = self.orders.iter().fold(0.0, |a, o| a + o.length);
        self.total_length = loco_length + order_length;
    }
}

impl eframe::App for ConsistManagerApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame::new()
            .inner_margin(egui::Margin::same(3))
            .fill(ctx.style().visuals.window_fill())
            .stroke(ctx.style().visuals.window_stroke());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Add Locomotive/Car").clicked() {
                    self.add_loco_modal_open = true;
                }
                ui.add_space(15.0);
                if ui.button("Add Order").clicked() {
                    self.new_order_modal.open = true;
                    self.order_index = None;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });

        egui::SidePanel::left("loco-menu")
            .frame(frame)
            .show(ctx, |ui| {
                let mut loco_to_delete = None;
                ui.vertical_centered(|ui| ui.heading("Current Locomotives"));
                ui.separator();
                for ix in 0..self.locomotives.len() {
                    ui.horizontal(|ui| {
                        let loco = self
                            .locomotives
                            .get_mut(ix)
                            .expect("Went out of bounds on locomotive list");
                        let response = ui.label(format!("- {}", loco.loco));
                        if loco.has_power {
                            let resp = ui.add(toggle(&mut loco.powered));
                            if resp.changed() {
                                self.recalc_loco_limits();
                            }
                        }
                        egui::Popup::context_menu(&response)
                            .id(egui::Id::new("loco_menu").with(ix))
                            .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                            .show(|ui| {
                                ui.set_min_width(200.0);
                                if ui.button("Delete locomotive").clicked() {
                                    loco_to_delete = Some(ix);
                                }
                            });
                    });
                }
                if let Some(loco) = loco_to_delete {
                    self.locomotives.remove(loco);
                    self.recalc_loco_limits();
                    self.recalc_consist();
                }
            });

        egui::SidePanel::right("status-menu")
            .frame(frame)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Consist Info"));
                ui.separator();
                ui.label(format!("- Total Weight: {:.2} T", self.total_weight));
                ui.label("- Supported Weights:");
                ui.label(format!("  - 0% grade: {} T", self.supported_weight_0_deg));
                ui.label(format!("  - 2% grade: {} T", self.supported_weight_2_deg));
                ui.label(format!(
                    "  - 2% grade in rain: {} T",
                    self.supported_weight_rain
                ));
                ui.separator();
                ui.label(format!("- Total Length: {:.2}m", self.total_length));
            });

        if self.add_loco_modal_open {
            let modal = egui::Modal::new("Add Locomotive/Car".into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Add Locomotive/Car");
                egui::ComboBox::from_label("Locomotive/Car")
                    .selected_text(self.selected_loco.loco.to_string())
                    .show_ui(ui, |ui| {
                        for l in LOCO_LIST {
                            let loco_str = l.to_string();
                            ui.selectable_value(
                                &mut self.selected_loco,
                                locomotives()
                                    .get(&l)
                                    .expect("Unknown locomotive/car")
                                    .clone(),
                                loco_str,
                            );
                        }
                    });
                ui.separator();
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Add").clicked() {
                            self.locomotives.push(self.selected_loco.clone());
                            self.recalc_consist();
                            self.recalc_loco_limits();
                            ui.close();
                        }
                        if ui.button("Cancel").clicked() {
                            ui.close();
                        }
                    },
                );
            });

            if modal.should_close() {
                self.add_loco_modal_open = false;
            }
        }

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("Orders"));
            ui.separator();
            TableBuilder::new(ui)
                .striped(true)
                .sense(egui::Sense::click())
                .columns(Column::auto().resizable(false), 7)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Order Name").strong());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Weight").strong());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Length").strong());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Pickup Station").strong());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Pickup Track").strong());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Dropoff Station").strong());
                    });
                    header.col(|ui| {
                        ui.label(egui::RichText::new("Dropoff Track").strong());
                    });
                })
                .body(|body| {
                    let mut order_to_delete = None;

                    body.rows(30.0, self.orders.len(), |mut row| {
                        let order = self.orders.get(row.index()).expect("Indexing woes").clone();
                        row.set_overline(true);
                        row.col(|ui| {
                            ui.label(&order.name);
                        });
                        row.col(|ui| {
                            ui.label(order.weight.to_string());
                        });
                        row.col(|ui| {
                            ui.label(order.length.to_string());
                        });
                        row.col(|ui| {
                            ui.label(order.pickup_station.to_abbrev());
                        });
                        row.col(|ui| {
                            ui.label(&order.pickup_track);
                        });
                        row.col(|ui| {
                            ui.label(order.dropoff_station.to_abbrev());
                        });
                        row.col(|ui| {
                            ui.label(&order.dropoff_track);
                        });

                        egui::Popup::context_menu(&row.response())
                            .id(egui::Id::new("order_menu").with(row.index()))
                            .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                            .show(|ui| {
                                let row_ix = row.index();
                                ui.set_min_width(200.0);
                                ui.menu_button("Add order...", |ui| {
                                    if ui.button("Above").clicked() {
                                        self.new_order_modal.open = true;
                                        if row_ix == 0 {
                                            self.order_index = Some(0);
                                        } else {
                                            self.order_index = Some(row.index() - 1);
                                        }
                                    } else if ui.button("Below").clicked() {
                                        self.new_order_modal.open = true;
                                        if row_ix == self.orders.len() {
                                            self.order_index = None;
                                        } else {
                                            self.order_index = Some(row.index() + 1);
                                        }
                                    }
                                });
                                ui.menu_button("Move order...", |ui| {
                                    if ui.button("Up").clicked() {
                                        if row_ix > 0 {
                                            self.orders.swap(row_ix, row_ix - 1);
                                        }
                                    } else if ui.button("Down").clicked()
                                        && row_ix < self.orders.len() - 1
                                    {
                                        self.orders.swap(row_ix, row_ix + 1);
                                    }
                                });
                                if ui.button("Edit order").clicked() {
                                    self.edit_order_modal.init_from_order(&order, row.index());
                                    self.edit_order_modal.open = true;
                                } else if ui.button("Delete order").clicked() {
                                    order_to_delete = Some(row.index());
                                }
                            });
                    });
                    if let Some(index) = order_to_delete {
                        self.orders.remove(index);
                        self.recalc_consist();
                    }
                });
        });

        self.new_order_modal.show(ctx);
        if let Some(order) = &self.new_order_modal.order {
            match self.order_index {
                Some(ix) => self.orders.insert(ix, order.clone()),
                None => self.orders.push(order.clone()),
            }
            self.recalc_consist();
            self.new_order_modal.order = None;
        }

        self.edit_order_modal.show(ctx);
        if let Some(edited_order) = &self.edit_order_modal.order {
            let ix = self.edit_order_modal.index;
            if let Some(order) = self.orders.get_mut(ix) {
                *order = edited_order.clone();
                self.recalc_consist();
                self.edit_order_modal.order = None;
            }
        }
    }
}
