use super::table_struct::UserAppPrivilege;
use crate::db::Db;
use crate::tables::registered_app::table_struct::REGISTERED_APPS_TABLE_NAME;
use crate::tables::user_app_privileges::table_struct::USER_APP_PRIVILEGES_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_privilege_by_user_id_and_app_id(
        &self,
        user_id: &String,
        app_id: &String,
    ) -> Result<UserAppPrivilege, sqlx::Error> {
        let query = format!(
            "SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1 AND app_id = $2"
        );
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&user_id)
            .bind(&app_id)
            .fetch_one(&self.connection_pool)
            .await;
    }

    pub async fn get_privileges_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, sqlx::Error> {
        let query = format!("SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1");
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&user_id)
            .fetch_all(&self.connection_pool)
            .await;
    }

    pub async fn get_privileges_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, sqlx::Error> {
        let query = format!("SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await;
    }

    // Get all privileges for a team
    pub async fn get_privileges_by_team_id(
        &self,
        team_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, sqlx::Error> {
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
            .await;
    }

    pub async fn get_teams_and_apps_membership_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Vec<(String, String)>, sqlx::Error> {
        let query = format!(
            "SELECT ra.team_id, ra.app_id FROM {USER_APP_PRIVILEGES_TABLE_NAME} uap 
             JOIN {REGISTERED_APPS_TABLE_NAME} ra ON uap.app_id = ra.app_id 
             WHERE uap.user_id = $1"
        );
        let typed_query = sqlx::query_as::<_, (String, String)>(&query);

        return typed_query
            .bind(user_id)
            .fetch_all(&self.connection_pool)
            .await;
    }
}
