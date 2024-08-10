mod plan;
mod run;

use anyhow::Context;
use plan::Plan;
use run::run_in_docker;

fn main() -> anyhow::Result<()> {
    let recipe = Plan::from_file("cio.yaml").with_context(|| "could not load cio.yaml")?;
    println!("{:#?}", recipe);

    for (name, mission) in recipe.missions {
        println!("Launching '{}'", name);
        let status = run_in_docker(mission)?;

        if !status.success() {
            eprintln!("Task failed with status: {:?}", status.code());
        }
    }

    Ok(())
}
