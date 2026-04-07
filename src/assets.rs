use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AssetManifest {
    pub face: Vec<String>,
    pub eyes: Vec<String>,
    pub mouth: Vec<String>,
    pub ears: Vec<String>,
}

impl AssetManifest {
    pub fn from_cat_dir(root: &Path) -> io::Result<Self> {
        Ok(Self {
            face: load_group(root, "face")?,
            eyes: load_group(root, "eyes")?,
            mouth: load_group(root, "mouth")?,
            ears: load_group(root, "ears")?,
        })
    }
}

fn load_group(root: &Path, group: &str) -> io::Result<Vec<String>> {
    let dir = root.join(group);
    let mut items = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let ext = path.extension()?.to_str()?;
            if ext.eq_ignore_ascii_case("png") {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    items.sort();
    Ok(items)
}
