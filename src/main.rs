use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use path_macro::path;
use std::{
    fs,
    os::unix,
    path::{Path, PathBuf},
    process::Command,
};

mod files;
use files::*;

const TARGET_DIR: &str = "target";
const RELEASE_DIR: &str = "target/release";
const DEBUG_DIR: &str = "target/debug";

#[derive(Parser)]
#[command(author, version)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Create a new carton project at a specified path
    New {
        /// Use a binary (application) template [default]
        #[arg(long, default_value_t = false)]
        bin: bool,

        /// Use a library template
        #[arg(long, default_value_t = false)]
        lib: bool,

        /// Path to create carton project at
        #[arg()]
        path: PathBuf,
    },
    /// Compile a local carton project
    Build {
        /// Build artifacts in release mode, with optimizations
        #[arg(long, default_value_t = false)]
        release: bool,
    },
    /// Execute all unit tests of a local carton project
    Test,
    /// Remove artifacts that carton has generated in the past
    Clean,
}

enum Template {
    Bin,
    Lib,
}

enum Profile {
    Debug,
    Release,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        Subcommands::New { bin, lib, path } => {
            let template = match (bin, lib) {
                (false, false) => Template::Bin,
                (true, false) => Template::Bin,
                (false, true) => Template::Lib,
                (true, true) => return Err(anyhow!("can't specify both library and binary")),
            };
            new(template, path)
        }
        Subcommands::Build { release } => {
            let profile = if release {
                Profile::Release
            } else {
                Profile::Debug
            };
            build(profile)
        }
        Subcommands::Test => test(),
        Subcommands::Clean => clean(),
    }?;

    Ok(())
}

fn save<P>(project: &str, path: P, contents: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    fs::write(path, contents.trim().replace("<PROJECT>", project))?;
    Ok(())
}

fn new(template: Template, path: PathBuf) -> Result<()> {
    if path.exists() {
        return Err(anyhow!("{} already exists", path.as_path().display()));
    }

    fs::create_dir_all(&path)?;

    let project = path
        .file_name()
        .ok_or(anyhow!("bad path: {}", path.as_path().display()))?
        .to_str()
        .ok_or(anyhow!("bad path: {}", path.as_path().display()))?;
    let project_build_path = path![&path / "meson.build"];
    let source_dir = path![&path / "src"];
    let source_build_path = path![&source_dir / "meson.build"];
    let (project_contents, source_contents, source_template, source_template_path) = match template
    {
        Template::Bin => (
            PROJECT_BIN_BUILD,
            SOURCE_BIN_BUILD,
            SOURCE_BIN_TEMPLATE,
            path![&source_dir / "main.c"],
        ),
        Template::Lib => (
            PROJECT_LIB_BUILD,
            SOURCE_LIB_BUILD,
            SOURCE_LIB_TEMPLATE,
            path![&source_dir / "lib.c"],
        ),
    };

    fs::create_dir_all(&source_dir)?;
    save(project, project_build_path, project_contents)?;
    save(project, source_build_path, source_contents)?;
    save(project, source_template_path, source_template)?;

    if matches!(template, Template::Lib) {
        let include_dir = path![&path / "include"];
        let include_build = path![&include_dir / "meson.build"];
        let include_template = path![&include_dir / &format!("{project}.h")];
        fs::create_dir_all(&include_dir)?;
        save(project, include_build, INCLUDE_BUILD)?;
        save(project, include_template, INCLUDE_TEMPLATE)?;

        let test_dir = path![&path / "tests"];
        let test_build = path![&test_dir / "meson.build"];
        let test_template = path![&test_dir / "test.c"];
        fs::create_dir_all(&test_dir)?;
        save(project, test_build, TEST_BUILD)?;
        save(project, test_template, TEST_TEMPLATE)?;
    }

    Ok(())
}

fn build(profile: Profile) -> Result<()> {
    if fs::metadata("meson.build").is_err() {
        return Err(anyhow!("not in a project"));
    }

    let (target, buildtype) = match profile {
        Profile::Debug => (DEBUG_DIR, "debug"),
        Profile::Release => (RELEASE_DIR, "release"),
    };

    if fs::metadata(target).is_err() {
        Command::new("meson")
            .arg("setup")
            .arg(&format!("-Dbuildtype={buildtype}"))
            .arg(target)
            .spawn()?
            .wait()?;
    }

    Command::new("ninja").args(["-C", target]).spawn()?.wait()?;

    if fs::metadata("compile_commands.json").is_ok() {
        fs::remove_file("compile_commands.json")?;
    }

    unix::fs::symlink(
        path![target / "compile_commands.json"],
        "compile_commands.json",
    )?;

    Ok(())
}

fn test() -> Result<()> {
    if fs::metadata(DEBUG_DIR).is_err() {
        build(Profile::Debug)?;
    }

    Command::new("meson")
        .arg("test")
        .args(["-C", DEBUG_DIR])
        .spawn()?
        .wait()?;

    Ok(())
}

fn clean() -> Result<()> {
    fs::remove_dir_all(TARGET_DIR)?;
    if fs::metadata("compile_commands.json").is_ok() {
        fs::remove_file("compile_commands.json")?;
    }
    Ok(())
}
