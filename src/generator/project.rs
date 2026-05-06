use crate::generator::commands::run_command;
use std::fs;
use std::path::Path;

fn init_git(project_path: &str) -> std::io::Result<()> {
    run_command("git", &["init"], project_path)?;
    Ok(())
}

fn create_git_ignore(project_path: &str) -> std::io::Result<()> {
    let content = fs::read_to_string("templates/express/gitignore")?;
    fs::write(format!("{}/.gitignore", project_path), content)?;
    Ok(())
}

fn install_dependencies(project_path: &str) -> std::io::Result<()> {
    run_command("npm", &["install", "express"], project_path)?;
    Ok(())
}

pub fn create_project(name: &str) -> std::io::Result<()> {
    let project_name = name;

    let path = Path::new(name);
    if path.exists() {
        println!("Project already exists");
        return Ok(());
    }

    fs::create_dir(path)?;
    fs::create_dir(path.join("src"))?;

    super::templates::generate_all(project_name)?;
    init_git(project_name)?;
    create_git_ignore(project_name)?;
    install_dependencies(project_name)?;

    println!("Project {} ready", project_name);

    Ok(())
}
