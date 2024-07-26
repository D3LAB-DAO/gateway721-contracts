use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{IncompleteProjectsResponse, TaskIdsResponse};

pub trait Gateway721<T, C>: Gateway721Execute<T, C> + Gateway721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Gateway721Execute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;

    fn request(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        input: String,
    ) -> Result<Response<C>, Self::Err>;

    fn response(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        task_id: String,
        output: String,
    ) -> Result<Response<C>, Self::Err>;

    fn update(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        title: String,
        description: String,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Gateway721Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn remains(&self, deps: Deps, token_id: String) -> StdResult<TaskIdsResponse>;

    fn incomplete_projects(&self, deps: Deps) -> StdResult<IncompleteProjectsResponse>;
}
