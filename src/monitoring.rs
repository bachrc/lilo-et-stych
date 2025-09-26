//! Monitoring service for checking new driving courses

use crate::{
    config::AppConfig,
    error::AppResult,
    logger::{log_operation_start, log_operation_success},
    matrix::MatrixBot,
    models::CourseProposal,
    stych::StychClient,
};

use chrono::Datelike;
use log::{error as log_error, info as log_info};
use matrix_sdk::ruma::OwnedUserId;
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::sleep;

/// Monitoring service for checking new driving courses
pub struct MonitoringService {
    /// Stych client for fetching course data
    stych_client: StychClient,
    /// Matrix bot for sending notifications
    matrix_bot: MatrixBot,
    /// Target user for notifications
    target_user: OwnedUserId,
    /// Seen courses to track new ones
    seen_courses: HashSet<String>,
    /// Check interval in seconds
    check_interval: Duration,
}

impl MonitoringService {
    /// Create a new monitoring service
    pub async fn new(config: AppConfig) -> AppResult<Self> {
        log_operation_start("Monitoring service initialization");

        // Create Matrix bot
        let matrix_bot = MatrixBot::new(config.matrix.clone()).await?;
        matrix_bot.login().await?;

        // Parse target user ID
        let target_user_str = config.get_target_user();
        let target_user = MatrixBot::parse_user_id(target_user_str)?;

        // Create Stych client
        let stych_client = StychClient::new(config.stych.clone());

        // Get check interval from config
        let check_interval = Duration::from_secs(config.monitoring.check_interval_seconds);

        let service = Self {
            stych_client,
            matrix_bot,
            target_user,
            seen_courses: HashSet::new(),
            check_interval,
        };

        log_operation_success("Monitoring service initialization");
        Ok(service)
    }

    /// Start the monitoring loop
    pub async fn start_monitoring(&mut self) -> AppResult<()> {
        log_info!(
            "Starting monitoring service with interval: {:?}",
            self.check_interval
        );

        // Initial fetch to populate seen courses
        self.fetch_and_process_courses().await?;

        loop {
            sleep(self.check_interval).await;
            log_info!("Checking for new courses...");

            match self.fetch_and_process_courses().await {
                Ok(_) => log_info!("Course check completed successfully"),
                Err(e) => {
                    log_error!("Error checking for courses: {}", e);
                    // Try to send error notification
                    if let Err(notify_err) = self
                        .matrix_bot
                        .send_error_message(&self.target_user, &e.to_string())
                        .await
                    {
                        log_error!("Failed to send error notification: {}", notify_err);
                    }
                }
            }
        }
    }

    /// Fetch courses and process new ones
    async fn fetch_and_process_courses(&mut self) -> AppResult<()> {
        // Get all courses using the convenience method
        let courses = self.stych_client.get_all_courses().await?;

        // Filter courses for 2025
        let courses_2025: Vec<CourseProposal> = courses
            .into_iter()
            .filter(|course| {
                if let Ok(course_date) =
                    chrono::NaiveDate::parse_from_str(&course.date_info, "%Y-%m-%d")
                {
                    course_date.year() == 2025
                } else {
                    false
                }
            })
            .collect();

        log_info!("Found {} courses for 2025", courses_2025.len());

        // Check for new courses
        let mut new_courses = Vec::new();

        for course in &courses_2025 {
            // Create a unique identifier for the course
            let course_id = format!(
                "{}_{}_{}",
                course.date_info, course.start_time, course.user_id
            );

            // If we haven't seen this course before, it's new
            if self.seen_courses.insert(course_id) {
                new_courses.push(course.clone());
            }
        }

        // Notify about new courses
        if !new_courses.is_empty() {
            log_info!("Found {} new courses", new_courses.len());
            for course in new_courses {
                self.notify_new_course(&course).await?;
            }
        }

        Ok(())
    }

    /// Notify about a new course
    async fn notify_new_course(&self, course: &CourseProposal) -> AppResult<()> {
        log_info!("Notifying about new course: {}", course.description());

        let message = format!(
            "🚗 Nouveau cours de conduite disponible !\n{}",
            course.description()
        );

        self.matrix_bot
            .send_message(&self.target_user, &message)
            .await
    }
}
