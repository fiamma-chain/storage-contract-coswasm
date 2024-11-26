use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

#[cw_serde]
pub struct StoredData {
    pub data: String,
}

pub const STORED_DATA: Map<&str, StoredData> = Map::new("stored_data");
