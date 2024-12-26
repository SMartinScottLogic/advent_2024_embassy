use anyhow::{Context, Result as AnyResult};
use std::{env, path::PathBuf};

fn main() -> AnyResult<()> {
    eprintln!("=-=-=-=-= No documentation as-yet for 'setup'");
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("setup") => setup(),
        _ => xtaskops::tasks::main(),
    }
}

fn setup() -> AnyResult<()> {
    let entry = env::args().nth(2).context("require new workspace entry")?;

    // Copy template to new directory
    println!("* copy 'template' to '{entry}'");
    xtaskops::ops::copy_contents("template", &entry, false)?;
    // Replace 'template' with entry name in Cargo.toml, src/main.rs
    for filename in ["Cargo.toml", "src/lib.rs"]
        .iter()
        .map(|name| PathBuf::from(&entry).join(name))
    {
        println!("* setup {filename:?}");
        let contents = std::fs::read_to_string(&filename)?;
        let contents = contents.replace("template", &entry);
        std::fs::write(filename, contents)?;
    }

    // Patch runner
    let runner_filename = PathBuf::from(&entry).join("examples").join("runner.rs");
    let runner_contents = std::fs::read_to_string(&runner_filename)?.replace("template", &entry);
    std::fs::write(&runner_filename, runner_contents)?;

    // Add entry to workspace
    println!("* setup Cargo.toml");

    let workspace_toml_contents = std::fs::read_to_string("Cargo.toml")?;
    let mut toml = workspace_toml_contents.parse::<toml_edit::DocumentMut>()?;
    toml["workspace"]["members"]
        .as_array_mut()
        .context("read workspace members")?
        .push(&entry);
    std::fs::write("Cargo.toml", toml.to_string())?;

    // // Setup launcher
    // println!("* setup launcher");

    // let launcher_toml_path = PathBuf::from(".").join("local_runner").join("Cargo.toml");
    // let launcher_toml_contents = std::fs::read_to_string(&launcher_toml_path)?;
    // let mut toml = launcher_toml_contents.parse::<toml_edit::DocumentMut>()?;
    // toml["dependencies"]
    //     .as_table_mut()
    //     .context("read local runner dependencies")?
    //     .entry(&entry)
    //     .or_insert_with(|| {
    //         let mut table = toml_edit::table();
    //         table
    //             .as_table_mut()
    //             .context("add dependency")
    //             .unwrap()
    //             .entry("path")
    //             .or_insert(format!("../{}", entry).into());
    //         table
    //     });

    // std::fs::write(launcher_toml_path, toml.to_string())?;
    Ok(())
}
