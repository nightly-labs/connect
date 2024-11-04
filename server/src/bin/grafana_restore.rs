use core::panic;
use database::db::Db;
use openapi::apis::configuration::Configuration;
use server::env::{GF_SECURITY_ADMIN_PASSWORD, GF_SECURITY_ADMIN_USER, GRAFANA_BASE_PATH};
use server::http::cloud::grafana_utils::add_user_to_team::handle_grafana_add_user_to_team;
use server::http::cloud::grafana_utils::create_new_app::handle_grafana_create_new_app;
use server::http::cloud::grafana_utils::create_new_team::handle_grafana_create_new_team;
use server::utils::import_template_dashboards;
use std::collections::HashSet;
use std::sync::Arc;

// This script is used to restore the state of the Grafana instance
// Before running this script, clear the contents of the grafana-data folder
#[tokio::main]
async fn main() {
    let db = Db::connect_to_the_pool().await;
    let mut conf: Configuration = Configuration::new();
    conf.base_path = GRAFANA_BASE_PATH().to_string();
    conf.basic_auth = Some((
        GF_SECURITY_ADMIN_USER().to_string(),
        Some(GF_SECURITY_ADMIN_PASSWORD().to_string()),
    ));

    let grafana_client_conf = Arc::new(conf);
    // Setup template dashboards
    import_template_dashboards(&grafana_client_conf).await;

    if let Err(err) = db.clear_all_grafana_ids().await {
        panic!("Failed to clear grafana ids in database: {:?}", err);
    }

    let teams = match db.get_all_teams().await {
        Ok(teams) => teams,
        Err(e) => {
            panic!("Failed to get teams. Error: {:?}", e);
        }
    };
    for team in teams {
        // create teams
        let grafana_id = match handle_grafana_create_new_team(
            &grafana_client_conf,
            &team.team_admin_id,
            &team.team_name,
        )
        .await
        {
            Ok(id) => {
                if let Err(err) = db.update_grafana_id(&team.team_id, &id.to_string()).await {
                    panic!("Failed to update grafana id in database: {:?}", err);
                }
                id.to_string()
            }
            Err(err) => {
                panic!("Failed to create team in grafana: {:?}", err);
            }
        };

        let privileges = match db.get_privileges_by_team_id(&team.team_id).await {
            Ok(privileges) => privileges,
            Err(e) => {
                panic!("Failed to get privileges. Error: {:?}", e);
            }
        };
        let unique_user_ids: Vec<String> = privileges
            .into_iter()
            .map(|privilege| privilege.user_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let users_emails = match db.get_users_emails_by_ids(&unique_user_ids).await {
            Ok(emails) => emails,
            Err(e) => {
                panic!("Failed to get users emails. Error: {:?}", e);
            }
        };
        for (_, user_email) in users_emails {
            // add users to teams
            match handle_grafana_add_user_to_team(&grafana_client_conf, &grafana_id, &user_email)
                .await
            {
                Ok(id) => id,
                Err(err) => {
                    panic!("Failed to add user to team in grafana: {:?}", err);
                }
            };
        }
        let apps = match db.get_registered_apps_by_team_id(&team.team_id).await {
            Ok(apps) => apps,
            Err(e) => {
                panic!("Failed to get apps. Error: {:?}", e);
            }
        };
        for app in apps {
            let app_id = app.app_id.clone();
            let app_name = app.app_name.clone();
            // create apps
            match handle_grafana_create_new_app(
                &grafana_client_conf,
                &app_id,
                &app_name,
                &grafana_id,
            )
            .await
            {
                Ok(id) => id,
                Err(err) => {
                    panic!("Failed to create app in grafana: {:?}", err);
                }
            };
        }
    }
    println!("Got it! Exiting...");
}
