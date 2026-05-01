use super::Job;

use JobIOError::*;

#[derive(Debug)]
pub enum JobIOError
{
    CircularDependency(String, String),
    DependencyNotFound(String, String),
    NotImplemented(&'static str),
    UnexpectedToken(String, u32),
}

impl std::error::Error for JobIOError {}

impl std::fmt::Display for JobIOError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            CircularDependency(dep1, dep2) => write!(f, "jobs {} and {} depend on each other", dep1, dep2),
            DependencyNotFound(dependent, provider) => write!(f, "job {} requested {}, but it was not found.", dependent, provider),
            NotImplemented(desc) => write!(f, "{} has not been implemented", desc),
            UnexpectedToken(token, line) => write!(f, "unexpected token {} on line {}", token, line)
        }
    }
}

pub fn get_jobs(filename: &std::path::Path) -> Result<Vec<Job>, JobIOError>
{
    Err(NotImplemented("get_jobs"))
}
