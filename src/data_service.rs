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
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub const DISK_SECTOR_METADATA_BLOCKSIZE: u64 = 0x130;

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

        let buffer = Self::read_bytes(
            &mut file,
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

    pub fn write_car_type(&self, track: Track, player: TeamPlayer, car_type: CarType) -> Result<()> {
        let mut file = OpenOptions::new().write(true).open(self.file_path)?;

        let byte_offset = self.language.get_car_type_offset() + track.get_track_select_val() as u64 + player.get_car_type_shift();

        file.seek(SeekFrom::Start(byte_offset))?;

        file.write(&[car_type.get_byte_val()])?;

        Ok(())
    }

    pub fn read_car_physics_by_track(&self, track: Track) -> Result<CarPhysicsByTrack> {
        let mut file = File::open(self.file_path)?;

        let buffer = Self::read_bytes(
            &mut file,
            None, 
            self.language.get_car_physics_by_track_offset(),
            6 * track.get_physics_select_val() as u64,
            6
        )?;
        
        Ok(CarPhysicsByTrack::new_from_byte_array(&buffer))
    }

    pub fn write_car_physics_by_track(&self, track: Track, physics: CarPhysicsByTrack) -> Result<()> {
        let mut file = OpenOptions::new().write(true).open(self.file_path)?;

        let byte_offset = self.language.get_car_physics_by_track_offset() + 6 * track.get_physics_select_val() as u64;

        file.seek(SeekFrom::Start(byte_offset))?;

        file.write(&physics.get_byte_array())?;

        Ok(())
    }

    pub fn read_car_physics_by_car_type(&self, car_type: CarType) -> Result<CarPhysicsByCarType> {
        let mut file = File::open(self.file_path)?;

        let buffer = Self::read_bytes(
            &mut file,
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

        println!("{:?}", physics);

        Ok(physics)
    }

    // Reads bytes (potentially over a sector partition)
    fn read_bytes(
        file: &mut File, 
        partition_start: Option<u64>, 
        disc_offset: u64,
        local_offset: u64,
        length: u64
    ) -> Result<Vec<u8>> {
        if let Some(partition) = partition_start {
            if disc_offset + local_offset + length <= partition {
                // We're before the partition, so handle normally
                Self::read_bytes_linear(file, disc_offset + local_offset, length)
            } else if disc_offset + local_offset >= partition {
                // We're after the partition, so shift by the partition size
                Self::read_bytes_linear(file, disc_offset + local_offset + DISK_SECTOR_METADATA_BLOCKSIZE, length)
            } else {
                // We're bridging the partition
                Self::read_around_sector_partition(file, partition, disc_offset + local_offset, length)
            }
        } else {
            Self::read_bytes_linear(file, disc_offset + local_offset, length)
        }
    }

    fn read_around_sector_partition(file: &mut File, partition_start: u64, byte_offset: u64, length: u64) -> Result<Vec<u8>> {
        let partition_point = partition_start - byte_offset;
        let partition_end = partition_start + DISK_SECTOR_METADATA_BLOCKSIZE;

        file.seek(SeekFrom::Start(byte_offset))?;

        let mut first_buffer = vec![0u8; partition_point as usize];
        file.read(&mut first_buffer)?;

        file.seek(SeekFrom::Start(partition_end))?;

        let mut second_buffer = vec![0u8; (length - partition_point) as usize];
        file.read(&mut second_buffer)?;

        first_buffer.append(&mut second_buffer);
        
        Ok(first_buffer)
    }

    fn read_bytes_linear(file: &mut File, byte_offset: u64, length: u64) -> Result<Vec<u8>> {
        file.seek(SeekFrom::Start(byte_offset))?;

        let mut buffer = vec![0u8; length as usize];
        file.read(&mut buffer)?;
        
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE_PATH: &'static str = "./assets/testbin";

    #[test]
    fn can_read_car_type() {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let mut data_service = DataService::new(&file_path);

        data_service.set_language(Language::TestLanguage);

        let car_type = data_service.read_car_type(Track::Wipeup, TeamPlayer::First);

        assert!(car_type.is_ok());
        assert_eq!(car_type.unwrap(), CarType::DumperTruck);
    }

    #[test]
    fn can_read_physics_by_track() {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let mut data_service = DataService::new(&file_path);

        data_service.set_language(Language::TestLanguage);

        let physics = data_service.read_car_physics_by_track(Track::Wipeup);

        assert!(physics.is_ok());
        assert_eq!(
            physics.unwrap(), 
            CarPhysicsByTrack {
                acceleration: 18,
                top_speed: 19,
                grip: 20,
                collision_impact: 21,
                turning: 22,
                sliding_friction: 23,
            }
        );
    }

    #[test]
    fn can_read_physics_by_car_type() {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let mut data_service = DataService::new(&file_path);

        data_service.set_language(Language::TestLanguage);

        let physics = data_service.read_car_physics_by_car_type(CarType::F1Sixties);

        assert!(physics.is_ok());
        assert_eq!(
            physics.unwrap(), 
            CarPhysicsByCarType {
                grip: 18,
                collision_impact: 257,
                braking: 12,
                top_speed: 8,
                reverse_related: 36,
                turning: 9,
                sliding_friction: 122,
                acceleration: 69,
                _unknown_1: 0,
                _unknown_2: 0,
            }
        );
    }

    #[test]
    fn can_read_physics_by_car_type_accross_partition() {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let mut data_service = DataService::new(&file_path);

        data_service.set_language(Language::TestLanguage);

        let physics = data_service.read_car_physics_by_car_type(CarType::Hovercraft);

        assert!(physics.is_ok());
        assert_eq!(
            physics.unwrap(), 
            CarPhysicsByCarType {
                grip: 19,
                collision_impact: 20,
                braking: 22,
                top_speed: 279,
                reverse_related: 0,
                turning: 0,
                sliding_friction: 0,
                acceleration: 0,
                _unknown_1: 0,
                _unknown_2: 0,
            }
        );
    }

    #[test]
    fn can_read_physics_by_car_type_after_partition() {
        let file_path = PathBuf::from(TEST_FILE_PATH);
        let mut data_service = DataService::new(&file_path);

        data_service.set_language(Language::TestLanguage);

        let physics = data_service.read_car_physics_by_car_type(CarType::Speedboat);

        assert!(physics.is_ok());
        assert_eq!(
            physics.unwrap(), 
            CarPhysicsByCarType {
                grip: 18,
                collision_impact: 20,
                braking: 22,
                top_speed: 274,
                reverse_related: 0,
                turning: 0,
                sliding_friction: 0,
                acceleration: 0,
                _unknown_1: 0,
                _unknown_2: 0,
            }
        );
    }
}