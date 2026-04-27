use std::fmt;
use std::path::Path;
use super::Job;

use JobIOError::*;
enum JobIOError
{
    CircularDependency(String, String),
    UnexpectedToken(String, u32),
}

impl fmt::Display for JobIOError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            CircularDependency(dep1, dep2) => write!(f, "jobs {} and {} depend on each other", dep1, dep2),
            UnexpectedToken(token, line) => write!(f, "unexpected token {} on line {}", token, line)
        }
    }
}
