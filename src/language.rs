use strum::*;

const ENGLISH_LOGO: &'static [u8] = include_bytes!("../assets/English.png");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumIter, EnumMessage, EnumProperty)]
pub enum Language {
    #[strum(props(CarTypeOffset="0xAEBCD6C", CarPhysicsByCarTypeOffset="0xAEBB78C", CarPhysicsByCarTypePartition="0xAEBB928", CarPhysicsByTrackOffset="0xAEBBC1C"))]
    English,
    #[strum(props(CarTypeOffset="0xA", CarPhysicsByCarTypeOffset="0xBB", CarPhysicsByCarTypePartition="123", CarPhysicsByTrackOffset="0xCCC"))]
    TestLanguage
}

impl Language {
    pub fn get_image(&self) -> &'static[u8] {
        ENGLISH_LOGO
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