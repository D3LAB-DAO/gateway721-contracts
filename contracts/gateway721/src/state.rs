use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CustomMsg};

use cw_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cw721_base::Cw721Contract;

use crate::msg::IncompleteProjectsResponse;

#[cw_serde]
pub struct Task {
    pub tid: String,
    pub input: String,
    pub output: Option<String>,
    // pub done: Option<bool>,
}

#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub destination: Option<Addr>,
    pub code: String,
    pub tasks: Option<Vec<Task>>,
}

pub type Extension = Option<Metadata>;

pub struct Gateway721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomMsg,
    E: CustomMsg,
{
    pub cw721: cw721_base::Cw721Contract<'a, T, C, E, Q>,

    pub incomplete_projects: Item<'a, IncompleteProjectsResponse>,
}

impl<T, C, E, Q> Default for Gateway721Contract<'static, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn default() -> Self {
        Self::new("incompleteprojects")
    }
}

impl<'a, T, C, E, Q> Gateway721Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn new(incomplete_projects_key: &'a str) -> Self {
        Self {
            incomplete_projects: Item::new(incomplete_projects_key),
            cw721: Cw721Contract::default(),
        }
    }
}
