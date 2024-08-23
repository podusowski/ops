mod plan;
mod run;

use clap::{Parser, Subcommand};
use plan::Plan;

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
    let plan = Plan::from_file("cio.yaml")?;

    match cli.command {
        Commands::Execute => execute(plan),
        Commands::Shell => shell(plan),
    }
}

fn execute(plan: Plan) -> anyhow::Result<()> {
    let mut absolute_success = true;

    for (name, mission) in plan.missions {
        println!("Launching '{}'", name);
        let status = run::execute(mission)?;

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

fn shell(plan: Plan) -> anyhow::Result<()> {
    run::shell(plan.shell)?;
    Ok(())
}
