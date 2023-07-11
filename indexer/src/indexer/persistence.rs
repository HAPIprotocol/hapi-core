use {
    anyhow::Result,
    serde::{Deserialize, Serialize},
    std::{
        collections::VecDeque,
        fs,
        path::{Path, PathBuf},
    },
};

use super::IndexingCursor;

#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    pub(crate) cursor: IndexingCursor,
    pub(crate) tx_stack: VecDeque<String>,
}

impl PersistedState {
    pub fn from_file(path: &Path) -> Result<Self> {
        let state = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&state)?)
    }

    pub fn to_file(&self, path: &Path) -> Result<()> {
        // Create the parent directory if it doesn't exist
        if let Some(dir) = path.ancestors().nth(1) {
            if !PathBuf::from(dir).exists() {
                fs::create_dir_all(dir)?;
            }
        }
        let state = serde_json::to_string(self)?;
        fs::write(path, state)?;
        Ok(())
    }
}
