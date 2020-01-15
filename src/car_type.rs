use strum::*;
use serde_derive::{Deserialize, Serialize};

/// Please note that I have changed the names of some of the cars below so that they match what they actually look like!
/// The Cherry's Driving School car has been omitted as it cannot be used in normal play.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumMessage, EnumProperty, Serialize, Deserialize, Hash)]
pub enum CarType {
    #[strum(message="Dune Buggy", props(CarSelect="0x0"))]
    DuneBuggy,
    #[strum(message="Locust", props(CarSelect="0x1"))]
    Locust,
    #[strum(message="Ford GT", props(CarSelect="0x2"))]
    FordGt,
    #[strum(message="Peace Wagon", props(CarSelect="0x3"))]
    PeaceWagon,
    #[strum(message="60s F1", props(CarSelect="0x4"))]
    F1Sixties,
    #[strum(message="Hovercraft", props(CarSelect="0x5"))]
    Hovercraft,
    #[strum(message="Speedboat", props(CarSelect="0x6"))]
    Speedboat,
    #[strum(message="Ferrari Testarossa", props(CarSelect="0x7"))]
    Testarossa,
    #[strum(message="Corvette", props(CarSelect="0x8"))]
    Corvette,
    #[strum(message="Truck", props(CarSelect="0x9"))]
    Truck,
    #[strum(message="Night Boat", props(CarSelect="0xA"))]
    NightBoat,
    #[strum(message="F1", props(CarSelect="0xB"))]
    F1,
    #[strum(message="BMW", props(CarSelect="0xC"))]
    Bmw,
    #[strum(message="Mafia", props(CarSelect="0xD"))]
    Mafia,
    #[strum(message="Tank", props(CarSelect="0xE"))]
    Tank,
    #[strum(message="APC", props(CarSelect="0xF"))]
    Apc,
    #[strum(message="Jeep", props(CarSelect="0x10"))]
    Jeep,
    #[strum(message="Stolly Truck", props(CarSelect="0x11"))]
    StollyTruck,
    #[strum(message="Mini", props(CarSelect="0x12"))]
    Mini,
    #[strum(message="Surf Wagon", props(CarSelect="0x13"))]
    SurfWagon,
    #[strum(message="Dumper Truck", props(CarSelect="0x14"))]
    DumperTruck,
    #[strum(message="Monster Truck", props(CarSelect="0x15"))]
    MonsterTruck,
    #[strum(message="Police Car", props(CarSelect="0x16"))]
    Police,
    #[strum(message="Hot Rod", props(CarSelect="0x17"))]
    HotRod,
    #[strum(message="Ice Cream Van", props(CarSelect="0x18"))]
    IceCream,
    #[strum(message="30s F1", props(CarSelect="0x19"))]
    F1Thirties,
    #[strum(message="Beamer", props(CarSelect="0x1A"))]
    Beamer,
    #[strum(message="Rattler", props(CarSelect="0x1B"))]
    Rattler,
    #[strum(message="Ultra", props(CarSelect="0x1C"))]
    Ultra,
    #[strum(message="Dolphin Car", props(CarSelect="0x1D"))]
    Dolphin,
    #[strum(message="Extreme", props(CarSelect="0x1E"))]
    Extreme,
}

const DUNEBUGGY_LOGO: &'static [u8] = include_bytes!("../assets/DuneBuggy.png");
const LOCUST_LOGO: &'static [u8] = include_bytes!("../assets/Locust.png");
const FORDGT_LOGO: &'static [u8] = include_bytes!("../assets/FordGt.png");
const PEACEWAGON_LOGO: &'static [u8] = include_bytes!("../assets/PeaceWagon.png");
const F1SIXTIES_LOGO: &'static [u8] = include_bytes!("../assets/F1Sixties.png");
const HOVERCRAFT_LOGO: &'static [u8] = include_bytes!("../assets/Hovercraft.png");
const SPEEDBOAT_LOGO: &'static [u8] = include_bytes!("../assets/Speedboat.png");
const TESTAROSSA_LOGO: &'static [u8] = include_bytes!("../assets/Testarossa.png");
const CORVETTE_LOGO: &'static [u8] = include_bytes!("../assets/Corvette.png");
const TRUCK_LOGO: &'static [u8] = include_bytes!("../assets/Truck.png");
const NIGHTBOAT_LOGO: &'static [u8] = include_bytes!("../assets/NightBoat.png");
const F1_LOGO: &'static [u8] = include_bytes!("../assets/F1.png");
const BMW_LOGO: &'static [u8] = include_bytes!("../assets/Bmw.png");
const MAFIA_LOGO: &'static [u8] = include_bytes!("../assets/Mafia.png");
const TANK_LOGO: &'static [u8] = include_bytes!("../assets/Tank.png");
const APC_LOGO: &'static [u8] = include_bytes!("../assets/Apc.png");
const JEEP_LOGO: &'static [u8] = include_bytes!("../assets/Jeep.png");
const STOLLYTRUCK_LOGO: &'static [u8] = include_bytes!("../assets/StollyTruck.png");
const MINI_LOGO: &'static [u8] = include_bytes!("../assets/Mini.png");
const SURFWAGON_LOGO: &'static [u8] = include_bytes!("../assets/SurfWagon.png");
const DUMPERTRUCK_LOGO: &'static [u8] = include_bytes!("../assets/DumperTruck.png");
const MONSTERTRUCK_LOGO: &'static [u8] = include_bytes!("../assets/MonsterTruck.png");
const POLICE_LOGO: &'static [u8] = include_bytes!("../assets/Police.png");
const HOTROD_LOGO: &'static [u8] = include_bytes!("../assets/HotRod.png");
const ICECREAM_LOGO: &'static [u8] = include_bytes!("../assets/IceCream.png");
const F1THIRTIES_LOGO: &'static [u8] = include_bytes!("../assets/F1Thirties.png");
const BEAMER_LOGO: &'static [u8] = include_bytes!("../assets/Beamer.png");
const RATTLER_LOGO: &'static [u8] = include_bytes!("../assets/Rattler.png");
const ULTRA_LOGO: &'static [u8] = include_bytes!("../assets/Ultra.png");
const DOLPHIN_LOGO: &'static [u8] = include_bytes!("../assets/Dolphin.png");
const EXTREME_LOGO: &'static [u8] = include_bytes!("../assets/Extreme.png");

