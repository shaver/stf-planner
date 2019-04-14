/*
    TODO:
    - test from string CSV
    - validate SkillRatings
    - print all
    - JSON
    - move to lib?
    - load talents
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
pub struct Job {
    pub name: String,
    pub skill_ratings: Vec<SkillRating>
}

pub type JobMap = HashMap<String, Job>;

impl Job {
    fn fill_rating(&mut self, rank: u32, name: String, rating: u32) {
        let ratings = &mut self.skill_ratings;
        let rank = (rank - 1) as usize;

        if name.eq("Null") { return; }

        match ratings.iter_mut().find(|sr| sr.name.eq(&name)) {
            Some(sr) => {
                sr.ratings[rank] = rating;
                println!("setting {}:{}[{}] to {}", &self.name, &sr.name, rank, rating);
            }
            None => {
                assert_ne!(ratings.len(), 3);
                let mut sr = SkillRating { name: name, ratings: [0; 32]};
                sr.ratings[rank] = rating;
                ratings.push(sr);
            }
        }
    }
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

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b':')
        .from_reader(rdr);

    for result in rdr.deserialize() {
        let record: JobSkillRecord = result?;
        let name = &record.job_name;
        let job = map.entry(name.clone())
            .or_insert(Job { name: name.clone(), ..Default::default()});

        job.fill_rating(record.rank, record.name1, record.rating1);
        job.fill_rating(record.rank, record.name2, record.rating2);
        if record.name3.eq("Null") { continue; }
        job.fill_rating(record.rank, record.name3, record.rating3);
    }

    Result::Ok(map.len())
}

pub fn load_jobs(skill_ratings_file: &str) -> Result<JobMap, std::io::Error> {

    let path = Path::new(skill_ratings_file);

    let mut jobs = JobMap::new();
    load_job_skills(File::open(&path)?, &mut jobs)?;

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