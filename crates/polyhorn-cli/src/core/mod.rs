//! Types and functions that are shared between platform-specific
//! implementations of Polyhorn CLI commands.

use ansi_term::Colour::{Cyan, Fixed, Green};
use indicatif::{ProgressBar, ProgressStyle};

mod cargo_build;
mod cargo_rustc;
mod change_crate_type;
mod rasterize;
pub mod tasks;

pub use cargo_build::CargoBuild;
pub use cargo_rustc::CargoRustc;
pub use change_crate_type::change_crate_type;
pub use rasterize::rasterize;

/// Represents an individual task that a CLI command is composed of.
pub trait Task {
    /// The type of context that is passed to this task, processed and
    /// subsequently returned by this task.
    type Context;

    /// The type of error that this task can return.
    type Error;

    /// The verb that describes this task (e.g. "Launching" or "Building") that
    /// is shown to the user while the task is running.
    fn verb(&self) -> &str;

    /// The message that is shown to the user alongside the verb. This usually
    /// starts with a lowercase letter (e.g. "[Generating] source tree").
    fn message(&self) -> &str;

    /// Optional additional text that is shown to the user alongside the
    /// message. This usually starts with a lowercase letter too (e.g.
    // "[Generating] [source tree] for Android").
    fn detail(&self) -> &str;

    /// This function should execute the task.
    fn run(
        &self,
        context: Self::Context,
        manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error>;
}

/// Manager that can provide additional utilities (e.g. progress tracking) to
/// tasks.
pub struct Manager<'a> {
    verb: &'a str,
    message: &'a str,
    detail: &'a str,
}

impl<'a> Manager<'a> {
    /// Creates a new manager for the given task.
    pub fn new<T>(task: &'a T) -> Manager<'a>
    where
        T: Task + ?Sized,
    {
        eprint!(
            "{} {} {}",
            Cyan.bold().paint(format!("{:>12}", task.verb())),
            task.message(),
            Fixed(8).paint(task.detail())
        );

        Manager {
            verb: task.verb(),
            message: task.message(),
            detail: task.detail(),
        }
    }

    /// Returns a progress bar for the task that corresponds to this manager.
    pub fn progress_bar(&mut self, len: usize) -> ProgressBar {
        let bar = ProgressBar::new(len as u64);

        eprint!("\r");

        bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{} [{{bar:57}}] {{pos:>{}}}/{{len}}: {} {}",
                    Cyan.bold().paint(format!("{:>12}", self.verb)),
                    1000.0f32.log10().ceil() as usize,
                    self.message,
                    Fixed(8).paint(self.detail)
                ))
                .progress_chars("=> "),
        );

        bar
    }
}

impl<'a> Drop for Manager<'a> {
    fn drop(&mut self) {
        eprintln!(
            "\r{} {} {} {}",
            Green.bold().paint(format!("    Finished")),
            self.verb,
            self.message,
            Fixed(8).paint(self.detail)
        );
    }
}

/// Executioner that manes the execution of a sequence of a tasks.
pub struct Executioner;

impl Executioner {
    /// Executes the given sequence of tasks with the given initial context. The
    /// first task receives the initial context. Each subsequent task receives
    /// the input from the previous task. This function will return the
    /// resulting context of the last task.
    pub fn execute<T>(tasks: &[T], context: T::Context) -> Result<T::Context, T::Error>
    where
        T: Task,
    {
        let mut context = context;

        for task in tasks {
            let mut manager = Manager::new(task);
            context = task.run(context, &mut manager)?;
        }

        Ok(context)
    }
}
