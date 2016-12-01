//! The export command allows the user to export
//! artifacts in multiple static text formats

use std::ascii::AsciiExt;

use super::types::*;
use super::super::core::{ArtifactData};

#[cfg(feature = "web")]
use super::super::api;

pub enum ExportType {
    Html,
}

pub struct Config {
    path: PathBuf,
    ty: ExportType,
}

/// Get the server subcommand for the cmdline
/// Partof: 
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("export")
        .about("export artfiacts")
        .settings(&[AS::DeriveDisplayOrder, COLOR])
        .arg(Arg::with_name("path")
                 .help("file or directory to put the files")
                 .use_delimiter(false))
        .arg(Arg::with_name("type")
                 .help("the type of export. Currently only html is supported.")
                 .use_delimiter(false))
}


/// pull out the command settings
pub fn get_cmd(matches: &ArgMatches) -> Result<Config, String> {
    let path = match matches.value_of("path") {
        Some(p) => PathBuf::from(p),
        None => return Err("no path selected".to_string())
    };
    let ty = match matches.value_of("type") {
        Some(t) => {
            let t = t.to_ascii_lowercase();
            match t.as_ref() {
                "html" => ExportType::Html,
                _ => return Err(format!("invalid type selected: {}", t)),
            }
        }
        None => return Err("no export type selected".to_string())
    };
    Ok(Config {
        path: path,
        ty: ty,
    })
}

#[cfg(feature = "web")]
pub fn run_cmd(artifacts: &Artifacts, config: Config) -> Result<(), String> {
    let data: Vec<ArtifactData> = artifacts
        .iter().map(|(name, model)| model.to_data(name)).collect();
    api::unpack_app(&config.path, "", Some(&data));
    Ok(())
}

#[cfg(not(feature = "web"))]
pub fn run_cmd(_: &Artifacts, _: Config) -> Result<(), String> {
    Err("rst was not compiled with the web option".to_string())
}
