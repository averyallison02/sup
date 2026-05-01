mod jobs;

use std::path::{Path, PathBuf};

use crate::jobs::Job;
use crate::jobs::io::get_jobs;

fn main() -> Result<(), anyhow::Error>
{
    let filename: PathBuf = PathBuf::from("jobsfile");
    let jobs: Vec<Job> = get_jobs(&filename)?;

    Ok(())
}
