use super::table_struct::UserAppPrivilege;
use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::{
        grafana_users::table_struct::GRAFANA_USERS_TABLE_NAME,
        registered_app::table_struct::{DbRegisteredApp, REGISTERED_APPS_TABLE_NAME},
        team::table_struct::{Team, TEAM_TABLE_NAME},
        user_app_privileges::table_struct::USER_APP_PRIVILEGES_TABLE_NAME,
    },
};
use sqlx::{query_as, types::chrono::DateTime};
use sqlx::{types::chrono::Utc, Row};
use std::collections::HashMap;

impl Db {
    pub async fn get_privilege_by_user_id_and_app_id(
        &self,
        user_id: &String,
        app_id: &String,
    ) -> Result<UserAppPrivilege, DbError> {
        let query = format!(
            "SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1 AND app_id = $2"
        );
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&user_id)
            .bind(&app_id)
            .fetch_one(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_privileges_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, DbError> {
        let query = format!("SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1");
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&user_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_privileges_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, DbError> {
        let query = format!("SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    // Get all privileges for a team
    pub async fn get_privileges_by_team_id(
        &self,
        team_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, DbError> {
        let query = format!(
            "SELECT uap.* FROM {USER_APP_PRIVILEGES_TABLE_NAME} uap 
             JOIN {REGISTERED_APPS_TABLE_NAME} ra ON uap.app_id = ra.app_id 
             WHERE ra.team_id = $1 
             GROUP BY uap.app_id, uap.user_id, uap.creation_timestamp, uap.privilege_level"
        );
        let typed_query = sqlx::query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(team_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_teams_and_apps_membership_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Vec<(String, String)>, DbError> {
        let query = format!(
            "SELECT ra.team_id, ra.app_id FROM {USER_APP_PRIVILEGES_TABLE_NAME} uap 
             JOIN {REGISTERED_APPS_TABLE_NAME} ra ON uap.app_id = ra.app_id 
             WHERE uap.user_id = $1"
        );
        let typed_query = sqlx::query_as::<_, (String, String)>(&query);

        return typed_query
            .bind(user_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_joined_teams_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<
        Vec<(
            Team,
            String,
            DateTime<Utc>,
            Vec<(DbRegisteredApp, UserAppPrivilege)>,
        )>,
        DbError,
    > {
        let query = format!(
            "WITH RelevantTeams AS (
                SELECT DISTINCT t.team_id, t.team_name, t.personal, t.subscription, 
                                t.registration_timestamp, gu.email AS team_admin_email,
                                gu.user_id AS team_admin_id,
                                CASE
                                    WHEN t.team_admin_id = $1 THEN t.registration_timestamp
                                    ELSE MAX(uap.creation_timestamp) OVER (PARTITION BY t.team_id)
                                END as user_joined_team_timestamp
                FROM {TEAM_TABLE_NAME} t
                LEFT JOIN {REGISTERED_APPS_TABLE_NAME} ra ON t.team_id = ra.team_id
                LEFT JOIN {USER_APP_PRIVILEGES_TABLE_NAME} uap ON ra.app_id = uap.app_id AND uap.user_id = $1
                JOIN {GRAFANA_USERS_TABLE_NAME} gu ON t.team_admin_id = gu.user_id
                WHERE t.team_admin_id = $1 OR uap.user_id = $1
            )
            SELECT rt.team_id, rt.team_name, rt.personal, rt.subscription, rt.registration_timestamp, 
                   rt.team_admin_email, rt.team_admin_id, ra.app_id, ra.app_name, ra.whitelisted_domains, 
                   ra.ack_public_keys, ra.registration_timestamp AS app_registration_timestamp, 
                   uap.user_id, uap.privilege_level, uap.creation_timestamp AS privilege_creation_timestamp,
                   rt.user_joined_team_timestamp
            FROM RelevantTeams rt
            LEFT JOIN {REGISTERED_APPS_TABLE_NAME} ra ON rt.team_id = ra.team_id
            LEFT JOIN {USER_APP_PRIVILEGES_TABLE_NAME} uap ON ra.app_id = uap.app_id AND uap.user_id = $1
            ORDER BY rt.team_id, ra.app_id"
        );

        let rows = sqlx::query(&query)
            .bind(user_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());

        if let Err(e) = rows {
            return Err(e);
        }

        let rows = rows.unwrap();
        let mut team_app_map: HashMap<
            String,
            (
                Team,
                String,
                DateTime<Utc>,
                Vec<(DbRegisteredApp, UserAppPrivilege)>,
            ),
        > = HashMap::new();

        for row in rows {
            let team = Team {
                team_id: row.get("team_id"),
                personal: row.get("personal"),
                team_name: row.get("team_name"),
                subscription: row.get("subscription"),
                registration_timestamp: row.get("registration_timestamp"),
                team_admin_id: row.get("team_admin_id"),
            };

            let admin_email = row.get("team_admin_email");
            let user_joined_team_timestamp: DateTime<Utc> = row.get("user_joined_team_timestamp");

            let team_id = team.team_id.clone();
            let team_entry = team_app_map
                .entry(team.team_id.clone())
                .or_insert_with(|| (team, admin_email, user_joined_team_timestamp, Vec::new()));

            if let Ok(app_id) = row.try_get("app_id") {
                if app_id != "" {
                    // Checking if app_id is present and not an empty string
                    let app = DbRegisteredApp {
                        team_id: team_id.clone(),
                        app_id,
                        app_name: row.get("app_name"),
                        whitelisted_domains: row.get("whitelisted_domains"),
                        ack_public_keys: row.get("ack_public_keys"),
                        registration_timestamp: row.get("app_registration_timestamp"),
                    };

                    let privilege = UserAppPrivilege {
                        user_id: row.get("user_id"),
                        app_id: app.app_id.clone(),
                        privilege_level: row.get("privilege_level"),
                        creation_timestamp: row.get("privilege_creation_timestamp"),
                    };

                    team_entry.3.push((app, privilege));
                }
            }
        }

        Ok(team_app_map.into_values().collect())
    }
}
