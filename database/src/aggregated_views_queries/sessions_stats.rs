#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        db::Db,
        structs::{
            client_data::ClientData, geo_location::Geolocation, privilege_level::PrivilegeLevel,
            session_type::SessionType,
        },
        tables::{
            registered_app::table_struct::DbRegisteredApp,
            sessions::table_struct::{DbNcSession, SESSIONS_TABLE_NAME},
            team::table_struct::Team,
            user_app_privileges::table_struct::UserAppPrivilege,
            utils::{get_current_datetime, to_microsecond_precision},
        },
    };
    use futures::future::join_all;
    use rand::{thread_rng, Rng};
    use sqlx::query;
    use sqlx::{types::chrono::Utc, Transaction};
    use std::{sync::Arc, time::Duration};
    use tokio::sync::Semaphore;

    #[derive(Debug, Clone)]
    struct BoundingBox {
        min_lat: f64,
        max_lat: f64,
        min_lon: f64,
        max_lon: f64,
    }

    #[tokio::test]
    async fn generate_session_stats_2() {
        let db = Db::connect_to_the_pool().await;

        // let team_id = "test_team_id".to_string();

        let now = get_current_datetime();
        let start_date = now - Duration::from_secs(60 * 60 * 24 * 45); // Start of second period, 45 days ago
        let end_date = now;
        let num_days = (end_date - start_date).num_days();

        let db_arc = Arc::new(db);

        // // Add second app
        let app_id_2 = "018fde27-ebf1-7397-a81f-640f6b9bcead".to_string();
        // let registered_app = DbRegisteredApp {
        //     team_id: team_id.clone(),
        //     app_id: app_id_2.clone(),
        //     app_name: format!("{app_id_2}_APP_NAME").to_string(),
        //     whitelisted_domains: vec!["localhost".to_string()],
        //     ack_public_keys: vec!["key".to_string()],
        //     registration_timestamp: get_current_datetime(),
        // };

        // let mut tx = db_arc.connection_pool.begin().await.unwrap();
        // db_arc
        //     .register_new_app_within_tx(&mut tx, &registered_app)
        //     .await
        //     .unwrap();
        // tx.commit().await.unwrap();

        let land_boxes = Arc::new(vec![
            BoundingBox {
                min_lat: 49.0,
                max_lat: 70.0,
                min_lon: -126.0,
                max_lon: -60.0,
            }, // Canada
            BoundingBox {
                min_lat: 24.0,
                max_lat: 50.0,
                min_lon: -125.0,
                max_lon: -66.0,
            }, // USA
            BoundingBox {
                min_lat: -55.0,
                max_lat: -21.0,
                min_lon: -74.0,
                max_lon: -34.0,
            }, // South America
            BoundingBox {
                min_lat: 34.0,
                max_lat: 60.0,
                min_lon: -10.0,
                max_lon: 35.0,
            }, // Europe
            BoundingBox {
                min_lat: -35.0,
                max_lat: 10.0,
                min_lon: 112.0,
                max_lon: 154.0,
            }, // Australia
            BoundingBox {
                min_lat: 8.0,
                max_lat: 37.0,
                min_lon: 68.0,
                max_lon: 97.0,
            }, // India
            BoundingBox {
                min_lat: 30.0,
                max_lat: 45.0,
                min_lon: 120.0,
                max_lon: 150.0,
            }, // Japan
        ]);

        let points_per_box = 10;
        let mut land_points = Vec::new();
        for box_ in land_boxes.iter() {
            let mut box_points = Vec::with_capacity(points_per_box);
            for _ in 0..points_per_box {
                let lat = thread_rng().gen_range(box_.min_lat..box_.max_lat);
                let lon = thread_rng().gen_range(box_.min_lon..box_.max_lon);
                box_points.push((lat, lon));
            }
            land_points.push(box_points);
        }
        let land_points = Arc::new(land_points);

        let semaphore = Arc::new(Semaphore::new(16));
        let mut handles = vec![];

        for day_offset in 0..num_days {
            let app_id_clone = app_id_2.clone();
            let db_arc_clone = db_arc.clone();
            let sem_clone = semaphore.clone();
            let land_points_clone = land_points.clone();

            let handle = tokio::task::spawn(async move {
                let _permit = sem_clone
                    .acquire()
                    .await
                    .expect("Failed to acquire semaphore permit");

                let day_start = start_date + Duration::from_secs(60 * 60 * 24 * day_offset as u64);

                let is_seventh_day = day_offset % 7 == 6; // day_offset is 0-based; 6 represents the 7th day

                // Loop through each hour of the day
                for hour in 0..24 {
                    let hour_start = day_start + Duration::from_secs(60 * 60 * hour as u64);
                    let session_count = if is_seventh_day {
                        // For the 7th day, limit sessions to 3-6 per hour
                        rand::thread_rng().gen_range(3..=6)
                    } else if hour >= 6 && hour <= 19 {
                        // Regular daytime hours
                        thread_rng().gen_range(20..=40)
                    } else {
                        // Regular nighttime hours
                        rand::thread_rng().gen_range(10..=20)
                    };

                    // Generate sessions for this hour
                    for i in 0..session_count {
                        let session_start =
                            hour_start + Duration::from_secs(rand::thread_rng().gen_range(0..3600));
                        let session_end = session_start + Duration::from_secs(600); // 10 minutes

                        let session = DbNcSession {
                            session_id: uuid7::uuid7().to_string(),
                            app_id: app_id_clone.clone(),
                            app_metadata: "test_metadata".to_string(),
                            persistent: false,
                            network: "Solana".to_string(),
                            client_data: None,
                            session_open_timestamp: session_start.clone(),
                            session_close_timestamp: Some(session_end),
                        };

                        let ip_address = "127.0.0.1".to_string();

                        db_arc_clone
                            .handle_new_session(&session, None, &ip_address, &session_start)
                            .await
                            .unwrap();

                        if i % 3 != 0 {
                            let data = ClientData {
                                client_id: "test_client_id".to_string(),
                                wallet_name: "test_wallet_name".to_string(),
                                wallet_type: "test_wallet_type".to_string(),
                                client_profile_id: session_start.timestamp(),
                                connected_at: session_start,
                            };

                            // Update the session with the client data
                            let query_body = format!(
                                "UPDATE {SESSIONS_TABLE_NAME} SET client_data = $1 WHERE session_id = $2"
                            );

                            query(&query_body)
                                .bind(&data)
                                .bind(&session.session_id)
                                .execute(&db_arc_clone.connection_pool)
                                .await
                                .unwrap();
                        }

                        let amount = thread_rng().gen_range(0..=10);
                        // Generate random amount of additional connections from user
                        for _ in 0..amount {
                            let geolocation = match rand::thread_rng().gen_range(0..=10) > 3 {
                                true => None,
                                false => {
                                    // Randomly select a bounding box
                                    let box_index =
                                        thread_rng().gen_range(0..land_points_clone.len());
                                    let point_index = thread_rng()
                                        .gen_range(0..land_points_clone[box_index].len());
                                    let (lat, lon) = land_points_clone[box_index][point_index];

                                    Some(Geolocation {
                                        country: None,
                                        city: None,
                                        lat: Some(lat),
                                        lon: Some(lon),
                                    })
                                }
                            };

                            let mut tx = db_arc_clone.connection_pool.begin().await.unwrap();
                            db_arc_clone
                                .create_new_connection_event_by_client(
                                    &mut tx,
                                    &session.app_id,
                                    &"Solana".to_string(),
                                    &session.session_id,
                                    &SessionType::Relay,
                                    &ip_address,
                                    geolocation,
                                    &(session_start + Duration::from_secs(1)),
                                )
                                .await
                                .unwrap();
                            tx.commit().await.unwrap();
                        }

                        db_arc_clone
                            .close_session(&session.session_id, session_end)
                            .await
                            .unwrap();
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        join_all(handles).await;

        // Manually refresh the continuous aggregates
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_sessions_stats_per_app".to_string(),
                "hourly_sessions_stats_per_app".to_string(),
                "daily_sessions_stats_per_app".to_string(),
                // "monthly_sessions_stats_per_app".to_string(),
            ])
            .await
            .unwrap();

        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_connection_stats_per_app".to_string(),
                "hourly_connection_stats_per_app".to_string(),
                "daily_connection_stats_per_app".to_string(),
                // "monthly_sessions_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn tesadasdasdasd() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let privilege = UserAppPrivilege {
            user_id: "test_admin".to_string().clone(),
            app_id: "test_app_id_2".to_string().clone(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();
    }

    #[tokio::test]
    async fn user_privs() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let user_id = "test@gmail.com".to_string();
        let email = "test@gmail.com".to_string();

        db.add_new_user(&user_id, &email, None, None).await.unwrap();

        let privilege = UserAppPrivilege {
            user_id,
            app_id: "test_app_id_2".to_string().clone(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();
    }

    #[tokio::test]
    async fn new_team() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let team_id = "test_team_id_2".to_string();
        let app_id = "test_app_id_3".to_string();

        let user_id = "test_admin_2".to_string();
        let email = "admin_2".to_string();
        let password_hash = "pass_hash".to_string();

        let registration_timestamp = get_current_datetime();

        db.add_new_user(&user_id, &email, Some(&password_hash), None)
            .await
            .unwrap();

        let team = Team {
            team_id: team_id.clone(),
            team_name: "test_team_name".to_string(),
            personal: false,
            subscription: None,
            team_admin_id: user_id.clone(),
            registration_timestamp: registration_timestamp,
        };

        let registered_app = DbRegisteredApp {
            team_id: team_id.clone(),
            app_id: app_id.clone(),
            app_name: format!("{app_id}_APP_NAME").to_string(),
            whitelisted_domains: vec!["localhost".to_string()],
            ack_public_keys: vec!["key".to_string()],
            registration_timestamp: registration_timestamp,
        };

        let admin_privilege = UserAppPrivilege {
            app_id: app_id.clone(),
            creation_timestamp: registration_timestamp,
            privilege_level: PrivilegeLevel::Admin,
            user_id: user_id.clone(),
        };

        // Start a transaction
        let mut tx: Transaction<'_, sqlx::Postgres> = db.connection_pool.begin().await.unwrap();

        // Attempt to create the new team within the transaction
        let _create_team_result = db.create_new_team_within_tx(&mut tx, &team).await.unwrap();

        // Attempt to register the new app within the same transaction
        let _register_app_result = db
            .register_new_app_within_tx(&mut tx, &registered_app)
            .await
            .unwrap();

        // Attempt to add team admin within the same transaction
        let _add_admin_result = db
            .add_new_privilege_within_tx(&mut tx, &admin_privilege)
            .await
            .unwrap();

        // If both actions succeeded, commit the transaction
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn generate_session_stats_3() {
        let db = Db::connect_to_the_pool().await;

        let app_id_3 = "test_app_id_3".to_string();

        let now = get_current_datetime();
        let start_date = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of second period, 60 days ago
        let end_date = now;
        let num_days = (end_date - start_date).num_days();

        let db_arc = Arc::new(db);

        let semaphore = Arc::new(Semaphore::new(8));
        let mut handles = vec![];

        for day_offset in 0..num_days {
            let app_id_clone = app_id_3.clone();
            let db_arc_clone = db_arc.clone();
            let sem_clone = semaphore.clone();

            let handle = tokio::task::spawn(async move {
                let _permit = sem_clone
                    .acquire()
                    .await
                    .expect("Failed to acquire semaphore permit");

                let day_start = start_date + Duration::from_secs(60 * 60 * 24 * day_offset as u64);

                let is_seventh_day = day_offset % 7 == 6; // day_offset is 0-based; 6 represents the 7th day

                // Loop through each hour of the day
                for hour in 0..24 {
                    let hour_start = day_start + Duration::from_secs(60 * 60 * hour as u64);
                    let session_count = if is_seventh_day {
                        // For the 7th day, limit sessions to 3-6 per hour
                        rand::thread_rng().gen_range(1..=5)
                    } else if hour >= 6 && hour <= 19 {
                        // Regular daytime hours
                        thread_rng().gen_range(40..=90)
                    } else {
                        // Regular nighttime hours
                        rand::thread_rng().gen_range(15..=45)
                    };

                    // Generate sessions for this hour
                    for i in 0..session_count {
                        let session_start =
                            hour_start + Duration::from_secs(rand::thread_rng().gen_range(0..3600));
                        let session_end = session_start + Duration::from_secs(600); // 10 minutes

                        let client_data = if i % 2 == 0 {
                            Some(ClientData {
                                client_id: "test_client_id".to_string(),
                                wallet_name: "test_wallet_name".to_string(),
                                wallet_type: "test_wallet_type".to_string(),
                                client_profile_id: 1,
                                connected_at: session_start,
                            })
                        } else {
                            None
                        };

                        let session = DbNcSession {
                            session_id: uuid7::uuid7().to_string(),
                            app_id: app_id_clone.clone(),
                            app_metadata: "test_metadata".to_string(),
                            persistent: false,
                            network: "Solana".to_string(),
                            client_data,
                            session_open_timestamp: session_start.clone(),
                            session_close_timestamp: Some(session_end),
                        };

                        db_arc_clone
                            .handle_new_session(
                                &session,
                                None,
                                &"127.0.0.1".to_string(),
                                &session_start,
                            )
                            .await
                            .unwrap();
                        db_arc_clone
                            .close_session(&session.session_id, session_end)
                            .await
                            .unwrap();
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        join_all(handles).await;

        // Manually refresh the continuous aggregates
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_sessions_stats_per_app".to_string(),
                "hourly_sessions_stats_per_app".to_string(),
                "daily_sessions_stats_per_app".to_string(),
                // "monthly_sessions_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn admin_privs_team_2() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let privilege = UserAppPrivilege {
            user_id: "test_admin".to_string().clone(),
            app_id: "test_app_id_3".to_string().clone(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();
    }
}
