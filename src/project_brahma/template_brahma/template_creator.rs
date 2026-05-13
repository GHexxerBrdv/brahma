use crate::errors::{Context, Result};
use include_dir::{Dir, DirEntry, include_dir};
use std::fs::{create_dir_all, write};
use std::path::Path;

pub static TEMPLATES_DIR: Dir<'_> = include_dir!("templates");

pub fn create_template(template_path: &str, project_name: &str) -> Result<()> {
    let project_root = Path::new(project_name);
    create_dir_all(project_root).context("Failed to create project directory")?;

    let dir = TEMPLATES_DIR
        .get_dir(template_path)
        .context("Template directory not found")?;

    let project_name = project_root
        .file_name()
        .and_then(|s| s.to_str())
        .context("Invalid project name")?;

    create_template_recursive(dir, template_path, project_root, project_name)?;

    Ok(())
}

fn create_template_recursive(
    dir: &Dir<'_>,
    template_path: &str,
    project_root: &Path,
    project_name: &str,
) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(subdir) => {
                create_template_recursive(subdir, template_path, project_root, project_name)?;
            }
            DirEntry::File(file) => {
                let content = file
                    .contents_utf8()
                    .context("Template is not valid UTF-8")?;
                let rendered = content.replace("{{project_name}}", project_name);

                let relative_path = file.path().strip_prefix(template_path).unwrap();
                let dest_path = project_root.join(relative_path);

                if let Some(parent) = dest_path.parent() {
                    create_dir_all(parent).context("Creating parent directory failed")?;
                }
                write(dest_path, rendered).context("Writing to file failed")?;
            }
        }
    }
    Ok(())
}
