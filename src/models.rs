//! Data models for Stych.fr API responses

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Response structure for planning data from Stych.fr API
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlanningResponse {
    /// List of available course proposals
    #[serde(rename = "rowsProposition")]
    pub course_proposals: Vec<CourseProposal>,
    /// List of available instructors
    #[serde(rename = "rowsMoniteur")]
    pub instructors: Vec<Instructor>,
}

/// Represents a driving course proposal
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CourseProposal {
    /// Date information for the course (format: YYYY-MM-DD)
    #[serde(rename = "info_date")]
    pub date_info: String,
    /// Start time of the course (format: HH:MM)
    #[serde(rename = "heure_debut")]
    pub start_time: String,
    /// End time of the course (format: HH:MM)
    #[serde(rename = "heure_fin")]
    pub end_time: String,
    /// Name of the instructor for this course
    #[serde(rename = "moniteur")]
    pub instructor: String,
    /// User ID of the instructor
    #[serde(rename = "id_user")]
    pub user_id: String,
}

/// Represents a driving instructor
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Instructor {
    /// User ID of the instructor
    #[serde(rename = "id_utilisateur")]
    pub user_id: String,
    /// First name of the instructor
    #[serde(rename = "prenom")]
    pub first_name: String,
    /// Last name initial of the instructor
    #[serde(rename = "initiale_nom")]
    pub last_name_initial: String,
}

impl CourseProposal {
    /// Create a new course proposal
    pub fn new(
        date_info: String,
        start_time: String,
        end_time: String,
        instructor: String,
        user_id: String,
    ) -> Self {
        Self {
            date_info,
            start_time,
            end_time,
            instructor,
            user_id,
        }
    }

    /// Get a formatted description of the course
    pub fn description(&self) -> String {
        format!(
            "📅 Date: {}\n🕐 Time: {} - {}\n👨‍🏫 Instructor: {}",
            self.date_info, self.start_time, self.end_time, self.instructor
        )
    }

    /// Check if this course is on a specific date
    pub fn is_on_date(&self, date: &NaiveDate) -> bool {
        if let Ok(course_date) = NaiveDate::parse_from_str(&self.date_info, "%Y-%m-%d") {
            course_date == *date
        } else {
            false
        }
    }
}

impl Instructor {
    /// Create a new instructor
    pub fn new(user_id: String, first_name: String, last_name_initial: String) -> Self {
        Self {
            user_id,
            first_name,
            last_name_initial,
        }
    }

    /// Get the instructor's full name with initial
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name_initial)
    }
}

impl PlanningResponse {
    /// Create a new planning response
    pub fn new(course_proposals: Vec<CourseProposal>, instructors: Vec<Instructor>) -> Self {
        Self {
            course_proposals,
            instructors,
        }
    }

    /// Find the nearest available course based on date
    pub fn find_nearest_course(&self) -> Option<&CourseProposal> {
        if self.course_proposals.is_empty() {
            return None;
        }

        self.course_proposals.iter().min()
    }

    /// Find all courses for a specific instructor
    pub fn find_courses_by_instructor(&self, instructor_name: &str) -> Vec<&CourseProposal> {
        self.course_proposals
            .iter()
            .filter(|course| course.instructor == instructor_name)
            .collect()
    }

    /// Find all courses on a specific date
    pub fn find_courses_by_date(&self, date: &NaiveDate) -> Vec<&CourseProposal> {
        self.course_proposals
            .iter()
            .filter(|course| course.is_on_date(date))
            .collect()
    }

    /// Get the total number of available courses
    pub fn total_courses(&self) -> usize {
        self.course_proposals.len()
    }

    /// Get the number of unique instructors
    pub fn unique_instructors(&self) -> usize {
        self.instructors.len()
    }

    /// Find all courses in a specific year
    pub fn find_courses_by_year(&self, year: i32) -> Vec<&CourseProposal> {
        self.course_proposals
            .iter()
            .filter(|course| {
                if let Ok(course_date) = NaiveDate::parse_from_str(&course.date_info, "%Y-%m-%d") {
                    course_date.year() == year
                } else {
                    false
                }
            })
            .collect()
    }
}

impl PartialEq for CourseProposal {
    fn eq(&self, other: &Self) -> bool {
        self.date_info == other.date_info
            && self.start_time == other.start_time
            && self.user_id == other.user_id
    }
}

impl Eq for CourseProposal {}

impl PartialOrd for CourseProposal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CourseProposal {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by date, then by start time
        match self.date_info.cmp(&other.date_info) {
            Ordering::Equal => self.start_time.cmp(&other.start_time),
            other => other,
        }
    }
}
