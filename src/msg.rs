use cosmwasm_std::{Api, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub enum InstantiateMsg {
    Nop {},
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    EvaporateTest { evaporate: u32 }
}

impl ExecuteMsg {
    pub fn execute_evaporate_gas(&self, api: &dyn Api) -> StdResult<()> {
        match self {
            ExecuteMsg::EvaporateTest { evaporate } => {
                return api.gas_evaporate(*evaporate);
            }
        }
    }
}
