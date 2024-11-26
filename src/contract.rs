use crate::{
    error::ContractError,
    msg::{DataResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{StoredData, STORED_DATA},
    utils::decode_hex,
};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use sha2::{Digest, Sha256};

// Version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SaveData(save_data_msg) => {
            let data = save_data_msg.data;
            let data_bytes = decode_hex(&data)?;
            let hash = Sha256::digest(data_bytes);
            let hash_string = hex::encode(hash);
            let hash_string_ref = hash_string.as_str();

            if STORED_DATA.has(deps.storage, hash_string_ref) {
                return Err(ContractError::DataAlreadyExists {});
            }

            let stored_data = StoredData {
                data: data.clone(),
            };

            STORED_DATA.save(deps.storage, hash_string_ref, &stored_data)?;

            Ok(Response::default()
                .add_attribute("data", data)
                .add_attribute("hash_string", hash_string))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryData(query_data_msg) => {
            let data = STORED_DATA.load(deps.storage, query_data_msg.data_hash.as_str())?;
            to_json_binary(&DataResponse {
                data,
            })
        }
    }
}

#[cfg(test)]
mod tests {}
