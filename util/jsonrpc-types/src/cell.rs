use crate::{Capacity, CellOutput, OutPoint, Script};
use ckb_types::{core::cell::CellStatus, H256};
use serde_derive::{Deserialize, Serialize};

// This is used as return value of get_cells_by_lock_hash RPC:
// it contains both OutPoint data used for referencing a cell, as well as
// cell's own data such as lock and capacity
#[derive(Serialize, Deserialize)]
pub struct CellOutputWithOutPoint {
    pub out_point: OutPoint,
    pub block_hash: H256,
    pub capacity: Capacity,
    pub lock: Script,
}

#[derive(Serialize, Deserialize)]
pub struct CellWithStatus {
    pub cell: Option<CellOutput>,
    pub status: String,
}

impl From<CellStatus> for CellWithStatus {
    fn from(status: CellStatus) -> Self {
        let (cell, status) = match status {
            CellStatus::Live(cell_meta) => (Some(cell_meta.cell_output), "live"),
            CellStatus::Dead => (None, "dead"),
            CellStatus::Unknown => (None, "unknown"),
        };
        Self {
            cell: cell.map(Into::into),
            status: status.to_string(),
        }
    }
}
