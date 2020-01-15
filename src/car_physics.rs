use serde_derive::{Deserialize, Serialize};

/// Overrides the corresponding CarPhysicsByCarType data 
/// unless acceleration = 0
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarPhysicsByTrack {
    pub acceleration: u8,
    pub top_speed: u8,
    pub grip: u8,
    pub collision_impact: u8,
    pub turning: u8,
    pub sliding_friction: u8,
}

/// Contains the base physics data for a car type but some fields 
/// can be overwritten by the relevant CarPhysicsByTrack values
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarPhysicsByCarType {
    pub grip: u16,
    pub collision_impact: u16,
    pub braking: u16, /// Not fully understood
    pub top_speed: u16,
    pub reverse_related: u16, /// Not fully understood
    pub turning: u16,
    pub sliding_friction: u16,
    pub acceleration: u16,
    pub _unknown_1: u16,
    pub _unknown_2: u16
}

impl From<CarPhysicsByCarType> for CarPhysicsByTrack {
    fn from(car_type_physics: CarPhysicsByCarType) -> Self {
        Self {
            acceleration: car_type_physics.acceleration as u8,
            top_speed: (car_type_physics.top_speed / 100) as u8,
            grip: (car_type_physics.grip / 10) as u8,
            collision_impact: (car_type_physics.collision_impact / 4) as u8,
            turning: car_type_physics.turning as u8,
            sliding_friction: car_type_physics.sliding_friction as u8
        }
    }
}

impl CarPhysicsByTrack {
    pub fn get_byte_array(&self) -> [u8; 6] {
        [
            self.acceleration, 
            self.top_speed,
            self.grip,
            self.collision_impact,
            self.turning,
            self.sliding_friction
        ]
    }

    pub fn new_from_byte_array(bytes: &[u8]) -> Self {
        Self {
            acceleration: bytes[0],
            top_speed: bytes[1],
            grip: bytes[2],
            collision_impact: bytes[3],
            turning: bytes[4],
            sliding_friction: bytes[5] 
        }
    }
}
