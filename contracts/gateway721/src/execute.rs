use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::msg::{ExecuteMsg, IncompleteProjectsResponse, InstantiateMsg};
use crate::state::{Extension, Gateway721Contract, Task};
use crate::traits::Gateway721Execute;

use cosmwasm_std::{CustomMsg, DepsMut, Env, MessageInfo, Response, StdResult};
use cw721_base::state::TokenInfo;
use cw721_base::{
    ContractError, ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg,
};

impl<'a, T, C, E, Q> Gateway721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response<C>> {
        self.cw721.instantiate(
            DepsMut {
                storage: deps.storage,
                api: deps.api,
                querier: deps.querier,
            },
            _env.clone(),
            _info.clone(),
            Cw721InstantiateMsg {
                name: msg.name.clone(),
                symbol: msg.symbol.clone(),
                minter: _info.sender.to_string(),
            },
        )?;

        let incomplete_projects_data = IncompleteProjectsResponse { pids: Vec::new() };
        self.incomplete_projects
            .save(deps.storage, &incomplete_projects_data)?;

        Ok(Response::default())
    }
}

impl<'a, C, E, Q> Gateway721Contract<'a, Extension, C, E, Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, E>,
    ) -> Result<Response<C>, ContractError> {
        match msg {
            ExecuteMsg::Mint {
                token_id: _,
                owner,
                token_uri,
                extension,
            } => self.mint_anyone(deps, info, owner, token_uri, extension),
            ExecuteMsg::Request { token_id, input } => {
                self.request(deps, env, info, token_id, input)
            }
            ExecuteMsg::Response {
                token_id,
                task_id,
                output,
            } => self.response(deps, env, info, token_id, task_id, output),
            ExecuteMsg::Update {
                token_id,
                title,
                description,
            } => self.update(deps, env, info, token_id, title, description),
            _ => self.cw721.execute(deps, env, info, msg.into()),
        }
    }
}

impl<'a, C, E, Q> Gateway721Execute<Extension, C> for Gateway721Contract<'a, Extension, C, E, Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    type Err = ContractError;

    fn request(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        input: String,
    ) -> Result<Response<C>, Self::Err> {
        let mut token: TokenInfo<Extension> = self.cw721.tokens.load(deps.storage, &token_id)?;
        let new_tid;

        if let Some(ref mut metadata) = token.extension {
            // Initialize the tasks vector if it does not exist
            if metadata.tasks.is_none() {
                metadata.tasks = Some(Vec::new());
            }

            // Generate a new id for the task
            new_tid = (metadata.tasks.as_ref().unwrap().len()).to_string();

            // Push the new task to the tasks vector
            if let Some(ref mut tasks) = metadata.tasks {
                tasks.push(Task {
                    tid: new_tid.clone(),
                    input,
                    output: None,
                });
            } else {
                return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                    "tasks are not valid.",
                )));
            }
        } else {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "token extension is not valid.",
            )));
        }

        // Save the updated token back to storage
        self.cw721.tokens.save(deps.storage, &token_id, &token)?;

        Ok(Response::new()
            .add_attribute("action", "request")
            .add_attribute("requester", info.sender)
            .add_attribute("token_id", token_id)
            .add_attribute("task_id", new_tid))
    }

    fn response(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: Env,
        info: cosmwasm_std::MessageInfo,
        token_id: String,
        task_id: String,
        output: String,
    ) -> Result<Response<C>, Self::Err> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut token: TokenInfo<Extension> = self.cw721.tokens.load(deps.storage, &token_id)?;

        if let Some(ref mut metadata) = token.extension {
            if let Some(ref mut tasks) = metadata.tasks {
                // Find the task with the specified id and set the output
                if let Some(task) = tasks.iter_mut().find(|task| task.tid == task_id) {
                    task.output = Some(output);
                } else {
                    return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                        "task not found.",
                    )));
                }
            } else {
                return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                    "tasks are not valid.",
                )));
            }
        } else {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "token extension is not valid.",
            )));
        }

        // Save the updated token back to storage
        self.cw721.tokens.save(deps.storage, &token_id, &token)?;

        // TODO: destination

        // Create response
        Ok(Response::new()
            .add_attribute("action", "response")
            .add_attribute("token_id", token_id)
            .add_attribute("task_id", task_id.to_string()))
    }

    fn update(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: Env,
        info: cosmwasm_std::MessageInfo,
        token_id: String,
        title: String,
        description: String,
    ) -> Result<Response<C>, Self::Err> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut token: TokenInfo<Extension> = self.cw721.tokens.load(deps.storage, &token_id)?;
        if let Some(mut extension) = token.extension {
            if extension.title.is_some() && extension.description.is_some() {
                return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                    "both title and description fields are already filled.",
                )));
            }
            if extension.title.is_none() {
                extension.title = Some(title.clone());
            }
            if extension.description.is_none() {
                extension.description = Some(description.clone());
            }
            token.extension = Some(extension);
        } else {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "invalid fields.",
            )));
        }

        // project request queue update
        let incomplete_projects = self
            .incomplete_projects
            .load(deps.storage)
            .unwrap_or(IncompleteProjectsResponse { pids: Vec::new() });
        let new_ids: Vec<String> = incomplete_projects
            .pids
            .into_iter()
            .filter(|x| x != &token_id)
            .collect();
        self.incomplete_projects
            .save(deps.storage, &IncompleteProjectsResponse { pids: new_ids })?;

        // Save the updated token back to storage
        self.cw721.tokens.save(deps.storage, &token_id, &token)?;

        // Create response
        Ok(Response::new()
            .add_attribute("action", "response")
            .add_attribute("token_id", token_id))
    }
}

impl<'a, T, C, E, Q> Gateway721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn mint_anyone(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        // token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        // cw_ownable::assert_owner(deps.storage, &info.sender)?;

        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&owner)?,
            approvals: vec![],
            token_uri,
            extension,
        };
        let token_id = self.cw721.token_count(deps.storage)?.to_string(); // counter

        self.cw721
            .tokens
            .update(deps.storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.cw721.increment_tokens(deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("token_id", token_id))
    }
}

impl<T, E> From<ExecuteMsg<T, E>> for Cw721ExecuteMsg<T, E> {
    fn from(item: ExecuteMsg<T, E>) -> Self {
        match item {
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => Cw721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::Revoke { spender, token_id } => {
                Cw721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                Cw721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::RevokeAll { operator } => Cw721ExecuteMsg::RevokeAll { operator },
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Burn { token_id } => Cw721ExecuteMsg::Burn { token_id },
            ExecuteMsg::UpdateOwnership(action) => Cw721ExecuteMsg::UpdateOwnership(action),
            ExecuteMsg::Extension { msg } => Cw721ExecuteMsg::Extension { msg },
            _ => panic!("Unsupported execute message"), // This should not happen if handled correctly
        }
    }
}
