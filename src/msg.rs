use cosmwasm_std::{Api, StdResult, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    EvaporateTest { evaporate: Uint64 }
}

impl ExecuteMsg {
    pub fn execute_evaporate_gas(&self, api: &dyn Api) -> StdResult<()> {
        match self {
            ExecuteMsg::EvaporateTest { evaporate } => {
                api.gas_evaporate(evaporate.u64().try_into().unwrap_or(0))
            }
        }
    }
}
