use crate::car_physics::CarPhysicsByCarType;
use crate::car_physics::CarPhysicsByTrack;
use crate::car_type::CarType;
use crate::car_type::TeamPlayer;
use crate::track::Track;
use crate::language::Language;
use crate::data_service::read_bytes;

use std::io::{Error, Result, ErrorKind};
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub const DEFAULT_DATA: &'static [u8] = include_bytes!("../assets/defaultbin");

pub struct DefaultDataService {
    language: Language,
}

impl<'a> DefaultDataService {
    pub fn new() -> Self {
        Self {
            language: Language::DefaultData
        }
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn read_car_type(&self, track: Track, player: TeamPlayer) -> Result<CarType> {
        let mut data_cursor = Cursor::new(DEFAULT_DATA);

        let buffer = read_bytes(
            &mut data_cursor,
            None, 
            self.language.get_car_type_offset(),
            track.get_track_select_val() as u64 + player.get_car_type_shift(),
            1
        )?;

        let car_type = CarType::new_from_byte(buffer[0]);

        match car_type {
            Some(car_type) => Ok(car_type),
            _ => Err(Error::new(ErrorKind::Other, "Byte val does not have a matching car type!"))
        }
    }

    pub fn read_car_physics_by_track(&self, track: Track) -> Result<CarPhysicsByTrack> {
        let mut data_cursor = Cursor::new(DEFAULT_DATA);

        let buffer = read_bytes(
            &mut data_cursor,
            None, 
            self.language.get_car_physics_by_track_offset(),
            6 * track.get_physics_select_val() as u64,
            6
        )?;
        
        Ok(CarPhysicsByTrack::new_from_byte_array(&buffer))
    }

    pub fn read_car_physics_by_car_type(&self, car_type: CarType) -> Result<CarPhysicsByCarType> {
        let mut data_cursor = Cursor::new(DEFAULT_DATA);

        let buffer = read_bytes(
            &mut data_cursor,
            self.language.get_car_physics_by_car_type_partition(), 
            self.language.get_car_physics_by_car_type_offset(),
            20 * car_type.get_physics_select_val() as u64,
            20
        )?;

        let mut rdr = Cursor::new(buffer);

        let physics = CarPhysicsByCarType {
            grip: rdr.read_u16::<LittleEndian>()?,
            collision_impact: rdr.read_u16::<LittleEndian>()?,
            braking: rdr.read_u16::<LittleEndian>()?, 
            top_speed: rdr.read_u16::<LittleEndian>()?,
            reverse_related: rdr.read_u16::<LittleEndian>()?,
            turning: rdr.read_u16::<LittleEndian>()?,
            sliding_friction: rdr.read_u16::<LittleEndian>()?,
            acceleration: rdr.read_u16::<LittleEndian>()?,
            _unknown_1: rdr.read_u16::<LittleEndian>()?,
            _unknown_2: rdr.read_u16::<LittleEndian>()?,
        };

        Ok(physics)
    }
}