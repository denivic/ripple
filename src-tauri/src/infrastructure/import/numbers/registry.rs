//! `IWAObjectType` numbers for the message types this reader navigates.
//! Apple ships no public registry (it's extracted from the app at
//! runtime); these were verified empirically against real `.numbers`
//! files' decompressed IWA streams (see the Phase 5 commit message) and
//! cross-checked against `TST.CellType` in the public iWork'13 proto
//! dump. Object identities the reader relies on; everything else it
//! walks past without needing a name for it.

// SHEET_ARCHIVE / TABLE_INFO_ARCHIVE: verified, but unused by the current
// navigation path — it scans for TABLE_MODEL_ARCHIVE directly rather than
// walking Sheet -> TableInfoArchive -> TableModelArchive by reference,
// which is more robust to iWork-version differences in that reference
// shape. Kept for a future sheet-name/ordering feature (SheetArchive.name
// may differ from TableModelArchive.table_name).
#[allow(dead_code)]
pub const SHEET_ARCHIVE: u32 = 2;
#[allow(dead_code)]
pub const TABLE_INFO_ARCHIVE: u32 = 6000;
pub const TABLE_MODEL_ARCHIVE: u32 = 6001;
pub const TILE: u32 = 6002;
pub const TABLE_DATA_LIST: u32 = 6005;
