mod io;

use std::path::PathBuf;

/**
 * A named group of `Task`s.
 */
pub struct Job
{
    name: String,
    tasks: Vec<Task>,
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
