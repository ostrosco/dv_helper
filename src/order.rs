use crate::station::{STATIONS, Station};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Order {
    pub name: String,
    pub weight: f32,
    pub length: f32,
    pub pickup_station: Station,
    pub pickup_track: String,
    pub dropoff_station: Station,
    pub dropoff_track: String,
}

pub enum OrderModalMode {
    New,
    Edit,
}

pub struct OrderModal {
    pub modal_mode: OrderModalMode,
    pub order_name: String,
    pub weight: String,
    pub length: String,
    pub pickup: Station,
    pub pickup_track: String,
    pub dropoff: Station,
    pub dropoff_track: String,
    pub order: Option<Order>,
    pub open: bool,
    pub index: usize,
}

impl OrderModal {
    pub fn new(modal_mode: OrderModalMode) -> Self {
        Self {
            modal_mode,
            order_name: String::new(),
            weight: String::new(),
            length: String::new(),
            pickup: Station::SteelMill,
            pickup_track: String::new(),
            dropoff: Station::Harbor,
            dropoff_track: String::new(),
            order: None,
            open: false,
            index: 0,
        }
    }

    pub fn init_from_order(&mut self, order: &Order, index: usize) {
        let Order {
            name,
            weight,
            length,
            pickup_station,
            pickup_track,
            dropoff_station,
            dropoff_track,
        } = order;
        self.order_name = name.clone();
        self.weight = weight.to_string();
        self.length = length.to_string();
        self.pickup = *pickup_station;
        self.pickup_track = pickup_track.clone();
        self.dropoff = *dropoff_station;
        self.dropoff_track = dropoff_track.clone();
        self.index = index;
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let (order_label, button_label): (String, String) = match self.modal_mode {
            OrderModalMode::New => ("Add Order".into(), "Add".into()),
            OrderModalMode::Edit => ("Edit Order".into(), "Edit".into()),
        };
        if self.open {
            egui::Modal::new(order_label.clone().into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading(order_label);
                ui.label("Order Name");
                ui.text_edit_singleline(&mut self.order_name);
                ui.label("Weight");
                ui.text_edit_singleline(&mut self.weight);
                ui.label("Length");
                ui.text_edit_singleline(&mut self.length);
                ui.separator();
                egui::ComboBox::from_label("Pickup Station:")
                    .selected_text(self.pickup.to_abbrev())
                    .show_ui(ui, |ui| {
                        for s in STATIONS {
                            let station_str = s.to_abbrev();
                            ui.selectable_value(&mut self.pickup, s, station_str);
                        }
                    });
                ui.label("Pickup Track");
                ui.separator();
                ui.text_edit_singleline(&mut self.pickup_track);
                egui::ComboBox::from_label("Dropoff Station:")
                    .selected_text(self.dropoff.to_abbrev())
                    .show_ui(ui, |ui| {
                        for s in STATIONS {
                            let station_str = s.to_abbrev();
                            ui.selectable_value(&mut self.dropoff, s, station_str);
                        }
                    });
                ui.label("Dropoff Track");
                ui.text_edit_singleline(&mut self.dropoff_track);
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button(button_label).clicked() {
                            self.update_order();
                            self.open = false;
                            ui.close();
                        }
                        if ui.button("Cancel").clicked() {
                            self.open = false;
                            ui.close();
                        }
                    },
                );
            });
        }
    }

    fn update_order(&mut self) {
        self.order = Some(Order {
            name: self.order_name.clone(),
            weight: str::parse(&self.weight).expect("Invalid weight"),
            length: str::parse(&self.length).expect("Invalid length"),
            pickup_station: self.pickup,
            pickup_track: self.pickup_track.clone(),
            dropoff_station: self.dropoff,
            dropoff_track: self.dropoff_track.clone(),
        });
        self.order_name = String::new();
        self.weight = String::new();
        self.length = String::new();
        self.pickup_track = String::new();
        self.dropoff_track = String::new();
    }
}
