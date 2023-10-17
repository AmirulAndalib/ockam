use clap::Args;
use colorful::Colorful;

use ockam_node::Context;

use crate::node::util::delete_all_nodes;
use crate::util::node_rpc;
use crate::{docs, fmt_ok, CommandGlobalOpts};

const LONG_ABOUT: &str = include_str!("./static/delete/long_about.txt");
const AFTER_LONG_HELP: &str = include_str!("./static/delete/after_long_help.txt");

/// Delete nodes
#[derive(Clone, Debug, Args)]
#[command(
long_about = docs::about(LONG_ABOUT),
after_long_help = docs::after_help(AFTER_LONG_HELP)
)]
pub struct DeleteCommand {
    /// Name of the node to be deleted
    #[arg(group = "nodes")]
    node_name: Option<String>,

    /// Terminate all node processes and delete all node configurations
    #[arg(long, short, group = "nodes")]
    all: bool,

    /// Terminate node process(es) immediately (uses SIGKILL instead of SIGTERM)
    #[arg(display_order = 901, long, short)]
    force: bool,

    /// Confirm the deletion without prompting
    #[arg(display_order = 901, long, short)]
    yes: bool,
}

impl DeleteCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        node_rpc(run_impl, (opts, self));
    }
}

async fn run_impl(
    _ctx: Context,
    (opts, cmd): (CommandGlobalOpts, DeleteCommand),
) -> miette::Result<()> {
    let prompt_msg = if cmd.all {
        "Are you sure you want to delete all nodes?"
    } else {
        "Are you sure you want to delete this node?"
    };
    if opts
        .terminal
        .confirmed_with_flag_or_prompt(cmd.yes, prompt_msg)?
    {
        if cmd.all {
            delete_all_nodes(&opts, cmd.force).await?;
            opts.terminal
                .stdout()
                .plain(fmt_ok!("All nodes have been deleted"))
                .write_line()?;
        } else {
            let node_name = opts.state.get_node_name_or_default(&cmd.node_name).await?;
            opts.state
                .delete_node_sigkill(&node_name, cmd.force)
                .await?;
            opts.terminal
                .stdout()
                .plain(fmt_ok!("Node with name '{}' has been deleted", &node_name))
                .machine(&node_name)
                .json(serde_json::json!({ "node": { "name": &node_name } }))
                .write_line()?;
        }
    }
    Ok(())
}
