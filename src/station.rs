use std::fmt::{self, Display, Formatter};

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

pub const STATIONS: [Station; 20] = [
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
