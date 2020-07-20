use httpmock::Method::GET;
use httpmock::{mock, with_mock_server};

mod common;

use goose::prelude::*;

const INDEX_PATH: &str = "/";
const ERROR_PATH: &str = "/error";

const STATS_LOG_FILE: &str = "stats.log";
const DEBUG_LOG_FILE: &str = "debug.log";

pub async fn get_index(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get(INDEX_PATH).await?;
    Ok(())
}

pub async fn get_error(user: &GooseUser) -> GooseTaskResult {
    let mut goose = user.get(ERROR_PATH).await?;

    if let Ok(r) = goose.response {
        let headers = &r.headers().clone();
        if r.text().await.is_err() {
            return user.set_failure(
                "there was an error",
                &mut goose.request,
                Some(headers),
                None,
            );
        }
    }
    Ok(())
}

fn cleanup_files() {
    let _ = std::fs::remove_file(STATS_LOG_FILE);
    let _ = std::fs::remove_file(DEBUG_LOG_FILE);
}

#[test]
#[with_mock_server]
fn test_stat_logs_json() {
    cleanup_files();

    let mock_index = mock(GET, INDEX_PATH).return_status(200).create();

    let mut config = common::build_configuration();
    config.stats_log_file = STATS_LOG_FILE.to_string();
    config.no_stats = false;
    let _stats = crate::GooseAttack::initialize_with_config(config)
        .setup()
        .unwrap()
        .register_taskset(taskset!("LoadTest").register_task(task!(get_index)))
        .execute()
        .unwrap();

    let called_index = mock_index.times_called();

    // Confirm that we loaded the mock endpoints.
    assert_ne!(called_index, 0);

    // Confirm only the stats log file exists.
    let stats_log_exists = std::path::Path::new(STATS_LOG_FILE).exists();
    let debug_log_exists = std::path::Path::new(DEBUG_LOG_FILE).exists();
    assert_eq!(stats_log_exists, true);
    assert_eq!(debug_log_exists, false);
}

#[test]
#[with_mock_server]
fn test_stat_logs_csv() {
    cleanup_files();

    let mock_index = mock(GET, INDEX_PATH).return_status(200).create();

    let mut config = common::build_configuration();
    config.stats_log_file = STATS_LOG_FILE.to_string();
    config.stats_log_format = "csv".to_string();
    config.no_stats = false;
    let _stats = crate::GooseAttack::initialize_with_config(config)
        .setup()
        .unwrap()
        .register_taskset(taskset!("LoadTest").register_task(task!(get_index)))
        .execute()
        .unwrap();

    let called_index = mock_index.times_called();

    // Confirm that we loaded the mock endpoints.
    assert_ne!(called_index, 0);

    // Confirm only the stats log file exists.
    let stats_log_exists = std::path::Path::new(STATS_LOG_FILE).exists();
    let debug_log_exists = std::path::Path::new(DEBUG_LOG_FILE).exists();
    assert_eq!(stats_log_exists, true);
    assert_eq!(debug_log_exists, false);
}

#[test]
#[with_mock_server]
fn test_stat_logs_raw() {
    cleanup_files();

    let mock_index = mock(GET, INDEX_PATH).return_status(200).create();

    let mut config = common::build_configuration();
    config.stats_log_file = STATS_LOG_FILE.to_string();
    config.stats_log_format = "raw".to_string();
    config.no_stats = false;
    let _stats = crate::GooseAttack::initialize_with_config(config)
        .setup()
        .unwrap()
        .register_taskset(taskset!("LoadTest").register_task(task!(get_index)))
        .execute()
        .unwrap();

    let called_index = mock_index.times_called();

    // Confirm that we loaded the mock endpoints.
    assert_ne!(called_index, 0);

    // Confirm only the stats log file exists.
    let stats_log_exists = std::path::Path::new(STATS_LOG_FILE).exists();
    let debug_log_exists = std::path::Path::new(DEBUG_LOG_FILE).exists();
    assert_eq!(stats_log_exists, true);
    assert_eq!(debug_log_exists, false);
}

