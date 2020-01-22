use strum::*;
use serde_derive::{Deserialize, Serialize};

const ENGLISH_LOGO: &'static [u8] = include_bytes!("../assets/English.png");
const FRENCH_LOGO: &'static [u8] = include_bytes!("../assets/French.png");
const GERMAN_LOGO: &'static [u8] = include_bytes!("../assets/German.png");
const ITALIAN_LOGO: &'static [u8] = include_bytes!("../assets/Italian.png");
const SPANISH_LOGO: &'static [u8] = include_bytes!("../assets/Spanish.png");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumIter, EnumMessage, EnumProperty, Serialize, Deserialize)]
pub enum Language {
    #[strum(props(CarTypeOffset="0xAEBCD6C", CarPhysicsByCarTypeOffset="0xAEBB78C", CarPhysicsByCarTypePartition="0xAEBB928", CarPhysicsByTrackOffset="0xAEBBC1C"))]
    English,
    #[strum(props(CarTypeOffset="0xB0B5A00", CarPhysicsByCarTypeOffset="0xB0B4420", CarPhysicsByCarTypePartition="0xB0B44F8", CarPhysicsByTrackOffset="0xB0B48B0"))]
    French,
    #[strum(props(CarTypeOffset="0xB2AE568", CarPhysicsByCarTypeOffset="0xB2ACF88", CarPhysicsByCarTypePartition="0xB2AD0C8", CarPhysicsByTrackOffset="0xB2AD418"))]
    German,
    #[strum(props(CarTypeOffset="0xB69FFEC", CarPhysicsByCarTypeOffset="0xB69EB3C", CarPhysicsByTrackOffset="0xB69EE9C"))]
    Italian,
    #[strum(props(CarTypeOffset="0xB4A732C", CarPhysicsByCarTypeOffset="0xB4A5E7C", CarPhysicsByTrackOffset="0xB4A61DC"))]
    Spanish,
    #[strum(props(CarTypeOffset="0xA", CarPhysicsByCarTypeOffset="0xBB", CarPhysicsByCarTypePartition="123", CarPhysicsByTrackOffset="0xCCC"))]
    TestLanguage,
    #[strum(props(CarTypeOffset="0x0", CarPhysicsByCarTypeOffset="0xC0",  CarPhysicsByTrackOffset="0x220"))]
    DefaultData
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl Language {
    pub fn get_image(&self) -> &'static[u8] {
        match self {
            Language::English => ENGLISH_LOGO,
            Language::French => FRENCH_LOGO,
            Language::German => GERMAN_LOGO,
            Language::Italian => ITALIAN_LOGO,
            Language::Spanish => SPANISH_LOGO,
            _ => ENGLISH_LOGO
        }
    }
    
    pub fn get_car_type_offset(&self) -> u64 {
        let trimmed_string = self.get_str("CarTypeOffset")
            .expect(&format!("Missing CarTypeOffset prop for language {:?}", self))
            .trim_start_matches("0x");

        u64::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse CarTypeOffset prop on language {:?}", self))
    }

    pub fn get_car_physics_by_car_type_offset(&self) -> u64 {
        let trimmed_string = self.get_str("CarPhysicsByCarTypeOffset")
            .expect(&format!("Missing CarPhysicsByCarTypeOffset prop for language {:?}", self))
            .trim_start_matches("0x");

        u64::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse CarPhysicsByCarTypeOffset prop on language {:?}", self))
    }

    pub fn get_car_physics_by_car_type_partition(&self) -> Option<u64> {
        let trimmed_string = self.get_str("CarPhysicsByCarTypePartition")?
            .trim_start_matches("0x");

        Some(
            u64::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse CarPhysicsByCarTypePartition prop on language {:?}", self))
        )
    }

    pub fn get_car_physics_by_track_offset(&self) -> u64 {
        let trimmed_string = self.get_str("CarPhysicsByTrackOffset")
            .expect(&format!("Missing CarPhysicsByTrackOffset prop for language {:?}", self))
            .trim_start_matches("0x");

        u64::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse CarPhysicsByTrackOffset prop on language {:?}", self))
    }
}