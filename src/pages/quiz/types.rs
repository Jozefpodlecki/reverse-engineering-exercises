use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub struct QuizConfig {
    pub questions_count: usize,
}

#[derive(PartialEq, Clone)]
pub enum QuizUiState {
    Idle,
    Loading,
    Active,
    Completed,
    Error(String),
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct QuizSession {
    pub datasets: Vec<String>,
    pub total_questions: usize,
    pub current_index: usize,
    pub score: usize,
    pub started_at: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice(MultipleChoiceQuestion),
    Select(SelectQuestion),
    Reorder(ReorderQuestion),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, )]
pub struct Question {
    pub id: u32,
    pub dataset_id: u32,
    pub kind: QuestionType,
    pub created_on: DateTime<Utc>,
    pub title: String,
    pub points: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    pub options: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectQuestion {
    pub options: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ReorderQuestion {
    pub order_items: Vec<String>,
}