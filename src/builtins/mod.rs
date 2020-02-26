use crate::process::ExitStatus;
use crate::shell::Shell;
use crate::utils::FdFile;
use std::collections::BTreeMap;
use std::io::Write;

mod alias;
mod bg;
mod cd;
mod echo;
mod eval;
mod exit;
mod export;
mod fg;
mod jobs;
mod popd;
mod pushd;
mod read;
mod set;
mod shift;
mod source;
mod unset;
mod wait;

pub struct InternalCommandContext<'a> {
    pub argv: &'a [String],
    pub shell: &'a mut Shell,
    pub stdin: FdFile,
    pub stdout: FdFile,
    pub stderr: FdFile,
}

#[derive(Debug, Fail)]
pub enum InternalCommandError {
    #[fail(display = "command not found")]
    NotFound,
    #[fail(display = "failed to create redirections")]
    BadRedirection,
}

/// A super powerful hidden command for some cryptographers.
/// https://xkcd.com/221/
pub fn xkcd_rand_command(ctx: &mut InternalCommandContext) -> ExitStatus {
    writeln!(ctx.stdout, "4").ok();
    ctx.stdout.flush().ok();
    ExitStatus::ExitedWith(0)
}

type InternalCommand = fn(&mut InternalCommandContext) -> ExitStatus;
lazy_static! {
    // TODO: Construct this map in compile time.
    pub static ref INTERNAL_COMMANDS: BTreeMap<&'static str, InternalCommand> = {
        let mut commands: BTreeMap<&'static str, InternalCommand> = BTreeMap::new();
        commands.insert("xkcd-true-random-number", xkcd_rand_command);
        commands.insert("alias", crate::builtins::alias::command);
        commands.insert("echo", crate::builtins::echo::command);
        commands.insert("cd", crate::builtins::cd::command);
        commands.insert("source", crate::builtins::source::command);
        commands.insert("exit", crate::builtins::exit::command);
        commands.insert("export", crate::builtins::export::command);
        commands.insert("set", crate::builtins::set::command);
        commands.insert("fg", crate::builtins::fg::command);
        commands.insert("bg", crate::builtins::bg::command);
        commands.insert("wait", crate::builtins::wait::command);
        commands.insert("jobs", crate::builtins::jobs::command);
        commands.insert("shift", crate::builtins::shift::command);
        commands.insert("read", crate::builtins::read::command);
        commands.insert("unset", crate::builtins::unset::command);
        commands.insert("pushd", crate::builtins::pushd::command);
        commands.insert("popd", crate::builtins::popd::command);
        commands.insert("eval", crate::builtins::eval::command);
        commands
    };
}