impl CarType {
    pub fn get_image(&self) -> &'static [u8] {
        match self {
            CarType::DuneBuggy => DUNEBUGGY_LOGO,
            CarType::Locust => LOCUST_LOGO,
            CarType::FordGt => FORDGT_LOGO,
            CarType::PeaceWagon => PEACEWAGON_LOGO,
            CarType::F1Sixties => F1SIXTIES_LOGO,
            CarType::Hovercraft => HOVERCRAFT_LOGO,
            CarType::Speedboat => SPEEDBOAT_LOGO,
            CarType::Testarossa => TESTAROSSA_LOGO,
            CarType::Corvette => CORVETTE_LOGO,
            CarType::Truck => TRUCK_LOGO,
            CarType::NightBoat => NIGHTBOAT_LOGO,
            CarType::F1 => F1_LOGO,
            CarType::Bmw => BMW_LOGO,
            CarType::Mafia => MAFIA_LOGO,
            CarType::Tank => TANK_LOGO,
            CarType::Apc => APC_LOGO,
            CarType::Jeep => JEEP_LOGO,
            CarType::StollyTruck => STOLLYTRUCK_LOGO,
            CarType::Mini => MINI_LOGO,
            CarType::SurfWagon => SURFWAGON_LOGO,
            CarType::DumperTruck => DUMPERTRUCK_LOGO,
            CarType::MonsterTruck => MONSTERTRUCK_LOGO,
            CarType::Police => POLICE_LOGO,
            CarType::HotRod => HOTROD_LOGO,
            CarType::IceCream => ICECREAM_LOGO,
            CarType::F1Thirties => F1THIRTIES_LOGO,
            CarType::Beamer => BEAMER_LOGO,
            CarType::Rattler => RATTLER_LOGO,
            CarType::Ultra => ULTRA_LOGO,
            CarType::Dolphin => DOLPHIN_LOGO,
            CarType::Extreme => EXTREME_LOGO,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum TeamPlayer {
    First,
    Second
}

impl Default for TeamPlayer {
    fn default() -> Self {
        TeamPlayer::First
    }
}

impl TeamPlayer {
    pub fn get_car_type_shift(&self) -> u64 {
        match self {
            TeamPlayer::First => 0,
            TeamPlayer::Second => 80
        }
    }

    pub fn get_car_physics_shift(&self) -> u64 {
        match self {
            TeamPlayer::First => 0,
            TeamPlayer::Second => 80
        }
    }
}

impl CarType {
    pub fn get_byte_val(&self) -> u8 {
        let trimmed_string = self.get_str("CarSelect")
            .expect(&format!("Missing CarSelect prop for car type {:?}", self))
            .trim_start_matches("0x");

        u8::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse CarSelect prop on car type {:?}", self))
    }

    pub fn new_from_byte(byte: u8) -> Option<CarType> {
        CarType::iter().find(|car_type| { 
            car_type.get_byte_val() == byte
        })
    }

    pub fn get_physics_select_val(&self) -> u8 {
        let trimmed_string = self.get_str("CarSelect")
            .expect(&format!("Missing CarSelect prop for car type {:?}", self))
            .trim_start_matches("0x");

        u8::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse CarSelect prop on car type {:?}", self))
    }
}

#[test]
fn correctly_parses_byte_val() {
    let car_type = CarType::Rattler;

    let byte_val = car_type.get_byte_val();

    assert_eq!(byte_val, 27)
}