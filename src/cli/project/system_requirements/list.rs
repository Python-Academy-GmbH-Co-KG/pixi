use crate::project::Environment;
use crate::Project;
use clap::Parser;
use fancy_display::FancyDisplay;
use miette::IntoDiagnostic;
use pixi_manifest::{EnvironmentName, SystemRequirements};
use serde::Serialize;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    pub json: bool,

    #[clap(long, short)]
    pub environment: Option<EnvironmentName>,
}

#[derive(Serialize)]
pub struct EnvironmentDisplay {
    name: EnvironmentName,
    system_requirements: SystemRequirements,
}

impl<'a> From<&'a Environment<'a>> for EnvironmentDisplay {
    fn from(env: &'a Environment<'a>) -> Self {
        Self {
            name: env.name().clone(),
            system_requirements: env.system_requirements().clone(),
        }
    }
}

impl std::fmt::Display for EnvironmentDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            console::style("Environment:").bold().bright(),
            self.name.fancy_display()
        )?;
        write!(f, "{}", self.system_requirements)
    }
}

pub(crate) fn execute(project: &Project, args: Args) -> miette::Result<()> {
    let environments: Vec<EnvironmentDisplay> = if let Some(env_name) = args.environment {
        let result: Vec<_> = project
            .environment(&env_name)
            .iter()
            .map(EnvironmentDisplay::from)
            .collect();
        if result.is_empty() {
            miette::bail!("Environment not found: {}", env_name);
        }
        result
    } else {
        project
            .environments()
            .iter()
            .map(EnvironmentDisplay::from)
            .collect()
    };

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&environments).into_diagnostic()?
        );
    } else {
        for env in &environments {
            println!("{}", env);
        }
    }
    Ok(())
}
