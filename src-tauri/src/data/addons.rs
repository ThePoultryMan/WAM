use confique::Config;
use serde::{Deserialize, Serialize};

use super::LocalSaveData;

#[derive(Config, Serialize)]
pub struct Addons {
    retail_list: Vec<AddonData>,
}

#[derive(Deserialize, Serialize)]
struct AddonData {
    name: String,
    version_id: String,
    version_name: Option<String>,
    provider: AddonProvider,
}

#[derive(Deserialize, Serialize)]
enum AddonProvider {
    CurseForge,
}

impl LocalSaveData for Addons {
    type Data = Addons;

    fn new() -> Self {
        Addons { retail_list: Vec::new() }
    }

    fn get_file_name() -> String {
        String::from("addons.toml")
    }
}
