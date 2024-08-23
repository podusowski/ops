mod plan;
mod run;

use clap::{Parser, Subcommand};
use plan::Plan;
use run::run_in_docker;

#[derive(Parser)]
#[command(version, about, long_about = None, infer_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter the shell.
    Shell,

    /// Execute all missions.
    Execute,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Execute => execute(),
        Commands::Shell => shell(),
    }
}

fn execute() -> anyhow::Result<()> {
    let recipe = Plan::from_file("cio.yaml")?;
    let mut absolute_success = true;

    for (name, mission) in recipe.missions {
        println!("Launching '{}'", name);
        let status = run_in_docker(mission)?;

        if !status.success() {
            println!("Mission '{}' failed with status: {:?}", name, status.code());
            absolute_success = false;
        }
    }

    if absolute_success {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Some missions have failed."))
    }
}

fn shell() -> anyhow::Result<()> {
    let plan = Plan::from_file("cio.yaml")?;
    run::shell(plan.shell)?;
    Ok(())
}
