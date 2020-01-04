/// Overrides the corresponding CarPhysicsByCarType data 
/// unless acceleration = 0
#[derive(Copy, Clone, Debug)]
pub struct CarPhysicsByTrack {
    acceleration: u8,
    top_speed: u8,
    grip: u8,
    collision_impact: u8,
    turning: u8,
    sliding_friction: u8,
}

/// Contains the base physics data for a car type but some fields 
/// can be overwritten by the relevant CarPhysicsByTrack values
#[derive(Copy, Clone, Debug)]
pub struct CarPhysicsByCarType {
    grip: u16,
    collision_impact: u16,
    braking: u16, /// Not fully understood
    top_speed: u16,
    reverse_related: u16, /// Not fully understood
    turning: u16,
    sliding_friction: u16,
    acceleration: u16,
    _unknown_1: u16,
    _unknown_2: u16
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