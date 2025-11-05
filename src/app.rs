use egui_extras::{Column, TableBuilder};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Station {
    CitySouth,
    CityWest,
    CoalMineEast,
    CoalMineSouth,
    CoalPowerPlant,
    Farm,
    FoodFactory,
    ForestCentral,
    ForestSouth,
    GoodsFactory,
    Harbor,
    IronMineEast,
    IronMineWest,
    MachineFactory,
    MilitaryBase,
    OilRefinery,
    OilWellCentral,
    OilWellNorth,
    Sawmill,
    SteelMill,
}

impl Display for Station {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let station_str = match self {
            Self::CitySouth => "City South",
            Self::CityWest => "City West",
            Self::CoalMineEast => "Coal Mine East",
            Self::CoalMineSouth => "Coal Mine South",
            Self::CoalPowerPlant => "Coal Power Plant",
            Self::Farm => "Farm",
            Self::FoodFactory => "Food Factory & Town",
            Self::ForestCentral => "Forest Central",
            Self::ForestSouth => "Forest South",
            Self::GoodsFactory => "Goods Factory & Town",
            Self::Harbor => "Harbor & Town",
            Self::IronMineEast => "Iron Ore Mine East",
            Self::IronMineWest => "Iron Ore Mine West",
            Self::MachineFactory => "Machine Factory & Town",
            Self::MilitaryBase => "Military Base",
            Self::OilRefinery => "Oil Refinery",
            Self::OilWellCentral => "Oil Well Central",
            Self::OilWellNorth => "Oil Well North",
            Self::Sawmill => "Sawmill",
            Self::SteelMill => "Steel Mill",
        };
        write!(f, "{station_str}")
    }
}

impl Station {
    pub fn to_abbrev(self) -> String {
        match self {
            Self::CitySouth => "CS",
            Self::CityWest => "CW",
            Self::CoalMineEast => "CME",
            Self::CoalMineSouth => "CMS",
            Self::CoalPowerPlant => "CP",
            Self::Farm => "FM",
            Self::FoodFactory => "FF",
            Self::ForestCentral => "FRC",
            Self::ForestSouth => "FRS",
            Self::GoodsFactory => "GF",
            Self::Harbor => "HB",
            Self::IronMineEast => "IME",
            Self::IronMineWest => "IMW",
            Self::MachineFactory => "MF",
            Self::MilitaryBase => "MB",
            Self::OilRefinery => "OR",
            Self::OilWellCentral => "OWC",
            Self::OilWellNorth => "OWN",
            Self::Sawmill => "SW",
            Self::SteelMill => "SM",
        }
        .to_owned()
    }
}

const STATIONS: [Station; 20] = [
    Station::CitySouth,
    Station::CityWest,
    Station::CoalMineEast,
    Station::CoalMineSouth,
    Station::CoalPowerPlant,
    Station::Farm,
    Station::FoodFactory,
    Station::ForestCentral,
    Station::ForestSouth,
    Station::GoodsFactory,
    Station::Harbor,
    Station::IronMineEast,
    Station::IronMineWest,
    Station::MachineFactory,
    Station::MilitaryBase,
    Station::OilRefinery,
    Station::OilWellCentral,
    Station::OilWellNorth,
    Station::Sawmill,
    Station::SteelMill,
];

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Order {
    name: String,
    weight: f32,
    length: f32,
    pickup_station: Station,
    pickup_track: String,
    dropoff_station: Station,
    dropoff_track: String,
}

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LocomotiveInfo {
    loco: Locomotive,
    weight: f32,
    length: f32,
    zero_grade_t: u16,
    two_grade_t: u16,
    rain_grade_t: u16,
}

