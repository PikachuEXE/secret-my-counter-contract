use std::ops::Not;
use crate::msg::MigrateMsg;
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use crate::state::schema_migrations::SCHEMA_MIGRATION_VERSIONS;

pub mod versions;
mod migration_2024_12_05_001;
mod migration_2024_12_10_001;

pub fn perform_migration(
    deps: DepsMut,
    env: Env,
    msg: MigrateMsg,
) -> StdResult<Response> {
    match msg {
        MigrateMsg::Migrate {} => migrate_state(deps, env),
    }
}

struct MigrationEntry<'a> {
    version: &'a str,
    execute_fn: &'a dyn Fn(&mut DepsMut, Env) -> StdResult<Response>
}

pub fn all_migration_version() -> Vec<String> {
    all_migration_entries().iter().map(|m| m.version.to_string()).collect()
}

fn all_migration_entries<'a>() -> Vec<MigrationEntry<'a>> {
    Vec::from([
        MigrationEntry {
            version: versions::V2024_12_05_001,
            execute_fn: &migration_2024_12_05_001::execute,
        },
        MigrationEntry {
            version: versions::V2024_12_10_001,
            execute_fn: &migration_2024_12_10_001::execute,
        },
    ])
}

fn migrate_state(mut deps: DepsMut, env: Env) -> StdResult<Response> {
    let mut schema_migration_versions = SCHEMA_MIGRATION_VERSIONS.load(deps.storage).unwrap_or(vec![]);

    let last_version = schema_migration_versions.last().cloned().unwrap_or("none".to_string());
    let mut versions_run = vec![];
    let migration_entries = all_migration_entries();
    migration_entries.iter().for_each(|entry| {
        run_migrate_with_version(
            entry.version,
            &entry.execute_fn,
            &schema_migration_versions,
            &mut versions_run,
            &mut deps,
            env.clone(),
        ).unwrap();
    });
    // endregion migrations
    schema_migration_versions.extend(&mut versions_run.iter().cloned());
    SCHEMA_MIGRATION_VERSIONS.save(deps.storage, &schema_migration_versions)?;

    let schema_migration_versions_run_str = if versions_run.is_empty() {
        "none".to_string()
    } else {
        versions_run.join(", ")
    };
    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("last_version", last_version)
        .add_attribute("schema_migration_versions_run", schema_migration_versions_run_str)
        .add_attribute("status", "success"))
}

fn run_migrate_with_version(
    version: &str,
    func: &dyn Fn(&mut DepsMut, Env) -> StdResult<Response>,
    schema_migration_versions: &Vec<String>,
    versions_run: &mut Vec<String>,
    deps: &mut DepsMut,
    env: Env,
) -> StdResult<()> {
    if schema_migration_versions.iter().any(|x| x == version).not() {
        func(deps, env)?;
        versions_run.push(version.to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env};

    #[test]
    fn migrate_works_without_schema_migration_versions_saved() -> StdResult<()> {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);

        migration_2024_12_05_001::OLD_STATE.save(deps.as_mut().storage, &migration_2024_12_05_001::OldState {
            count: 3,
            count_increment_count: 2,
        })?;
        migration_2024_12_10_001::OLD_CONFIG.save(deps.as_mut().storage, &migration_2024_12_10_001::OldConfig {
            contract_manager: Addr::unchecked("contract_manager"),
        })?;

        // Ensure nothing saved
        assert!(SCHEMA_MIGRATION_VERSIONS.load(deps.as_mut().storage).is_err());

        let res = migrate_state(deps.as_mut(), mock_env())?;
        assert_eq!(
            res.attributes.iter().find(|a| a.key == "last_version").unwrap().value,
            "none",
        );
        assert_eq!(
            res.attributes.iter().find(|a| a.key == "schema_migration_versions_run").unwrap().value,
            all_migration_version().join(", "),
        );

        Ok(())
    }

    #[test]
    fn migrate_works_with_all_schema_migration_versions_saved() -> StdResult<()> {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);

        // Mark it as all migration run
        SCHEMA_MIGRATION_VERSIONS.save(
            deps.as_mut().storage,
            &crate::migrate::all_migration_version(),
        )?;

        let res = migrate_state(deps.as_mut(), mock_env())?;
        assert_eq!(
            res.attributes.iter().find(|a| a.key == "last_version").unwrap().value,
            crate::migrate::all_migration_version().last().cloned().unwrap_or("none".to_string()),
        );
        assert_eq!(
            res.attributes.iter().find(|a| a.key == "schema_migration_versions_run").unwrap().value,
            "none",
        );

        Ok(())
    }
}
