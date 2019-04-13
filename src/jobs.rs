use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default)]
pub struct SkillRating {
    pub name: String,          // Name of the skill
    pub ratings: Vec<u32>      // Ratings at levels 1 through 32
}

#[derive(Debug, Default)]
pub struct Job {
    pub name: String,
    pub skill_ratings: [SkillRating; 3]
}

pub type JobMap = HashMap<String, Job>;

impl Job {
}

#[derive(Debug, Deserialize)]
struct JobSkillRecord {
    #[serde(rename = "Rank")]
    rank: u32,

    #[serde(rename = "1-Name")]
    name1: String,

    #[serde(rename = "1-Num")]
    rating1: u32,

    #[serde(rename = "2-Name")]
    name2: String,

    #[serde(rename = "2-Num")]
    rating2: u32,

    #[serde(rename = "3-Name")]
    name3: String,

    #[serde(rename = "3-Num")]
    rating3: u32,

    #[serde(rename = "Job")]
    job_name: String
}

fn load_job_skills<T: std::io::Read>(rdr: T, map: &mut JobMap) -> Result<usize, csv::Error> {

    const SINGLE_JOB : &str = "Commander";
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b':')
        .from_reader(rdr);

    for result in rdr.deserialize() {
        let record: JobSkillRecord = result?;
        let name = &record.job_name;
        if name == SINGLE_JOB {
            let mut _job = map.entry(name.clone()).or_insert(Job { name: name.clone(), ..Default::default()});
            println!("{:?}", record);
        }
    }

    Result::Ok(map.len())
}

pub fn load_jobs(skill_ratings_file: &str) -> Result<JobMap, std::io::Error> {

    let path = Path::new(skill_ratings_file);

    let mut jobs = JobMap::new();
    load_job_skills(File::open(&path)?, &mut jobs)?;

/*
    for result in rdr.records() {
        let record = result?;
        println!("record: {:?}", record);
    }
*/

    Result::Ok(jobs)
}
