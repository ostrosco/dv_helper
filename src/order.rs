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

pub struct NewOrderModal {
    pub selected_order_name: String,
    pub selected_weight: String,
    pub selected_length: String,
    pub selected_pickup: Station,
    pub selected_pickup_track: String,
    pub selected_dropoff: Station,
    pub selected_dropoff_track: String,
    pub new_order: Option<Order>,
    pub open: bool,
}

impl NewOrderModal {
    pub fn new() -> Self {
        Self {
            selected_order_name: String::new(),
            selected_weight: String::new(),
            selected_length: String::new(),
            selected_pickup: Station::SteelMill,
            selected_pickup_track: String::new(),
            selected_dropoff: Station::Harbor,
            selected_dropoff_track: String::new(),
            new_order: None,
            open: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if self.open {
            egui::Modal::new("Add Order".into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Add Order");
                ui.label("Order Name");
                ui.text_edit_singleline(&mut self.selected_order_name);
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
                            self.create_order();
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

    fn create_order(&mut self) {
        self.new_order = Some(Order {
            name: self.selected_order_name.clone(),
            weight: str::parse(&self.selected_weight).expect("Invalid weight"),
            length: str::parse(&self.selected_length).expect("Invalid length"),
            pickup_station: self.selected_pickup,
            pickup_track: self.selected_pickup_track.clone(),
            dropoff_station: self.selected_dropoff,
            dropoff_track: self.selected_dropoff_track.clone(),
        });
        self.selected_order_name = String::new();
        self.selected_weight = String::new();
        self.selected_length = String::new();
        self.selected_pickup_track = String::new();
        self.selected_dropoff_track = String::new();
    }
}

pub struct EditOrderModal {
    pub edited_order_name: String,
    pub edited_weight: String,
    pub edited_length: String,
    pub edited_pickup: Station,
    pub edited_pickup_track: String,
    pub edited_dropoff: Station,
    pub edited_dropoff_track: String,
    pub edited_order: Option<Order>,
    pub open: bool,
    pub index: usize,
}

impl EditOrderModal {
    pub fn new() -> Self {
        Self {
            edited_order_name: String::new(),
            edited_weight: String::new(),
            edited_length: String::new(),
            edited_pickup: Station::SteelMill,
            edited_pickup_track: String::new(),
            edited_dropoff: Station::Harbor,
            edited_dropoff_track: String::new(),
            edited_order: None,
            open: false,
            index: 0,
        }
    }

    pub fn init(&mut self, order: &Order, index: usize) {
        *self = Self {
            edited_order_name: order.name.clone(),
            edited_weight: order.weight.to_string(),
            edited_length: order.length.to_string(),
            edited_pickup: order.pickup_station,
            edited_pickup_track: order.pickup_track.clone(),
            edited_dropoff: order.dropoff_station,
            edited_dropoff_track: order.dropoff_track.clone(),
            edited_order: None,
            open: false,
            index,
        };
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if self.open {
            egui::Modal::new("Edit Order".into()).show(ctx, |ui| {
                ui.set_width(250.0);
                ui.heading("Edit Order");
                ui.label("Order Name");
                ui.text_edit_singleline(&mut self.edited_order_name);
                ui.label("Weight");
                ui.text_edit_singleline(&mut self.edited_weight);
                ui.label("Length");
                ui.text_edit_singleline(&mut self.edited_length);
                ui.separator();
                egui::ComboBox::from_label("Pickup Station:")
                    .selected_text(self.edited_pickup.to_abbrev())
                    .show_ui(ui, |ui| {
                        for s in STATIONS {
                            let station_str = s.to_abbrev();
                            ui.selectable_value(&mut self.edited_pickup, s, station_str);
                        }
                    });
                ui.label("Pickup Track");
                ui.separator();
                ui.text_edit_singleline(&mut self.edited_pickup_track);
                egui::ComboBox::from_label("Dropoff Station:")
                    .selected_text(self.edited_dropoff.to_abbrev())
                    .show_ui(ui, |ui| {
                        for s in STATIONS {
                            let station_str = s.to_abbrev();
                            ui.selectable_value(&mut self.edited_dropoff, s, station_str);
                        }
                    });
                ui.label("Dropoff Track");
                ui.text_edit_singleline(&mut self.edited_dropoff_track);
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Edit").clicked() {
                            self.edit_order();
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

    fn edit_order(&mut self) {
        self.edited_order = Some(Order {
            name: self.edited_order_name.clone(),
            weight: str::parse(&self.edited_weight).expect("Invalid weight"),
            length: str::parse(&self.edited_length).expect("Invalid length"),
            pickup_station: self.edited_pickup,
            pickup_track: self.edited_pickup_track.clone(),
            dropoff_station: self.edited_dropoff,
            dropoff_track: self.edited_dropoff_track.clone(),
        });
        self.edited_order_name = String::new();
        self.edited_weight = String::new();
        self.edited_length = String::new();
        self.edited_pickup_track = String::new();
        self.edited_dropoff_track = String::new();
    }
}
