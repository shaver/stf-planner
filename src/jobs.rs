pub struct SkillRating {
    _name: String,          // Name of the skill
    _ratings: Vec<u32>      // Ratings at levels 1 through 32
}

pub struct Job {
    _name:          String,
    _skill_ratings: (SkillRating, SkillRating, SkillRating),
}

pub fn load_jobs(_file: &str) -> Vec<Job> {
    extern crate csv;
    let jobs = vec![Job {
                    _name: "Dude".to_string(),
                    _skill_ratings: (
                        SkillRating {
                            _name: String::from("Drinking"),
                            _ratings: vec![0; 32] 
                        },
                        SkillRating {
                            _name: String::from("Eating"),
                            _ratings: vec![0; 32] 

                        },
                        SkillRating {
                            _name: String::from("Sleeping"),
                            _ratings: vec![0; 32] 
                        }

                    )
                }];
    jobs
}
