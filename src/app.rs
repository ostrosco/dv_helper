use egui_extras::{Column, TableBuilder};
use std::collections::HashMap;
use std::sync::OnceLock;

/*
let stations = [
    (Station::CityWest, "CW"),
    (Station::CitySouth, "CS"),
    (Station::CoalMineEast, "CME"),
    (Station::CoalMineSouth, "CMS"),
    (Station::CoalPowerPlant, "CP"),
    (Station::Farm, "FM"),
    (Station::FoodFactory, "FF"),
    (Station::ForestCentral, "FRC"),
    (Station::ForestSouth, "FRS"),
    (Station::GoodsFactory, "GF"),
    (Station::Harbor, "HB"),
    (Station::IronMineEast, "IME"),
    (Station::IronMineWest, "IMW"),
    (Station::MachineFactory, "MF"),
    (Station::MilitaryBase, "MB"),
    (Station::OilRefinery, "OR"),
    (Station::OilWellCentral, "OWC"),
    (Station::OilWellNorth, "OWN"),
    (Station::Sawmill, "SW"),
    (Station::SteelMill, "SM"),
];
*/

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize)]
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

impl ToString for Station {
    fn to_string(&self) -> String {
        match self {
            Station::CitySouth => "City South",
            Station::CityWest => "City West",
            Station::CoalMineEast => "Coal Mine East",
            Station::CoalMineSouth => "Coal Mine South",
            Station::CoalPowerPlant => "Coal Power Plant",
            Station::Farm => "Farm",
            Station::FoodFactory => "Food Factory & Town",
            Station::ForestCentral => "Forest Central",
            Station::ForestSouth => "Forest South",
            Station::GoodsFactory => "Goods Factory & Town",
            Station::Harbor => "Harbor & Town",
            Station::IronMineEast => "Iron Ore Mine East",
            Station::IronMineWest => "Iron Ore Mine West",
            Station::MachineFactory => "Machine Factory & Town",
            Station::MilitaryBase => "Military Base",
            Station::OilRefinery => "Oil Refinery",
            Station::OilWellCentral => "Oil Well Central",
            Station::OilWellNorth => "Oil Well North",
            Station::Sawmill => "Sawmill",
            Station::SteelMill => "Steel Mill",
        }
        .to_string()
    }
}

impl Station {
    pub fn to_abbrev(&self) -> String {
        match self {
            Station::CitySouth => "CS",
            Station::CityWest => "CW",
            Station::CoalMineEast => "CME",
            Station::CoalMineSouth => "CMS",
            Station::CoalPowerPlant => "CP",
            Station::Farm => "FM",
            Station::FoodFactory => "FF",
            Station::ForestCentral => "FRC",
            Station::ForestSouth => "FRS",
            Station::GoodsFactory => "GF",
            Station::Harbor => "HB",
            Station::IronMineEast => "IME",
            Station::IronMineWest => "IMW",
            Station::MachineFactory => "MF",
            Station::MilitaryBase => "MB",
            Station::OilRefinery => "OR",
            Station::OilWellCentral => "OWC",
            Station::OilWellNorth => "OWN",
            Station::Sawmill => "SW",
            Station::SteelMill => "SM",
        }
        .to_string()
    }
}

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

impl ToString for Locomotive {
    fn to_string(&self) -> String {
        match self {
            Locomotive::DE2 => "DE2",
            Locomotive::S060 => "S060",
            Locomotive::DM3 => "DM3",
            Locomotive::DH4 => "DH4",
            Locomotive::S282 => "S282",
            Locomotive::DE6 => "DE6",
        }
        .to_string()
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

    locomotives: Vec<LocomotiveInfo>,
    orders: Vec<Order>,
}

impl Default for ConsistManagerApp {
    fn default() -> Self {
        Self {
            add_loco_modal_open: false,
            add_order_modal_open: false,
            selected_loco: locomotives().get(&Locomotive::DE2).unwrap().clone(),
            locomotives: Vec::new(),
            orders: Vec::new(),
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
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("side_menu").show(ctx, |ui| {
            if ui.button("Add Locomotive").clicked() {
                self.add_loco_modal_open = true;
            }
            if ui.button("Add Order").clicked() {
                self.add_order_modal_open = true;
            }
        });

        if self.add_loco_modal_open {
            let modal = egui::Modal::new("Add Locomotive".into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Add Locomotive");
                egui::ComboBox::new("loco", "Locomotive:").show_ui(ui, |ui| {
                    for l in LOCO_LIST {
                        let loco_str = l.to_string();
                        ui.selectable_value(
                            &mut self.selected_loco,
                            locomotives().get(&l).unwrap().clone(),
                            loco_str,
                        );
                    }
                });
                ui.separator();
            });

            if modal.should_close() {
                self.add_loco_modal_open = false;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            TableBuilder::new(ui)
                .columns(Column::auto().resizable(true), 7)
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
                .body(|mut body| {
                    body.rows(30.0, self.orders.len(), |mut row| {
                        let order = self.orders.get(row.index()).unwrap();
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
                    });
                });
        });
    }
}
