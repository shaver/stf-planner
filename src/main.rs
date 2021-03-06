extern crate csv;
#[macro_use]
extern crate serde_derive;

mod jobs;
mod ships;

fn main() {
    extern crate clap;
    use clap::{App, Arg, SubCommand};

    let matches = App::new("stf-data")
        .subcommand(
            SubCommand::with_name("jobs")
                .about("Generate jobs as JSON from CSV")
                .arg(
                    Arg::with_name("skills")
                        .required(true)
                        .help("job skills")
                        .default_value("csv/job_skills.csv"),
                )
                .arg(
                    Arg::with_name("talents")
                        .required(true)
                        .help("job talents")
                        .default_value("csv/job_talents.csv"),
                )
                .arg(
                    Arg::with_name("json")
                        .required(true)
                        .help("JSON output file")
                        .default_value("json/jobs.json"),
                ),
        )
        .version("0.1")
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("jobs") {
            let skills = matches.value_of("skills").unwrap();
            let talents = matches.value_of("talents").unwrap();
            let _json = matches.value_of("json").unwrap();

        match jobs::load_jobs(skills, talents) {
            Err(e) => {
                    println!("Damn: {:?}", e);
            }
            Ok(jobs) => {
                println!("loaded some jobs, here's the first:");
                println!("{:?}", jobs.iter().next());
             }
        };
    }
}