impl LocomotiveInfo {
    pub fn new(
        loco: Locomotive,
        weight: f32,
        length: f32,
        zero_grade_t: u16,
        two_grade_t: u16,
        rain_grade_t: u16,
    ) -> Self {
        let length = length / 1000.0;
        Self {
            loco,
            weight,
            length,
            zero_grade_t,
            two_grade_t,
            rain_grade_t,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub enum Locomotive {
    DE2,
    S060,
    DM3,
    DH4,
    S282,
    DE6,
}

impl Display for Locomotive {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let loco_str = match self {
            Self::DE2 => "DE2",
            Self::S060 => "S060",
            Self::DM3 => "DM3",
            Self::DH4 => "DH4",
            Self::S282 => "S282",
            Self::DE6 => "DE6",
        };
        write!(f, "{loco_str}")
    }
}

const LOCO_LIST: [Locomotive; 6] = [
    Locomotive::DE2,
    Locomotive::S060,
    Locomotive::DM3,
    Locomotive::DH4,
    Locomotive::S282,
    Locomotive::DE6,
];

fn locomotives() -> &'static HashMap<Locomotive, LocomotiveInfo> {
    static LOCOMOTIVES: OnceLock<HashMap<Locomotive, LocomotiveInfo>> = OnceLock::new();
    LOCOMOTIVES.get_or_init(|| {
        let mut l = HashMap::new();
        l.insert(
            Locomotive::DE2,
            LocomotiveInfo::new(Locomotive::DE2, 38.0, 7600.0, 1200, 300, 250),
        );
        l.insert(
            Locomotive::S060,
            LocomotiveInfo::new(Locomotive::S060, 50.7, 9320.0, 1500, 400, 300),
        );
        l.insert(
            Locomotive::DM3,
            LocomotiveInfo::new(Locomotive::DM3, 52.0, 8600.0, 2000, 500, 400),
        );
        l.insert(
            Locomotive::DH4,
            LocomotiveInfo::new(Locomotive::DH4, 77.5, 12840.0, 2000, 600, 500),
        );
        l.insert(
            Locomotive::S282,
            LocomotiveInfo::new(Locomotive::S282, 174.8, 22180.0, 3000, 1000, 800),
        );
        l.insert(
            Locomotive::DE6,
            LocomotiveInfo::new(Locomotive::DE6, 125.0, 18640.0, 3000, 1200, 1000),
        );
        l
    })
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ConsistManagerApp {
    add_loco_modal_open: bool,
    selected_loco: LocomotiveInfo,

    add_order_modal_open: bool,
    selected_order: String,
    selected_weight: String,
    selected_length: String,
    selected_pickup: Station,
    selected_pickup_track: String,
    selected_dropoff: Station,
    selected_dropoff_track: String,

    locomotives: Vec<LocomotiveInfo>,
    orders: Vec<Order>,

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

            add_order_modal_open: false,
            selected_order: String::new(),
            selected_weight: String::new(),
            selected_length: String::new(),
            selected_pickup: Station::SteelMill,
            selected_pickup_track: String::new(),
            selected_dropoff: Station::Harbor,
            selected_dropoff_track: String::new(),

            locomotives: Vec::new(),
            orders: Vec::new(),

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
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

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
        let weight_0_deg = self.locomotives.iter().fold(0, |a, l| a + l.zero_grade_t);
        let weight_2_deg = self.locomotives.iter().fold(0, |a, l| a + l.two_grade_t);
        let weight_rain = self.locomotives.iter().fold(0, |a, l| a + l.rain_grade_t);
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Add Locomotive").clicked() {
                    self.add_loco_modal_open = true;
                }
                ui.add_space(15.0);
                if ui.button("Add Order").clicked() {
                    self.add_order_modal_open = true;
                }
                ui.add_space(360.0);
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("loco-menu").show(ctx, |ui| {
            let mut loco_to_delete = None;
            ui.vertical_centered(|ui| ui.heading("Current Locomotives"));
            ui.separator();
            for ix in 0..self.locomotives.len() {
                let loco = self
                    .locomotives
                    .get(ix)
                    .expect("Went out of bounds on locomotive list");
                let response = ui.label(format!("- {}", loco.loco));
                egui::Popup::context_menu(&response)
                    .id(egui::Id::new("loco_menu").with(ix))
                    .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                    .show(|ui| {
                        ui.set_min_width(200.0);
                        if ui.button("Delete locomotive").clicked() {
                            loco_to_delete = Some(ix);
                        }
                    });
            }
            if let Some(loco) = loco_to_delete {
                self.locomotives.remove(loco);
                self.recalc_loco_limits();
                self.recalc_consist();
            }
        });

        egui::SidePanel::right("status-menu").show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("Consist Info"));
            ui.separator();
            ui.label(format!("- Total Weight: {} T", self.total_weight));
            ui.label("- Supported Weights:");
            ui.label(format!("  - 0% grade: {} T", self.supported_weight_0_deg));
            ui.label(format!("  - 2% grade: {} T", self.supported_weight_2_deg));
            ui.label(format!(
                "  - 2% grade in rain: {} T",
                self.supported_weight_rain
            ));
            ui.separator();
            ui.label(format!("- Total Length: {}m", self.total_length));
        });

        if self.add_loco_modal_open {
            let modal = egui::Modal::new("Add Locomotive".into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Add Locomotive");
                egui::ComboBox::from_label("Locomotive:")
                    .selected_text(self.selected_loco.loco.to_string())
                    .show_ui(ui, |ui| {
                        for l in LOCO_LIST {
                            let loco_str = l.to_string();
                            ui.selectable_value(
                                &mut self.selected_loco,
                                locomotives().get(&l).expect("Unknown locomotive").clone(),
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

        if self.add_order_modal_open {
            let modal = egui::Modal::new("Add Order".into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Add Order");
                ui.label("Order Name");
                ui.text_edit_singleline(&mut self.selected_order);
                ui.label("Weight");
                ui.text_edit_singleline(&mut self.selected_weight);
                ui.label("Length");
                ui.text_edit_singleline(&mut self.selected_length);
                ui.separator();
                egui::ComboBox::from_label("Pickup Station:")
                    .selected_text(self.selected_pickup.to_abbrev())
                    .show_ui(ui, |ui| {
                        for s in STATIONS {
                            let station_str = s.to_abbrev();
                            ui.selectable_value(&mut self.selected_pickup, s, station_str);
                        }
                    });
                ui.label("Pickup Track");
                ui.separator();
                ui.text_edit_singleline(&mut self.selected_pickup_track);
                egui::ComboBox::from_label("Dropoff Station:")
                    .selected_text(self.selected_dropoff.to_abbrev())
                    .show_ui(ui, |ui| {
                        for s in STATIONS {
                            let station_str = s.to_abbrev();
                            ui.selectable_value(&mut self.selected_dropoff, s, station_str);
                        }
                    });
                ui.label("Dropoff Track");
                ui.text_edit_singleline(&mut self.selected_dropoff_track);
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Add").clicked() {
                            let order = Order {
                                name: self.selected_order.clone(),
                                weight: str::parse(&self.selected_weight).expect("Invalid weight"),
                                length: str::parse(&self.selected_length).expect("Invalid length"),
                                pickup_station: self.selected_pickup,
                                pickup_track: self.selected_pickup_track.clone(),
                                dropoff_station: self.selected_dropoff,
                                dropoff_track: self.selected_dropoff_track.clone(),
                            };
                            self.selected_order = String::new();
                            self.selected_weight = String::new();
                            self.selected_length = String::new();
                            self.selected_pickup_track = String::new();
                            self.selected_dropoff_track = String::new();
                            self.orders.push(order);
                            self.recalc_consist();
                            ui.close();
                        }
                        if ui.button("Cancel").clicked() {
                            ui.close();
                        }
                    },
                );
            });
            if modal.should_close() {
                self.add_order_modal_open = false;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("Orders"));
            ui.separator();
            TableBuilder::new(ui)
                .striped(true)
                .sense(egui::Sense::click())
                .columns(Column::auto().resizable(false), 7)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Order Name");
                    });
                    header.col(|ui| {
                        ui.heading("Weight");
                    });
                    header.col(|ui| {
                        ui.heading("Length");
                    });
                    header.col(|ui| {
                        ui.heading("Pickup Station");
                    });
                    header.col(|ui| {
                        ui.heading("Pickup Track");
                    });
                    header.col(|ui| {
                        ui.heading("Dropoff Station");
                    });
                    header.col(|ui| {
                        ui.heading("Dropoff Track");
                    });
                })
                .body(|body| {
                    body.rows(30.0, self.orders.len(), |mut row| {
                        let order = &self
                            .orders
                            .get(row.index())
                            .expect("Indexing somehow doesn't work anymore");
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
                                ui.set_min_width(200.0);
                                if ui.button("Delete order").clicked() {
                                    self.orders.remove(row.index());
                                    self.recalc_consist();
                                }
                            });
                    });
                });
        });
    }
}
