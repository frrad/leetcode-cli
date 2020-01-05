//! test command
use super::Command;
use clap::{App, ArgMatches};

//// Test Command
pub struct TestCommand;

impl Command for TestCommand {
    /// `test` usage
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{SubCommand, Arg};
        SubCommand::with_name("test")
            .about("Edit question by id")
            .visible_alias("t")
            .arg(Arg::with_name("id")
                 .takes_value(true)
                 .required(true)
                 .help("question id")
            )
    }

    /// `test` handler
    fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Run, Cache};
        let id: i32 = m.value_of("id")?.parse()?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Test)?;

        println!("{}", res);
        Ok(())
    }
}
