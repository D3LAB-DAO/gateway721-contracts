mod execute;
pub mod msg;
mod query;
mod state;
mod traits;

use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use state::Extension;
use state::Gateway721Contract;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw721_base::ContractError;

// Version info for migration
const CONTRACT_NAME: &str = "Gateway721";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "library"))]
pub mod entry {
    use msg::IncompleteProjectsResponse;

    use super::*;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract = Gateway721Contract::<Extension, Empty, Empty, Empty>::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, ContractError> {
        let contract = Gateway721Contract::<Extension, Empty, Empty, Empty>::default();

        // project request queue
        let extension = if let ExecuteMsg::Mint { extension, .. } = &msg {
            extension.clone()
        } else {
            None
        };
        if let Some(ext) = &extension {
            if ext.title.is_none() || ext.description.is_none() {
                let token_id = contract.cw721.token_count(deps.storage)?.to_string(); // counter

                let mut incomplete_projects = contract
                    .incomplete_projects
                    .load(deps.storage)
                    .unwrap_or(IncompleteProjectsResponse { pids: Vec::new() });
                incomplete_projects.pids.push(token_id.clone());
                contract
                    .incomplete_projects
                    .save(deps.storage, &incomplete_projects)?;
            }
        }

        contract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let contract = Gateway721Contract::<Extension, Empty, Empty, Empty>::default();
        contract.query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };
    use cw721::NftInfoResponse;
    use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use state::Metadata;

    const CREATOR: &str = "creator";

    /// Make sure cw2 version info is properly initialized during instantiation,
    /// and NOT overwritten by the base contract.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        entry::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("larry", &[]),
            InstantiateMsg {
                name: "".into(),
                symbol: "".into(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
        assert_ne!(version.contract, cw721_base::CONTRACT_NAME);
    }

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Gateway721Contract::<Extension, Empty, Empty, Empty>::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let extension = Some(Metadata {
            code: "MEOW".into(),
            ..Metadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: "Not used".to_string(),
            owner: "john".to_string(),
            token_uri: Some("Not used".into()),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let query_msg: QueryMsg<Empty> = QueryMsg::NftInfo {
            token_id: "0".to_string(),
        };
        let res = contract.query(deps.as_ref(), mock_env(), query_msg);

        match res {
            Ok(binary_res) => {
                let res: NftInfoResponse<Metadata> =
                    from_binary(&binary_res).expect("Failed to parse binary response");
                // println!("{}", res.extension.code);
                assert_eq!(
                    res.extension,
                    extension.expect("Extension is None"),
                    "Extension does not match"
                );
            }
            Err(err) => panic!("Query failed: {:?}", err),
        };
    }
}
