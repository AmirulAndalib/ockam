use clap::{Args, Subcommand};

pub use enroll::EnrollCommand;
pub use import::ImportCommand;
pub use info::InfoCommand;
pub use list::ListCommand;
pub use show::ShowCommand;
pub use version::VersionCommand;

use crate::{docs, Command, CommandGlobalOpts};

mod addon;
mod create;
mod delete;
pub(crate) mod enroll;
mod import;
mod info;
mod list;
mod show;
mod ticket;
#[allow(unused)]
pub mod util;
mod version;

pub use addon::AddonCommand;
pub use create::CreateCommand;
pub use delete::DeleteCommand;
pub use ticket::TicketCommand;

use ockam_node::Context;

const LONG_ABOUT: &str = include_str!("./static/long_about.txt");

#[derive(Clone, Debug, Args)]
#[command(
arg_required_else_help = true,
subcommand_required = true,
about = docs::about("Manage Projects in Ockam Orchestrator"),
long_about = docs::about(LONG_ABOUT),
)]
pub struct ProjectCommand {
    #[command(subcommand)]
    pub subcommand: ProjectSubcommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ProjectSubcommand {
    Enroll(EnrollCommand),
    Import(ImportCommand),
    List(ListCommand),
    Show(ShowCommand),
    Version(VersionCommand),
    Information(InfoCommand),
    Ticket(TicketCommand),
    Create(CreateCommand),
    Delete(DeleteCommand),
    Addon(AddonCommand),
}

impl ProjectCommand {
    pub async fn run(self, ctx: &Context, opts: CommandGlobalOpts) -> miette::Result<()> {
        match self.subcommand {
            ProjectSubcommand::Enroll(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Import(c) => c.run(opts).await,
            ProjectSubcommand::List(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Show(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Version(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Information(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Ticket(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Create(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Delete(c) => c.run(ctx, opts).await,
            ProjectSubcommand::Addon(c) => c.run(ctx, opts).await,
        }
    }

    pub fn name(&self) -> String {
        match &self.subcommand {
            ProjectSubcommand::Enroll(c) => c.name(),
            ProjectSubcommand::Import(c) => c.name(),
            ProjectSubcommand::List(c) => c.name(),
            ProjectSubcommand::Show(c) => c.name(),
            ProjectSubcommand::Version(c) => c.name(),
            ProjectSubcommand::Information(c) => c.name(),
            ProjectSubcommand::Ticket(c) => c.name(),
            ProjectSubcommand::Create(c) => c.name(),
            ProjectSubcommand::Delete(c) => c.name(),
            ProjectSubcommand::Addon(c) => c.name(),
        }
    }
}
