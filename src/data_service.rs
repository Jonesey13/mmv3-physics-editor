use std::path::PathBuf;
use crate::car_physics::CarPhysicsByCarType;
use crate::car_physics::CarPhysicsByTrack;
use crate::car_type::CarType;
use crate::car_type::TeamPlayer;
use crate::track::Track;
use crate::language::Language;

use std::fs::File;
use std::io::{Error, Seek, SeekFrom, Read, Write, Result, ErrorKind};
use std::fs::OpenOptions;

pub struct DataService<'a> {
    language: Language,
    file_path: &'a PathBuf
}

impl<'a> DataService<'a> {
    pub fn new(file_path: &'a PathBuf) -> Self {
        Self {
            language: Language::English,
            file_path
        }
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn read_car_type(&self, track: Track, player: TeamPlayer) -> Result<CarType> {
        let mut file = File::open(self.file_path)?;

        let byte_offset = self.language.get_car_type_offset() + track.get_track_select_val() as u64 + player.get_car_type_shift();

        file.seek(SeekFrom::Start(byte_offset))?;

        let mut buffer = [0u8; 1];
        file.read(&mut buffer)?;
        let car_type_byte = buffer[0];
        let car_type = CarType::new_from_byte(car_type_byte);

        match car_type {
            Some(car_type) => Ok(car_type),
            _ => Err(Error::new(ErrorKind::Other, "Byte val does not have a matching car type!"))
        }
    }

    pub fn write_car_type(&self, track: Track, player: TeamPlayer, car_type: CarType) -> Result<()> {
        let mut file = OpenOptions::new().write(true).open(self.file_path)?;

        let byte_offset = self.language.get_car_type_offset() + track.get_track_select_val() as u64 + player.get_car_type_shift();

        file.seek(SeekFrom::Start(byte_offset))?;

        file.write(&[car_type.get_byte_val()])?;

        Ok(())
    }

    pub fn read_car_physics_by_track(&self, track: Track) -> CarPhysicsByTrack {
        unimplemented!()
    }

    pub fn read_car_physics_by_car_type(&self, car_type: CarType) -> CarPhysicsByCarType {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE_PATH: &'static str = "./assets/test.bin";

    #[test]
    fn can_read_car_type() {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let mut data_service = DataService::new(&file_path);

        data_service.set_language(Language::TestLanguage);

        let car_type = data_service.read_car_type(Track::Wipeup, TeamPlayer::First);

        assert!(car_type.is_ok());
        assert_eq!(car_type.unwrap(), CarType::DumperTruck);
    }
}