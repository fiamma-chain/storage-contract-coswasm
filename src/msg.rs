use crate::state::StoredData;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SaveData(SaveDataMsg),
}

#[cw_serde]
pub struct SaveDataMsg {
    pub data: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(DataResponse)]
    QueryData(QueryDataMsg),
}

#[cw_serde]
pub struct QueryDataMsg {
    // hex encoded hash of given data
    pub data_hash: String,
}

#[cw_serde]
pub struct DataResponse {
    pub data: StoredData,
}
