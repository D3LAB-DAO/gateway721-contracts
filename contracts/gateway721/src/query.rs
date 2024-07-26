use cw721_base::state::TokenInfo;
use schemars::JsonSchema;

use cosmwasm_std::{to_binary, Binary, CustomMsg, Deps, Env, StdResult};

use crate::msg::{IncompleteProjectsResponse, QueryMsg, TaskIdsResponse};
use crate::state::{Extension, Gateway721Contract};
use crate::traits::Gateway721Query;
use cw721_base::QueryMsg as Cw721QueryMsg;

impl<'a, C, E, Q> Gateway721Contract<'a, Extension, C, E, Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            QueryMsg::Remains { token_id } => to_binary(&self.remains(deps, token_id)?),
            QueryMsg::IncompleteProjects {} => to_binary(&self.incomplete_projects(deps)?),
            _ => self.cw721.query(deps, env, msg.into()),
        }
    }
}

impl<'a, C, E, Q> Gateway721Query<Extension> for Gateway721Contract<'a, Extension, C, E, Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn remains(&self, deps: Deps, token_id: String) -> StdResult<TaskIdsResponse> {
        let token: TokenInfo<Extension> = self.cw721.tokens.load(deps.storage, &token_id)?;

        // Collect unresponded task IDs
        let task_ids = if let Some(metadata) = token.extension {
            if let Some(tasks) = metadata.tasks {
                tasks
                    .iter()
                    .filter(|task| task.output.is_none())
                    .map(|task| task.tid.clone())
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        Ok(TaskIdsResponse { tids: task_ids })
    }

    fn incomplete_projects(&self, deps: Deps) -> StdResult<IncompleteProjectsResponse> {
        self.incomplete_projects.load(deps.storage)
    }
}

impl<Q: JsonSchema> From<QueryMsg<Q>> for Cw721QueryMsg<Q> {
    fn from(item: QueryMsg<Q>) -> Self {
        match item {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
            QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            } => Cw721QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            },
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            QueryMsg::Extension { msg } => Cw721QueryMsg::Extension { msg },
            _ => panic!("Unsupported query message"), // This should not happen if handled correctly
        }
    }
}
