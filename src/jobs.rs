pub struct Job<'a> {
    name:          &'a str,
    skill_names:   (&'a str, &'a str, &'a str),
    skill_ratings: Vec<(u32, u32, u32)>
}

pub fn load_jobs(file: &str) -> Vec<Job> {
    let jobs = vec![Job {
                    name: "Dude",
                    skill_names: ("Cooking", "Drinking", "Sleeping"),
                    skill_ratings: vec![(1,1,1)]
                }];
    jobs
}
