use strum::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumMessage, EnumProperty, Serialize, Deserialize)]
pub enum Track
{
    #[strum(message="Cheesey Jumps", props(TrackSelect="1", PhysicsSelect="0x1"))]
    Cheesey,
    #[strum(message="Cereal Killer", props(TrackSelect="2", PhysicsSelect="0x2"))]
    Cereal,
    #[strum(message="Breakfast At Cherry's", props(TrackSelect="3", PhysicsSelect="0x3"))]
    Breakfast,
    #[strum(message="Super Bowl", props(TrackSelect="4", PhysicsSelect="0x4"))]
    SuperBowl,
    #[strum(message="Hair Of The Dog", props(TrackSelect="5", PhysicsSelect="0x5"))]
    HairDog,
    #[strum(message="Wipeup", props(TrackSelect="6", PhysicsSelect="0x6"))]
    Wipeup,
    #[strum(message="Brake-Fast Bends", props(TrackSelect="7", PhysicsSelect="0x28"))]
    BrakeFast,
    #[strum(message="Destruction Dirtbox", props(TrackSelect="11", PhysicsSelect="0x7"))]
    Destruction,
    #[strum(message="Beware of the Dog", props(TrackSelect="12", PhysicsSelect="0x8"))]
    BewareDog,
    #[strum(message="Toad Rage", props(TrackSelect="13", PhysicsSelect="0x9"))]
    Toad,
    #[strum(message="Snail Trail", props(TrackSelect="14", PhysicsSelect="0xA"))]
    Snail,
    #[strum(message="Splash N Dash", props(TrackSelect="15", PhysicsSelect="0xB"))]
    Splash,
    #[strum(message="Pond Life", props(TrackSelect="16", PhysicsSelect="0xC"))]
    Pond,
    #[strum(message="Crash and Fern", props(TrackSelect="17", PhysicsSelect="0xD"))]
    CrashFern,
    #[strum(message="Swerve Shot", props(TrackSelect="21", PhysicsSelect="0xE"))]
    Swerve,
    #[strum(message="Rack N Roll", props(TrackSelect="22", PhysicsSelect="0xF"))]
    Rack,
    #[strum(message="Right On Cue", props(TrackSelect="23", PhysicsSelect="0x10"))]
    RightCue,
    #[strum(message="Pot Luck", props(TrackSelect="24", PhysicsSelect="0x11"))]
    PotLuck,
    #[strum(message="Love Triangle", props(TrackSelect="25", PhysicsSelect="0x0"))]
    Love,
    #[strum(message="Stinky Sinks", props(TrackSelect="31", PhysicsSelect="0x12"))]
    StinkySinks,
    #[strum(message="Pulling Power", props(TrackSelect="32", PhysicsSelect="0x13"))]
    Pulling,
    #[strum(message="Interesting Voyage", props(TrackSelect="33", PhysicsSelect="0x14"))]
    InterestingVoyage,
    #[strum(message="Formula X", props(TrackSelect="34", PhysicsSelect="0x15"))]
    FormulaX,
    #[strum(message="Bio Hazard", props(TrackSelect="35", PhysicsSelect="0x16"))]
    BioHazard,
    #[strum(message="Periodic Park", props(TrackSelect="36", PhysicsSelect="0x17"))]
    Periodic,
    #[strum(message="Chemical Warfare", props(TrackSelect="37", PhysicsSelect="0x18"))]
    Chemical,
    #[strum(message="Pebble Dash", props(TrackSelect="51", PhysicsSelect="0x19"))]
    PebbleDash,
    #[strum(message="Bikini Blazer", props(TrackSelect="52", PhysicsSelect="0x1A"))]
    Bikini,
    #[strum(message="Beached Buggies", props(TrackSelect="53", PhysicsSelect="0x1B"))]
    BeachedBuggies,
    #[strum(message="Bucket and Speed", props(TrackSelect="54", PhysicsSelect="0x1C"))]
    BucketSpeed,
    #[strum(message="Sand Blaster", props(TrackSelect="55", PhysicsSelect="0x1D"))]
    SandBlaster,
    #[strum(message="Dunes Of Hazard", props(TrackSelect="56", PhysicsSelect="0x29"))]
    DunesHazard,
    #[strum(message="Vindaloo Drive Thru", props(TrackSelect="61", PhysicsSelect="0x1E"))]
    Vindaloo,
    #[strum(message="Baguette Balance", props(TrackSelect="62", PhysicsSelect="0x1F"))]
    Baguette,
    #[strum(message="The Main Course", props(TrackSelect="63", PhysicsSelect="0x20"))]
    MainCourse,
    #[strum(message="Tanks Alot", props(TrackSelect="64", PhysicsSelect="0x21"))]
    TanksAlot,
    #[strum(message="Fast Food", props(TrackSelect="65", PhysicsSelect="0x22"))]
    FastFood,
    #[strum(message="Calculator Risk", props(TrackSelect="71", PhysicsSelect="0x23"))]
    Calculator,
    #[strum(message="Trucker's Luck", props(TrackSelect="72", PhysicsSelect="0x24"))]
    Truckers,
    #[strum(message="Text Book Manoeuver", props(TrackSelect="73", PhysicsSelect="0x25"))]
    Textbook,
    #[strum(message="Must Try Harder", props(TrackSelect="74", PhysicsSelect="0x26"))]
    MustTryHarder,
    #[strum(message="Learning Curves", props(TrackSelect="75", PhysicsSelect="0x27"))]
    Learning,
    #[strum(message="School Rulez", props(TrackSelect="76", PhysicsSelect="0x28"))]
    SchoolRulez,
}

impl Track {
    pub fn get_track_select_val(&self) -> u8 {
        self.get_str("TrackSelect")
            .expect(&format!("Missing TrackSelect prop for track {:?}", self))
            .parse::<u8>()
            .expect(&format!("Could not parse TrackSelect prop on track {:?}", self))
    }

    pub fn get_physics_select_val(&self) -> u8 {
        let trimmed_string = self.get_str("PhysicsSelect")
            .expect(&format!("Missing PhysicsSelect prop for track {:?}", self))
            .trim_start_matches("0x");

        u8::from_str_radix(trimmed_string, 16)
            .expect(&format!("Could not parse PhysicsSelect prop on track {:?}", self))
    }
}