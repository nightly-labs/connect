#[cfg(test)]
pub mod test_utils {
    use crate::{
        auth::AuthToken,
        env::JWT_SECRET,
        http::cloud::{
            add_user_to_team::{HttpAddUserToTeamRequest, HttpAddUserToTeamResponse},
            login_with_password::{HttpLoginRequest, HttpLoginResponse},
            register_new_app::{HttpRegisterNewAppRequest, HttpRegisterNewAppResponse},
            register_new_team::{HttpRegisterNewTeamRequest, HttpRegisterNewTeamResponse},
            register_with_password::HttpRegisterWithPasswordRequest,
            remove_user_from_team::{
                HttpRemoveUserFromTeamRequest, HttpRemoveUserFromTeamResponse,
            },
        },
        routes::router::get_router,
        structs::cloud_http_endpoints::HttpCloudEndpoint,
    };
    use anyhow::bail;
    use axum::{
        body::{to_bytes, Body},
        extract::{ConnectInfo, Request},
        http::{Method, Response, StatusCode},
        Router,
    };
    use database::db::Db;
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use sqlx::Row;
    use std::net::SocketAddr;
    use tower::ServiceExt;

    pub async fn create_test_app(only_relay: bool) -> Router {
        let app = get_router(only_relay).await;

        let listener = tokio::net::TcpListener::bind(&"127.0.0.1:6969")
            .await
            .unwrap();
        let serve = axum::serve(
            listener,
            app.clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        );

        tokio::spawn(async move {
            serve.await.unwrap();
        });

        return app;
    }

    pub async fn truncate_all_tables(db: &mut Db) -> anyhow::Result<()> {
        let rows = sqlx::query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
        )
        .fetch_all(&db.connection_pool)
        .await?;

        if rows.is_empty() {
            println!("No tables to truncate");
            return Ok(());
        }

        // Join all names except _sqlx_migrations into a single string and run single truncate
        let tables_names = rows
            .iter()
            .map(|row| row.get::<String, &str>("table_name"))
            .filter(|table_name| !table_name.starts_with("_sqlx_migrations"))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!("TRUNCATE TABLE {tables_names} CASCADE");
        sqlx::query(&query).execute(&db.connection_pool).await?;

        // Reset all sequences
        let sequences = sqlx::query(
            "SELECT sequence_name FROM information_schema.sequences WHERE sequence_schema = 'public'",
        )
        .fetch_all(&db.connection_pool)
        .await?;

        for sequence in sequences {
            let seq_name = sequence.get::<String, &str>("sequence_name");
            let seq_reset_query = format!("ALTER SEQUENCE {} RESTART", seq_name);
            sqlx::query(&seq_reset_query)
                .execute(&db.connection_pool)
                .await?;
        }

        Ok(())
    }

    pub async fn register_and_login_random_user(app: &Router) -> (AuthToken, String, String) {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let email = format!("{rand_string}@gmail.com");
        let password = format!("Password123");

        // Register user
        let register_payload = HttpRegisterWithPasswordRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&register_payload).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::RegisterWithPassword.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // send request to app and get response
        let register_response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(register_response.status(), StatusCode::OK);

        // Login user
        let login_payload = HttpLoginRequest {
            email: email.to_string(),
            password: password.to_string(),
            enforce_ip: false,
        };

        let json = serde_json::to_string(&login_payload).unwrap();
        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::LoginWithPassword.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        let login_response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(login_response.status(), StatusCode::OK);

        let response = serde_json::from_slice::<HttpLoginResponse>(
            &body_to_vec(login_response).await.unwrap(),
        )
        .unwrap();

        let auth_token = AuthToken::decode(&response.auth_token, JWT_SECRET(), ip.0).unwrap();
        return (auth_token, email, password);
    }

    pub async fn add_test_team(
        team_name: &String,
        admin_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<String> {
        // Register new team
        let request = HttpRegisterNewTeamRequest {
            team_name: team_name.clone(),
            personal: false,
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = admin_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RegisterNewTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRegisterNewTeamResponse>(response)
            .await
            .map(|response| Ok(response.team_id))?
    }

    pub async fn add_test_app(
        request: &HttpRegisterNewAppRequest,
        admin_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<String> {
        // Register new app
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = admin_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RegisterNewApp.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRegisterNewAppResponse>(response)
            .await
            .map(|response| Ok(response.app_id))?
    }

    pub async fn add_user_to_test_team(
        team_id: &String,
        user_email: &String,
        admin_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<()> {
        // Add user to test team
        let request = HttpAddUserToTeamRequest {
            team_id: team_id.clone(),
            user_email: user_email.clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = admin_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::AddUserToTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpAddUserToTeamResponse>(response)
            .await
            .map(|_| Ok(()))?
    }

    pub async fn remove_user_from_test_team(
        team_id: &String,
        user_email: &String,
        admin_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<()> {
        // Add user to test team
        let request = HttpRemoveUserFromTeamRequest {
            team_id: team_id.clone(),
            user_email: user_email.clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = admin_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RemoveUserFromTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRemoveUserFromTeamResponse>(response)
            .await
            .map(|_| Ok(()))?
    }

    pub async fn body_to_vec(response: Response<Body>) -> anyhow::Result<Vec<u8>> {
        match to_bytes(response.into_body(), usize::MAX).await {
            Ok(body) => Ok(body.to_vec()),
            Err(e) => bail!("Error converting response into bytes: {}", e),
        }
    }

    pub async fn convert_response_into_error_string(
        response: Response<Body>,
    ) -> anyhow::Result<String> {
        match String::from_utf8(body_to_vec(response).await?) {
            Ok(message) => Ok(message),
            Err(e) => bail!("Error converting response into string: {}", e),
        }
    }

    pub async fn convert_response<T>(response: Response<Body>) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK => {
                let payload = serde_json::from_slice(&body_to_vec(response).await?)?;
                return Ok(payload);
            }
            _ => {
                let error_message = convert_response_into_error_string(response).await?;
                bail!(error_message);
            }
        }
    }
}
