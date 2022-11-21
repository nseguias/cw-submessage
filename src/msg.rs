use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    contract_id: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    RedirectFunds { address: String, amount: u128 },
    Withdraw {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetAdminResponse)]
    GetAdmin {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Addr,
}
