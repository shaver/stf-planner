/*
    TODO:
    - load ships
    - test ships
    - load ship default configs
    - test default configs
    - JSON
    - move to lib?
*/

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct ShipComponent {
    name: String,
    size: String,
    #[serde(rename="Current Mass")] mass: i32,
    pilot: u32,
    #[serde(rename="Ship Ops")] ship_ops: u32,
    gunnery: u32,
    electronics: u32,
    navigation: u32,
    cargo: u32,
    #[serde(rename="Max Crew")] max_crew: u32,
    #[serde(rename="Max Officers")] max_officers: u32,
    armour: u32,
    shield: u32,
    #[serde(rename="Jump Cost")] jump_cost: u32,
    #[serde(rename="Fuel Tank")] fuel_tank: u32,
    guest: u32,
    prison: u32,
    medical: u32
}

pub type ShipComponentMap = HashMap<String, ShipComponent>;

fn make_csv_reader<T: std::io::Read>(rdr: T) -> csv::Reader<T> {
    csv::ReaderBuilder::new().delimiter(b':').from_reader(rdr)
}

fn load_ship_components_from_reader<T: std::io::Read>(rdr: T) -> Result<ShipComponentMap, csv::Error> {
    let mut rdr = make_csv_reader(rdr);
    let mut map = ShipComponentMap::new();

    for result in rdr.deserialize() {
        let record: ShipComponent = result?;
        map.insert(record.name.clone(), record);
    }

    Result::Ok(map)
}

pub fn load_ship_components(components_file: &str) -> Result<ShipComponentMap, std::io::Error> {
    let components_path = Path::new(components_file);
    
    let cm = load_ship_components_from_reader(File::open(&components_path)?)?;
    Result::Ok(cm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SHIP_COMPONENTS_CSV: &str =
"Name:Size:Current Mass:Pilot:Ship Ops:Gunnery:Electronics:Navigation:Cargo:Max Crew:Max Officers:Armour:Shield:Jump Cost:Fuel Tank:Guest:Prison:Medical
Adv. Mass Dampener 1:Medium:-300:0:3:0:0:0:0:0:0:0:0:5:0:0:0:0
Aramech Missile Pod:Small:125:0:2:7:1:0:0:0:0:0:0:0:0:0:0:0
Capital Bridge 3:Large:600:9:6:0:10:9:0:0:1:1:0:0:0:0:0:0
Combat Hospital:Large:650:0:5:0:3:0:0:0:0:1:2:3:0:0:0:8
Luxury Suites:Large:500:0:0:0:0:0:0:0:0:0:2:0:0:3:0:0
Mass Modulator 4:Small:-165:3:0:0:2:0:0:0:0:0:0:4:0:0:0:0
Surface canner:Small:150:0:0:0:0:0:0:0:0:0:0:0:0:0:0:0
Weapons Locker A5:Small:175:0:0:0:0:0:0:0:0:0:0:0:0:0:0:0";

    #[test]
    fn test_load_ship_components() {
        let csv = TEST_SHIP_COMPONENTS_CSV.as_bytes();
        let sc = load_ship_components_from_reader(csv).unwrap();

        let c = &sc["Adv. Mass Dampener 1"];
        assert_eq!(c.size, "Medium");
        assert_eq!(c.mass, -300);
        assert_eq!(c.jump_cost, 5);

        let c = &sc["Capital Bridge 3"];
        assert_eq!(c.size, "Large");
        assert_eq!(c.mass, 600);
        assert_eq!(c.pilot, 9);
        assert_eq!(c.ship_ops, 6);
        assert_eq!(c.electronics, 10);
        assert_eq!(c.navigation, 9);
        assert_eq!(c.max_officers, 1);
        assert_eq!(c.armour, 1);

        let c = &sc["Combat Hospital"];
        assert_eq!(c.mass, 650);
        assert_eq!(c.medical, 8);
    }
}