#[test]
#[with_mock_server]
fn test_debug_logs_raw() {
    cleanup_files();

    let mock_index = mock(GET, INDEX_PATH).return_status(200).create();
    let mock_error = mock(GET, ERROR_PATH).return_status(503).create();

    let mut config = common::build_configuration();
    config.debug_log_file = DEBUG_LOG_FILE.to_string();
    config.debug_log_format = "raw".to_string();
    let _stats = crate::GooseAttack::initialize_with_config(config)
        .setup()
        .unwrap()
        .register_taskset(
            taskset!("LoadTest")
                .register_task(task!(get_index))
                .register_task(task!(get_error)),
        )
        .execute()
        .unwrap();

    let called_index = mock_index.times_called();
    let called_error = mock_error.times_called();

    // Confirm that we loaded the mock endpoints.
    assert_ne!(called_index, 0);
    assert_ne!(called_error, 0);

    // Confirm only the debug log file exists.
    let stats_log_exists = std::path::Path::new(STATS_LOG_FILE).exists();
    let debug_log_exists = std::path::Path::new(DEBUG_LOG_FILE).exists();
    assert_eq!(stats_log_exists, false);
    assert_eq!(debug_log_exists, true);
}

#[test]
#[with_mock_server]
fn test_debug_logs_json() {
    cleanup_files();

    let mock_index = mock(GET, INDEX_PATH).return_status(200).create();
    let mock_error = mock(GET, ERROR_PATH).return_status(503).create();

    let mut config = common::build_configuration();
    config.debug_log_file = DEBUG_LOG_FILE.to_string();
    let _stats = crate::GooseAttack::initialize_with_config(config)
        .setup()
        .unwrap()
        .register_taskset(
            taskset!("LoadTest")
                .register_task(task!(get_index))
                .register_task(task!(get_error)),
        )
        .execute()
        .unwrap();

    let called_index = mock_index.times_called();
    let called_error = mock_error.times_called();

    // Confirm that we loaded the mock endpoints.
    assert_ne!(called_index, 0);
    assert_ne!(called_error, 0);

    // Confirm only the debug log file exists.
    let stats_log_exists = std::path::Path::new(STATS_LOG_FILE).exists();
    let debug_log_exists = std::path::Path::new(DEBUG_LOG_FILE).exists();
    assert_eq!(stats_log_exists, false);
    assert_eq!(debug_log_exists, true);
}

#[test]
#[with_mock_server]
fn test_stats_and_debug_logs() {
    cleanup_files();

    let mock_index = mock(GET, INDEX_PATH).return_status(200).create();
    let mock_error = mock(GET, ERROR_PATH).return_status(503).create();

    let mut config = common::build_configuration();
    config.stats_log_file = STATS_LOG_FILE.to_string();
    config.stats_log_format = "raw".to_string();
    config.no_stats = false;
    config.debug_log_file = DEBUG_LOG_FILE.to_string();
    let _stats = crate::GooseAttack::initialize_with_config(config)
        .setup()
        .unwrap()
        .register_taskset(
            taskset!("LoadTest")
                .register_task(task!(get_index))
                .register_task(task!(get_error)),
        )
        .execute()
        .unwrap();

    let called_index = mock_index.times_called();
    let called_error = mock_error.times_called();

    // Confirm that we loaded the mock endpoints.
    assert_ne!(called_index, 0);
    assert_ne!(called_error, 0);

    // Confirm both the stats and debug logs exist.
    let stats_log_exists = std::path::Path::new(STATS_LOG_FILE).exists();
    let debug_log_exists = std::path::Path::new(DEBUG_LOG_FILE).exists();
    assert_eq!(stats_log_exists, true);
    assert_eq!(debug_log_exists, true);
}
