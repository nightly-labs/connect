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
            "WITH TeamJoinTimes AS (
                SELECT
                    uap.team_id,
                    MAX(uap.creation_timestamp) as user_joined_team_timestamp
                FROM
                    {USER_APP_PRIVILEGES_TABLE_NAME} uap
                GROUP BY
                    uap.team_id
            )
            SELECT
                t.team_id, t.team_name, t.personal, t.subscription, t.registration_timestamp, gu.email AS team_admin_email,
                ra.app_id, ra.app_name, ra.whitelisted_domains, ra.ack_public_keys, ra.registration_timestamp, uap.user_id,
                uap.privilege_level, uap.creation_timestamp, tjt.user_joined_team_timestamp
            FROM
                {TEAM_TABLE_NAME} t
            JOIN
                {REGISTERED_APPS_TABLE_NAME} ra ON t.team_id = ra.team_id
            JOIN
                {USER_APP_PRIVILEGES_TABLE_NAME} uap ON ra.app_id = uap.app_id
            JOIN
                {GRAFANA_USERS_TABLE_NAME} gu ON t.team_admin_id = gu.user_id
            LEFT JOIN
                TeamJoinTimes tjt ON t.team_id = tjt.team_id
            WHERE
                uap.user_id = $1
            ORDER BY
                t.team_id, ra.app_id"
        );
        let rows = sqlx::query(&query)
            .bind(user_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());

        if rows.is_err() {
            return Err(rows.err().unwrap());
        }

        let mut team_app_map: HashMap<
            String,
            (
                Team,
                String,
                DateTime<Utc>,
                Vec<(DbRegisteredApp, UserAppPrivilege)>,
            ),
        > = HashMap::new();

        // Safe unwrap
        for row in rows.unwrap() {
            let team = Team {
                team_id: row.get("team_id"),
                personal: row.get("personal"),
                team_name: row.get("team_name"),
                subscription: row.get("subscription"),
                registration_timestamp: row.get("registration_timestamp"),
                team_admin_id: row.get("team_admin_id"),
            };

            let admin_email = row.get("team_admin_email");

            let app = DbRegisteredApp {
                team_id: row.get("team_id"),
                app_id: row.get("app_id"),
                app_name: row.get("app_name"),
                whitelisted_domains: row.get("whitelisted_domains"),
                ack_public_keys: row.get("ack_public_keys"),
                registration_timestamp: row.get("registration_timestamp"),
            };

            let privilege = UserAppPrivilege {
                user_id: row.get("user_id"),
                app_id: row.get("app_id"),
                privilege_level: row.get("privilege_level"),
                creation_timestamp: row.get("creation_timestamp"),
            };

            let user_joined_team_timestamp: DateTime<Utc> = row.get("user_joined_team_timestamp");

            team_app_map
                .entry(team.team_id.clone())
                .or_insert_with(|| (team, admin_email, user_joined_team_timestamp, Vec::new()))
                .3
                .push((app, privilege));
        }

        Ok(team_app_map.into_values().collect())
    }
}
