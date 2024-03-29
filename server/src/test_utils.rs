#[cfg(test)]
pub mod test_utils {
    use crate::{
        auth::AuthToken,
        env::{JWT_PUBLIC_KEY, JWT_SECRET},
        http::cloud::{
            accept_team_invite::{HttpAcceptTeamInviteRequest, HttpAcceptTeamInviteResponse},
            get_team_user_invites::HttpGetTeamUserInvitesResponse,
            get_user_joined_teams::HttpGetUserJoinedTeamsResponse,
            get_user_team_invites::HttpGetUserTeamInvitesResponse,
            invite_user_to_team::{HttpInviteUserToTeamRequest, HttpInviteUserToTeamResponse},
            login::login_with_password::{HttpLoginRequest, HttpLoginResponse},
            register_new_app::{HttpRegisterNewAppRequest, HttpRegisterNewAppResponse},
            register_new_team::{HttpRegisterNewTeamRequest, HttpRegisterNewTeamResponse},
            register_with_password::HttpRegisterWithPasswordRequest,
            remove_user_from_team::{
                HttpRemoveUserFromTeamRequest, HttpRemoveUserFromTeamResponse,
            },
        },
        routes::router::get_router,
        statics::NAME_REGEX,
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
    };
    use anyhow::bail;
    use axum::{
        body::{to_bytes, Body},
        extract::{ConnectInfo, Request},
        http::{Method, Response, StatusCode},
        Router,
    };
    use database::db::Db;
    use rand::{
        distributions::{Alphanumeric, Uniform},
        thread_rng, Rng,
    };
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

        let auth_token = AuthToken::decode(&response.auth_token, JWT_PUBLIC_KEY(), ip.0).unwrap();
        return (auth_token, email, password);
    }

    pub async fn add_test_team(
        team_name: &String,
        admin_token: &AuthToken,
        app: &Router,
        personal: bool,
    ) -> anyhow::Result<String> {
        // Register new team
        let request = HttpRegisterNewTeamRequest {
            team_name: team_name.clone(),
            personal: personal,
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

    pub async fn invite_user_to_test_team(
        team_id: &String,
        user_email: &String,
        admin_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<()> {
        // Invite user to test team
        let request = HttpInviteUserToTeamRequest {
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
                HttpCloudEndpoint::InviteUserToTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpInviteUserToTeamResponse>(response)
            .await
            .map(|_| Ok(()))?
    }

    pub async fn accept_invite_to_test_team(
        team_id: &String,
        user_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<()> {
        // Invite user to test team
        let request = HttpAcceptTeamInviteRequest {
            team_id: team_id.clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = user_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::AcceptTeamInvite.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpAcceptTeamInviteResponse>(response)
            .await
            .map(|_| Ok(()))?
    }

    pub async fn add_user_to_test_team(
        team_id: &String,
        user_email: &String,
        admin_token: &AuthToken,
        user_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<()> {
        // Add user to test team
        match invite_user_to_test_team(team_id, user_email, admin_token, app).await {
            Ok(_) => accept_invite_to_test_team(team_id, user_token, app).await,
            Err(e) => bail!("Failed to invite user to the team: {}", e),
        }
    }

    pub async fn get_test_team_user_invites(
        team_id: &String,
        user_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<HttpGetTeamUserInvitesResponse> {
        // Get team invites for users
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = user_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}?teamId={team_id}",
                HttpCloudEndpoint::GetTeamUserInvites.to_string()
            ))
            .extension(ip)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpGetTeamUserInvitesResponse>(response).await
    }

    pub async fn get_test_user_team_invites(
        user_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<HttpGetUserTeamInvitesResponse> {
        // Get use team invites

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = user_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::GetUserTeamInvites.to_string()
            ))
            .extension(ip)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpGetUserTeamInvitesResponse>(response).await
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

    pub async fn get_test_user_joined_teams(
        user_token: &AuthToken,
        app: &Router,
    ) -> anyhow::Result<HttpGetUserJoinedTeamsResponse> {
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = user_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::GetUserJoinedTeams.to_string()
            ))
            .extension(ip)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpGetUserJoinedTeamsResponse>(response).await
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
                let body = body_to_vec(response).await?;
                match serde_json::from_slice::<T>(&body) {
                    Ok(payload) => Ok(payload),
                    Err(e) => bail!("Error deserializing response: {}", e),
                }
            }
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::BAD_REQUEST => {
                let error_message = convert_response_into_error_string(response).await?;
                bail!("{}", error_message)
            }
            _ => {
                let status = response.status();
                bail!("{}", status)
            }
        }
    }

    pub fn generate_valid_name() -> String {
        let mut rng = rand::thread_rng();

        // Define ranges for alphanumeric characters and individual characters for underscore and slash.
        let char_ranges = ['a'..'z', 'A'..'Z', '0'..'9'];
        let single_chars = ['_', '-'];

        // Flatten the char_ranges into a single collection of characters and add individual characters.
        let mut chars: Vec<char> = char_ranges.into_iter().flat_map(|range| range).collect();
        chars.extend(single_chars.iter());

        // Ensure the distribution covers the range of available characters.
        let dist = Uniform::from(0..chars.len());

        // Define minimum and maximum length based on the regex pattern.
        let min_len = 3;
        let max_len = 30;
        let name_len = rng.gen_range(min_len..=max_len);

        // Generate a random string of valid characters within the defined length.
        let name: String = (0..name_len).map(|_| chars[rng.sample(&dist)]).collect();

        // Ensure the generated name matches the regex pattern.
        assert!(NAME_REGEX.is_match(&name));

        name
    }
}
