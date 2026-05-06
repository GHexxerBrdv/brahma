use crate::generator::commands::run_command;
use std::fs;
use std::path::Path;

fn init_git(project_path: &str) -> std::io::Result<()> {
    run_command("git", &["init"], project_path)?;
    Ok(())
}

fn install_dependencies(project_path: &str) -> std::io::Result<()> {
    run_command("npm", &["install", "express"], project_path)?;
    Ok(())
}

pub fn create_project(name: &str) -> std::io::Result<()> {
    let path = Path::new(name);
    if path.exists() {
        println!("Project already exists");
        return Ok(());
    }

    fs::create_dir(path)?;
    fs::create_dir(path.join("src"))?;

    super::templates::generate_all(name)?;
    init_git(name)?;
    install_dependencies(name)?;

    println!("Project {} ready", name);

    Ok(())
}
