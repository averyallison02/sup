mod io;

use std::path::PathBuf;

/**
 * A named group of `Task`s.
 * `depends`: A vector to the names of jobs which must run before self.
 */
pub struct Job
{
    name: String,
    tasks: Vec<Task>,
    depends: Option<Vec<String>>,
}

/**
 * A component of a `Job` representing a program and its arguments.
 * `runner`: A path to the program to run this task.
 * `args`: Program arguments, including the argument used to run the task itself.
 */
struct Task
{
    runner: PathBuf,
    args: Vec<String>,
}
