use std::path::Path;
use std::collections::HashMap;

use time;


// General
pub mod types;

// for loading
pub mod utils;
#[macro_use] pub mod load;  // macro use so the macro can be tested
pub mod vars;
pub mod link;
pub mod locs;

// serialization
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));
pub mod serialize;

#[cfg(test)]
mod tests;

// export for other modules to use
pub use core::utils::find_repo;
pub use core::types::{
    LoadResult, LoadError,
    Artifact, Artifacts,
    ArtType, Loc,
    ArtName, ArtNameRc, ArtNames,
    Settings, LoadFromStr,
    PARENT_PATH};
pub use core::load::load_toml;

#[cfg(test)]
use super::init_logger;
#[cfg(test)]
pub fn init_logger_test() {
    match init_logger(false, 3, false) {
        Ok(_) => {},
        Err(_) => {},
    }
}



/// Load all items from the toml file at path
pub fn load_path(path: &Path) -> LoadResult<(Artifacts, Settings, HashMap<ArtName, Loc>)>{
    let start = time::get_time();
    info!("loading path: {}", path.to_string_lossy().as_ref());
    let (mut artifacts, mut settings, loaded_vars, mut repo_map) = try!(load::load_raw(path));

    info!("resolving and filling variables");
    let mut variables = try!(vars::resolve_loaded_vars(loaded_vars, &mut repo_map));
    try!(vars::fill_text_fields(&mut artifacts, &mut variables, &mut repo_map));

    info!("finding and attaching locations");
    let locs = try!(locs::find_locs(&mut settings));
    let dne_locs = locs::attach_locs(&mut artifacts, locs);

    // do all links
    try!(link::do_links(&mut artifacts));
    let total = time::get_time() - start;
    info!("Done loading: {} artifacts loaded successfullly in {:.3} seconds",
          artifacts.len(), total.num_milliseconds() as f64 * 1e-3);
    Ok((artifacts, settings, dne_locs))
}

