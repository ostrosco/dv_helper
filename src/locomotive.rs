use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::sync::OnceLock;

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LocomotiveInfo {
    pub loco: Locomotive,
    pub weight: f32,
    pub length: f32,
    pub zero_grade_t: u16,
    pub two_grade_t: u16,
    pub rain_grade_t: u16,
    pub has_power: bool,
    pub powered: bool,
}

impl LocomotiveInfo {
    pub fn new(
        loco: Locomotive,
        weight: f32,
        length: f32,
        zero_grade_t: u16,
        two_grade_t: u16,
        rain_grade_t: u16,
        powered: bool,
    ) -> Self {
        let length = length / 1000.0;
        Self {
            loco,
            weight,
            length,
            zero_grade_t,
            two_grade_t,
            rain_grade_t,
            has_power: powered,
            powered,
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
    DE6Slug,
    BE2,
    DM1U,
    Caboose,
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
            Self::DE6Slug => "DE6 Slug",
            Self::BE2 => "BE2-260",
            Self::DM1U => "DM1U-150",
            Self::Caboose => "Caboose",
        };
        write!(f, "{loco_str}")
    }
}

pub const LOCO_LIST: [Locomotive; 10] = [
    Locomotive::DE2,
    Locomotive::S060,
    Locomotive::DM3,
    Locomotive::DH4,
    Locomotive::S282,
    Locomotive::DE6,
    Locomotive::DE6Slug,
    Locomotive::BE2,
    Locomotive::DM1U,
    Locomotive::Caboose,
];

pub fn locomotives() -> &'static HashMap<Locomotive, LocomotiveInfo> {
    static LOCOMOTIVES: OnceLock<HashMap<Locomotive, LocomotiveInfo>> = OnceLock::new();
    LOCOMOTIVES.get_or_init(|| {
        let mut l = HashMap::new();
        l.insert(
            Locomotive::DE2,
            LocomotiveInfo::new(Locomotive::DE2, 38.0, 7600.0, 1200, 300, 250, true),
        );
        l.insert(
            Locomotive::S060,
            LocomotiveInfo::new(Locomotive::S060, 50.7, 9320.0, 1500, 400, 300, true),
        );
        l.insert(
            Locomotive::DM3,
            LocomotiveInfo::new(Locomotive::DM3, 52.0, 8600.0, 2000, 500, 400, true),
        );
        l.insert(
            Locomotive::DH4,
            LocomotiveInfo::new(Locomotive::DH4, 77.5, 12840.0, 2000, 600, 500, true),
        );
        l.insert(
            Locomotive::S282,
            LocomotiveInfo::new(Locomotive::S282, 174.8, 22180.0, 3000, 1000, 800, true),
        );
        l.insert(
            Locomotive::DE6,
            LocomotiveInfo::new(Locomotive::DE6, 125.0, 18640.0, 3000, 1200, 1000, true),
        );
        l.insert(
            Locomotive::DE6Slug,
            LocomotiveInfo::new(Locomotive::DE6Slug, 125.0, 16800.0, 0, 0, 0, false),
        );
        l.insert(
            Locomotive::BE2,
            LocomotiveInfo::new(Locomotive::BE2, 12.0, 4080.0, 800, 100, 50, true),
        );
        l.insert(
            Locomotive::DM1U,
            LocomotiveInfo::new(Locomotive::DM1U, 10.4, 14470.0, 0, 0, 0, true),
        );
        l.insert(
            Locomotive::Caboose,
            LocomotiveInfo::new(Locomotive::Caboose, 22.0, 13200.0, 0, 0, 0, false),
        );
        l
    })
}
