/*
    TODO:
    - load talents
    - JSON
    - move to lib?
*/

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default)]
pub struct SkillRating {
    pub name: String,          // Name of the skill
    pub ratings: [u32; 32]      // Ratings at levels 1 through 32
}

#[derive(Debug, Default)]
pub struct Talent {
    pub name: String,
    pub rank: u32,
    pub description: String,
    pub cost: String,
    pub type_: String
}

#[derive(Debug, Default)]
pub struct Job {
    pub name: String,
    pub skill_ratings: Vec<SkillRating>,
    pub talents: Vec<Talent>
}

pub type JobMap = HashMap<String, Job>;

fn find_job<'a>(map: &'a mut JobMap, name: &String) -> &'a mut Job {
    map.entry(name.clone())
        .or_insert(Job { name: name.clone(), ..Default::default()})
}

fn make_csv_reader<T: std::io::Read>(rdr: T) -> csv::Reader<T> {
    csv::ReaderBuilder::new().delimiter(b':').from_reader(rdr)
}

impl Job {
    fn fill_rating(&mut self, rank: u32, name: String, rating: u32) {
        let ratings = &mut self.skill_ratings;
        let rank = (rank - 1) as usize;

        if name.eq("Null") { return; }

        match ratings.iter_mut().find(|sr| sr.name.eq(&name)) {
            Some(sr) => {
                sr.ratings[rank] = rating;
            }
            None => {
                assert_ne!(ratings.len(), 3);
                let mut sr = SkillRating { name: name, ratings: [0; 32]};
                sr.ratings[rank] = rating;
                ratings.push(sr);
            }
        }
    }

    fn fill_talent(&mut self, name: String, rank: u32, description: String, cost: String, type_: String) {
        let talents = &mut self.talents;
        
        talents.push(Talent { name, rank, description, cost, type_});
        talents.sort_by(|a, b| {
            if a.rank == b.rank {
                a.name.cmp(&b.name)
            } else {
                a.rank.cmp(&b.rank)
            }
        });
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="PascalCase")]
struct TalentRecord {
    name: String,
    rank: u32,
    description: String,
    #[serde(rename="Cooldown")] cost: String,
    job: String,
    type_: String
}

fn load_job_talents<T: std::io::Read>(rdr: T, map: &mut JobMap) -> Result<usize, csv::Error> {
    let mut rdr = make_csv_reader(rdr);

    for result in rdr.deserialize() {
        let record: TalentRecord = result?;
        let job = find_job(map, &record.job);

        job.fill_talent(record.name, record.rank, record.description, record.cost, record.type_);
    }

    Result::Ok(map.len())
}

#[derive(Debug, Deserialize)]
struct JobSkillRecord {
    #[serde(rename = "Rank")]   rank: u32,
    #[serde(rename = "1-Name")] name1: String,
    #[serde(rename = "1-Num")]  rating1: u32,
    #[serde(rename = "2-Name")] name2: String,
    #[serde(rename = "2-Num")]  rating2: u32,
    #[serde(rename = "3-Name")] name3: String,
    #[serde(rename = "3-Num")]  rating3: u32,
    #[serde(rename = "Job")]    job_name: String
}

fn load_job_skills<T: std::io::Read>(rdr: T, map: &mut JobMap) -> Result<usize, csv::Error> {
    let mut rdr = make_csv_reader(rdr);

    for result in rdr.deserialize() {
        let record: JobSkillRecord = result?;
        let job = find_job(map, &record.job_name);

        job.fill_rating(record.rank, record.name1, record.rating1);
        job.fill_rating(record.rank, record.name2, record.rating2);
        if record.name3.eq("Null") { continue; }
        job.fill_rating(record.rank, record.name3, record.rating3);
    }

    Result::Ok(map.len())
}

pub fn load_jobs(skill_ratings_file: &str, talents_file: &str) -> Result<JobMap, std::io::Error> {
    let skills_path = Path::new(skill_ratings_file);
    let talents_path = Path::new(talents_file);

    let mut jobs = JobMap::new();
    println!("Loading skill ratings from {}", skill_ratings_file);
    load_job_skills(File::open(&skills_path)?, &mut jobs)?;
    println!("Loading talents from {}", talents_file);
    load_job_talents(File::open(&talents_path)?, &mut jobs)?;

    Result::Ok(jobs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SKILLS_CSV: &str = 
"Rank:1-Name:1-Num:2-Name:2-Num:3-Name:3-Num:Job
1:Blades:2:Evasion:1:Stealth:1:Assassin
2:Blades:3:Evasion:1:Stealth:3:Assassin
3:Blades:4:Evasion:1:Stealth:3:Assassin
4:Blades:4:Evasion:1:Stealth:4:Assassin
5:Blades:5:Evasion:1:Stealth:4:Assassin
6:Blades:5:Evasion:1:Stealth:5:Assassin
7:Blades:5:Evasion:1:Stealth:6:Assassin
8:Blades:6:Evasion:1:Stealth:6:Assassin
9:Blades:6:Evasion:1:Stealth:7:Assassin
1:Pistols:2:Doctor:2:Null:0:Combat Medic
2:Pistols:3:Doctor:4:Null:0:Combat Medic
3:Pistols:3:Doctor:5:Null:0:Combat Medic
4:Pistols:4:Doctor:5:Null:0:Combat Medic
5:Pistols:4:Doctor:6:Null:0:Combat Medic
6:Pistols:4:Doctor:7:Null:0:Combat Medic
7:Pistols:4:Doctor:8:Null:0:Combat Medic";

    #[test]
    fn test_load_skills() {
        let csv = TEST_SKILLS_CSV.as_bytes();
        let mut j = JobMap::new();

        load_job_skills(csv, &mut j).unwrap();

        let aj = &j["Assassin"];
        let cmj = &j["Combat Medic"];
        assert_eq!(aj.name, ("Assassin"));
        assert_eq!(aj.skill_ratings.len(), 3);

        let aj_blades = aj.skill_ratings.iter().find(|sr| sr.name.eq("Blades")).unwrap();
        assert_eq!(aj_blades.ratings[2], 4);
        assert_eq!(aj_blades.ratings[8], 6);

        assert_eq!(cmj.skill_ratings.len(), 2);
    }
}