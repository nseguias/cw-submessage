use cosmwasm_std::{DepsMut, Response};

use crate::{state::STATE, ContractError};

pub fn redirect_funds(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}
