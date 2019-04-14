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
