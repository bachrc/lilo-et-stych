//! Matrix bot functionality for sending notifications

use crate::{
    config::MatrixConfig,
    error::{AppError, AppResult},
    logger::{log_operation_start, log_operation_success},
    models::CourseProposal,
};
use matrix_sdk::{
    config::SyncSettings,
    ruma::{events::room::message::RoomMessageEventContent, OwnedUserId, UserId},
    Client,
};
use std::sync::Arc;

/// Matrix bot for sending notifications
#[derive(Debug, Clone)]
pub struct MatrixBot {
    client: Arc<Client>,
    config: MatrixConfig,
}

impl MatrixBot {
    /// Create a new Matrix bot instance
    pub async fn new(config: MatrixConfig) -> AppResult<Self> {
        log_operation_start("Matrix bot initialization");

        let client = Client::builder()
            .homeserver_url(&config.homeserver.url)
            .build()
            .await
            .map_err(|e| AppError::Matrix(format!("Failed to build Matrix client: {}", e)))?;

        let bot = Self {
            client: Arc::new(client),
            config,
        };

        log_operation_success("Matrix bot initialization");
        Ok(bot)
    }

    /// Login to Matrix homeserver
    pub async fn login(&self) -> AppResult<()> {
        log_operation_start("Matrix login");

        log::info!("Logging in as {}", self.config.bot.username);

        self.client
            .matrix_auth()
            .login_username(&self.config.bot.username, &self.config.bot.password)
            .send()
            .await
            .map_err(|e| AppError::Matrix(format!("Login failed: {}", e)))?;

        log::info!("Successfully logged in as {}", self.config.bot.username);
        log_operation_success("Matrix login");

        Ok(())
    }

    /// Get or create a direct message room with a user
    pub async fn get_or_create_dm_room(
        &self,
        user_id: &OwnedUserId,
    ) -> AppResult<matrix_sdk::Room> {
        log_operation_start(&format!("Get or create DM room with {}", user_id));

        // Sync the client to ensure we have the latest state
        log::debug!("Syncing Matrix client...");
        self.client
            .sync_once(SyncSettings::default())
            .await
            .map_err(|e| AppError::Matrix(format!("Sync failed: {}", e)))?;

        // Get all rooms the client knows about
        let rooms = self.client.rooms();
        log::debug!("Searching among {} known rooms...", rooms.len());

        // Look for a DM room with the target user
        for room in rooms {
            if room.is_direct().await.unwrap_or(false) {
                log::debug!("Room {} is a DM room", room.room_id());

                // Check if the target user is a member of this room
                if let Ok(Some(_member)) = room.get_member(user_id).await {
                    log::info!(
                        "Found existing DM room with {}: {}",
                        user_id,
                        room.room_id()
                    );
                    log_operation_success("Get or create DM room");
                    return Ok(room);
                }
            }
        }

        // No existing DM room found, create a new one
        log::info!(
            "No existing DM room found with {}, creating new one",
            user_id
        );
        let room = self
            .client
            .create_dm(user_id)
            .await
            .map_err(|e| AppError::Matrix(format!("Failed to create DM room: {}", e)))?;

        log::info!("Created new DM room: {}", room.room_id());
        log_operation_success("Get or create DM room");

        Ok(room)
    }

    /// Send a text message to a user
    pub async fn send_message(&self, user_id: &OwnedUserId, message: &str) -> AppResult<()> {
        log_operation_start(&format!("Send message to {}", user_id));

        let room = self.get_or_create_dm_room(user_id).await?;

        log::debug!("Sending message: {}", message);
        room.send(RoomMessageEventContent::text_plain(message))
            .await
            .map_err(|e| AppError::Matrix(format!("Failed to send message: {}", e)))?;

        log::info!("Successfully sent message to {}", user_id);
        log_operation_success("Send message");

        Ok(())
    }

    /// Send course information to a user
    pub async fn send_course_info(
        &self,
        user_id: &OwnedUserId,
        course: &CourseProposal,
    ) -> AppResult<()> {
        let message = format!(
            "🚗 Nearest driving course:\n📅 Date: {}\n🕐 Time: {} - {}\n👨‍🏫 Instructor: {}",
            course.date_info, course.start_time, course.end_time, course.instructor
        );

        self.send_message(user_id, &message).await
    }

    /// Send a notification about no available courses
    pub async fn send_no_courses_message(&self, user_id: &OwnedUserId) -> AppResult<()> {
        self.send_message(user_id, "No driving courses available at the moment.")
            .await
    }

    /// Send an error notification
    pub async fn send_error_message(&self, user_id: &OwnedUserId, error: &str) -> AppResult<()> {
        let message = format!("❌ Error retrieving course information: {}", error);
        self.send_message(user_id, &message).await
    }

    /// Parse a user ID string
    pub fn parse_user_id(user_id_str: &str) -> AppResult<OwnedUserId> {
        UserId::parse(user_id_str)
            .map_err(|e| AppError::Matrix(format!("Invalid user ID '{}': {}", user_id_str, e)))
    }
}

/// Convenience function to send course notification via Matrix
pub async fn send_course_notification(
    matrix_config: MatrixConfig,
    user_id_str: &str,
    course: Option<&CourseProposal>,
    error: Option<&str>,
) -> AppResult<()> {
    let bot = MatrixBot::new(matrix_config).await?;
    bot.login().await?;

    let user_id = MatrixBot::parse_user_id(user_id_str)?;

    match (course, error) {
        (Some(course), None) => bot.send_course_info(&user_id, course).await,
        (None, Some(err)) => bot.send_error_message(&user_id, err).await,
        (None, None) => bot.send_no_courses_message(&user_id).await,
        (Some(_), Some(_)) => {
            // Both course and error provided, prefer course info
            bot.send_course_info(&user_id, course.unwrap()).await
        }
    }
}
