use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use super::archive::{parse_archives, ArchiveMessage};
use super::snappy_frames::decode_iwa_chunks;
use super::IwaError;

/// A `.numbers` file's every `Index/**/*.iwa`, decompressed and held for
/// the bundle's lifetime so [`Bundle::index`] can hand out borrowed
/// [`ArchiveMessage`]s without copying payload bytes.
pub struct Bundle {
    buffers: Vec<Vec<u8>>,
}

impl Bundle {
    /// Opens either the single-file zip form or an already-expanded
    /// package-directory form — Numbers produces both, depending on
    /// platform and how the file was extracted.
    pub fn open(path: &Path) -> Result<Self, IwaError> {
        if path.is_dir() {
            Self::open_directory(path)
        } else {
            Self::open_zip(path)
        }
    }

    fn open_zip(path: &Path) -> Result<Self, IwaError> {
        let file = std::fs::File::open(path)?;
        let mut zip = ZipArchive::new(file)?;
        let names: Vec<String> = (0..zip.len())
            .map(|i| zip.by_index(i).map(|f| f.name().to_string()))
            .collect::<Result<_, _>>()?;
        let mut buffers = Vec::new();
        for name in names {
            if !is_index_iwa(&name) {
                continue;
            }
            let mut entry = zip.by_name(&name)?;
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;
            buffers.push(decode_iwa_chunks(&bytes)?);
        }
        Ok(Self { buffers })
    }

    fn open_directory(path: &Path) -> Result<Self, IwaError> {
        let mut buffers = Vec::new();
        for entry in walk_iwa_files(&path.join("Index"))? {
            let bytes = std::fs::read(&entry)?;
            buffers.push(decode_iwa_chunks(&bytes)?);
        }
        Ok(Self { buffers })
    }

    /// Every archived object in the bundle, indexed by identifier. A
    /// `TSP.Reference.identifier` from any archive resolves through this
    /// map regardless of which physical `.iwa` file actually holds the
    /// referenced object — references routinely cross file boundaries
    /// (verified: `TableModelArchive` lives in `CalculationEngine.iwa`,
    /// its `Tile`s in `Index/Tables/Tile*.iwa`).
    pub fn index(&self) -> HashMap<u64, Vec<ArchiveMessage<'_>>> {
        let mut map: HashMap<u64, Vec<ArchiveMessage<'_>>> = HashMap::new();
        for buf in &self.buffers {
            if let Ok(archives) = parse_archives(buf) {
                for a in archives {
                    map.entry(a.identifier).or_default().push(a);
                }
            }
        }
        map
    }
}

fn is_index_iwa(name: &str) -> bool {
    name.starts_with("Index/") && name.ends_with(".iwa")
}

fn walk_iwa_files(dir: &Path) -> Result<Vec<PathBuf>, IwaError> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            out.extend(walk_iwa_files(&path)?);
        } else if path.extension().is_some_and(|e| e == "iwa") {
            out.push(path);
        }
    }
    Ok(out)
}
