use super::table_struct::UserAppPrivilege;
use crate::db::Db;
use crate::tables::user_app_privileges::table_struct::{
    USER_APP_PRIVILEGES_KEYS, USER_APP_PRIVILEGES_TABLE_NAME,
};
use sqlx::query;
use sqlx::Transaction;

impl Db {
    pub async fn add_new_privilege_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        privilege: &UserAppPrivilege,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {USER_APP_PRIVILEGES_TABLE_NAME} ({USER_APP_PRIVILEGES_KEYS}) VALUES ($1, $2, $3, $4)"
        );

        let query_result = query(&query_body)
            .bind(&privilege.user_id)
            .bind(&privilege.app_id)
            .bind(&privilege.privilege_level)
            .bind(&privilege.creation_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn add_new_privilege(&self, privilege: &UserAppPrivilege) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {USER_APP_PRIVILEGES_TABLE_NAME} ({USER_APP_PRIVILEGES_KEYS}) VALUES ($1, $2, $3, $4)"
        );

        let query_result = query(&query_body)
            .bind(&privilege.user_id)
            .bind(&privilege.app_id)
            .bind(&privilege.privilege_level)
            .bind(&privilege.creation_timestamp)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        structs::privilege_level::PrivilegeLevel,
        tables::{
            grafana_users::table_struct::GrafanaUser,
            user_app_privileges::table_struct::UserAppPrivilege, utils::to_microsecond_precision,
        },
    };
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_add_new_privilege() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let user = GrafanaUser {
            email: "test_user_email".to_string(),
            password_hash: "test_password_hash".to_string(),
            user_id: "test_user_id".to_string(),
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_user(&user).await.unwrap();

        let privilege = UserAppPrivilege {
            user_id: user.user_id.clone(),
            app_id: app_id.clone(),
            privilege_level: PrivilegeLevel::Edit,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();

        let get_by_user_id_and_app_id = db
            .get_privilege_by_user_id_and_app_id(&user.user_id, &app_id)
            .await
            .unwrap();
        assert_eq!(privilege, get_by_user_id_and_app_id);

        let get_by_user_id = db.get_privileges_by_user_id(&user.user_id).await.unwrap();
        assert_eq!(vec![privilege.clone()], get_by_user_id);

        let get_by_app_id = db.get_privileges_by_app_id(&app_id).await.unwrap();
        assert!(get_by_app_id.len() == 2);
        assert!(get_by_app_id.contains(&privilege));
    }
}
