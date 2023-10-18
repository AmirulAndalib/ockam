use clap::Args;
use miette::IntoDiagnostic;
use ockam_node::Context;

use crate::util::node_rpc;
use crate::{docs, CommandGlobalOpts};

const LONG_ABOUT: &str = include_str!("./static/list/long_about.txt");
const PREVIEW_TAG: &str = include_str!("../static/preview_tag.txt");
const AFTER_LONG_HELP: &str = include_str!("./static/list/after_long_help.txt");

/// List vaults
#[derive(Clone, Debug, Args)]
#[command(
    long_about = docs::about(LONG_ABOUT),
    before_help = docs::before_help(PREVIEW_TAG),
    after_long_help = docs::after_help(AFTER_LONG_HELP)
)]
pub struct ListCommand;

impl ListCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        node_rpc(run_impl, opts);
    }
}

async fn run_impl(_ctx: Context, opts: CommandGlobalOpts) -> miette::Result<()> {
    let vaults = opts.state.get_named_vaults().await?;
    let plain = opts
        .terminal
        .build_list(&vaults, "Vaults", "No vaults found on this system.")?;

    let json = serde_json::to_string_pretty(&vaults).into_diagnostic()?;

    opts.terminal
        .stdout()
        .plain(plain)
        .json(json)
        .write_line()?;

    Ok(())
}
