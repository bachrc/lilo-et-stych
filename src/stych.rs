//! Stych.fr API client implementation

use crate::{
    config::StychConfig,
    error::{AppError, AppResult},
    http,
    logger::{
        log_auth_status, log_operation_failure, log_operation_start, log_operation_success,
        log_request, log_response,
    },
    models::{CourseProposal, PlanningResponse},
};
use reqwest::Client;
use std::time::Duration;

/// Stych.fr API client
#[derive(Debug, Clone)]
pub struct StychClient {
    client: Client,
    config: StychConfig,
}

impl StychClient {
    /// Create a new StychClient instance
    pub fn new(config: StychConfig) -> Self {
        let client = http::create_client().expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Build headers for login requests
    fn build_login_headers(&self) -> AppResult<reqwest::header::HeaderMap> {
        let mut headers = http::get_default_headers();
        headers.insert(
            "Referer",
            "https://www.stych.fr/connexion"
                .parse()
                .map_err(|e| AppError::Network(format!("Invalid header value: {}", e)))?,
        );
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8"
                .parse()
                .map_err(|e| AppError::Network(format!("Invalid header value: {}", e)))?,
        );
        Ok(headers)
    }

    /// Build form data for login request
    fn build_login_form_data(&self) -> Vec<(&str, &str)> {
        vec![
            ("email", &self.config.email),
            ("mdp", &self.config.mdp),
            ("mdp_forgotten", "0"),
            ("remember_me", "0"),
            ("remember_me", "1"),
            ("submit", "Connexion"),
        ]
    }

    /// Build headers with session cookies
    fn build_headers_with_cookies(&self, cookies: &[String]) -> reqwest::header::HeaderMap {
        let mut headers = http::get_default_headers();

        if !cookies.is_empty() {
            let cookie_string = cookies.join("; ");
            headers.insert(
                "Cookie",
                reqwest::header::HeaderValue::from_str(&cookie_string)
                    .unwrap_or_else(|_| reqwest::header::HeaderValue::from_static("")),
            );
        }

        headers
    }

    /// Login to Stych platform and return session cookies
    pub async fn login(&self) -> AppResult<Vec<String>> {
        log_operation_start("Stych login");
        log::info!("Attempting login with email: {}", self.config.email);

        // Build form data and headers
        let form_data = self.build_login_form_data();
        let headers = self.build_login_headers()?;

        let url = "https://www.stych.fr/connexion/0/record3";
        log_request("POST", url);

        // Send login request
        let response = self
            .client
            .post(url)
            .headers(headers)
            .form(&form_data)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let status = response.status().as_u16();
        log_response(status, &response.status().to_string());

        if status == 200 {
            // Extract cookies from response headers
            let cookies: Vec<String> = response
                .headers()
                .get_all("set-cookie")
                .iter()
                .filter_map(|header_value| header_value.to_str().ok().map(|s| s.to_string()))
                .collect();

            // Consume response body
            let _body = response.text().await?;

            log::info!("Extracted {} cookies from login response", cookies.len());
            log_auth_status(true, "Successfully logged in to Stych.fr");
            log_operation_success("Stych login");
            Ok(cookies)
        } else {
            // Get response body for error checking
            let body = response.text().await?;
            let error_msg = format!("Login failed with status {}: {}", status, body);
            log_auth_status(false, &error_msg);
            log_operation_failure("Stych login", &error_msg);
            Err(AppError::Auth(error_msg))
        }
    }

    /// Get planning data using session cookies
    pub async fn get_planning(&self, cookies: &[String]) -> AppResult<PlanningResponse> {
        log_operation_start("Get planning data");
        log::info!("Fetching planning data with {} cookies", cookies.len());

        let url = "https://www.stych.fr/elearning/planning-conduite/get-planning-proposition";
        log_request("GET", url);

        // Send planning request
        let response = self
            .client
            .get(url)
            .headers(self.build_headers_with_cookies(cookies))
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        self.handle_planning_response(response).await
    }

    /// Handle the planning response
    async fn handle_planning_response(
        &self,
        response: reqwest::Response,
    ) -> AppResult<PlanningResponse> {
        let status = response.status().as_u16();
        log_response(status, &response.status().to_string());

        if status == 200 {
            let body = response.text().await?;

            match serde_json::from_str::<PlanningResponse>(&body) {
                Ok(planning_data) => {
                    log::info!(
                        "Successfully retrieved {} course proposals",
                        planning_data.total_courses()
                    );
                    log_operation_success("Get planning data");
                    Ok(planning_data)
                }
                Err(e) => {
                    let error_msg = format!("Failed to parse planning response: {}", e);
                    log_operation_failure("Get planning data", &error_msg);
                    Err(AppError::Parse(error_msg))
                }
            }
        } else {
            let error_msg = format!("Failed to get planning data: HTTP {}", status);
            log_operation_failure("Get planning data", &error_msg);
            Err(AppError::Network(error_msg))
        }
    }

    /// Find the nearest available course from planning data
    pub fn find_nearest_course(courses: &[CourseProposal]) -> Option<CourseProposal> {
        if courses.is_empty() {
            log::info!("No courses available to find nearest");
            return None;
        }

        let nearest = courses.iter().min().cloned();

        if let Some(ref course) = nearest {
            log::info!(
                "Found nearest course on {} at {}",
                course.date_info,
                course.start_time
            );
        }

        nearest
    }

    /// Get and display the nearest course information
    pub async fn get_nearest_course(&self) -> AppResult<Option<CourseProposal>> {
        log_operation_start("Get nearest course");

        // Login and get cookies
        let cookies = self.login().await?;

        // Get planning data
        let planning_data = self.get_planning(&cookies).await?;

        // Find nearest course
        let nearest_course = Self::find_nearest_course(&planning_data.course_proposals);

        match &nearest_course {
            Some(course) => {
                log::info!("Nearest course found: {}", course.description());
                log_operation_success("Get nearest course");
            }
            None => {
                log::info!("No courses found");
                log_operation_success("Get nearest course");
            }
        }

        Ok(nearest_course)
    }

    /// Get all available courses
    pub async fn get_all_courses(&self) -> AppResult<Vec<CourseProposal>> {
        log_operation_start("Get all courses");

        // Login and get cookies
        let cookies = self.login().await?;

        // Get planning data
        let planning_data = self.get_planning(&cookies).await?;

        let courses = planning_data.course_proposals;
        log::info!("Retrieved {} total courses", courses.len());
        log_operation_success("Get all courses");

        Ok(courses)
    }

    /// Find courses by instructor name
    pub async fn find_courses_by_instructor(
        &self,
        instructor_name: &str,
    ) -> AppResult<Vec<CourseProposal>> {
        log_operation_start(&format!("Find courses by instructor: {}", instructor_name));

        // Login and get cookies
        let cookies = self.login().await?;

        // Get planning data
        let planning_data = self.get_planning(&cookies).await?;

        let courses = planning_data.find_courses_by_instructor(instructor_name);
        let course_vec: Vec<CourseProposal> = courses.into_iter().cloned().collect();

        log::info!(
            "Found {} courses for instructor {}",
            course_vec.len(),
            instructor_name
        );
        log_operation_success("Find courses by instructor");

        Ok(course_vec)
    }
}
