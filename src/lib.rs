use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Supported contestant programming languages inside tyyoj-judger sandbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SubmissionLanguage {
    #[serde(rename = "cpp20")]
    Cpp20,
    #[serde(rename = "pascal")]
    Pascal,
    #[serde(rename = "python3")]
    Python3,
}

/// Supported judging modes for competitive programming tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum JudgeMode {
    Standard,
    CustomChecker,
    Interactive,
}

/// Official submission verdicts supported by tyyoj pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SubmissionVerdict {
    Pending,
    Compiling,
    Running,
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    CompilationError,
    IdlenessLimitExceeded,
}

/// Brief problem summary returned in list views.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProblemSummary {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub time_limit_ms: u64,
    pub memory_limit_mb: u64,
    pub judge_mode: JudgeMode,
}

/// Submission request sent by contestant.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSubmissionRequest {
    pub problem_id: Uuid,
    pub language: SubmissionLanguage,
    pub source_code: String,
}

/// Dynamic judging task dispatched from backend to judger worker via Redis queue.
/// Carries dynamic per-problem resource limits (strictly non-hardcoded).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JudgeTask {
    pub submission_id: Uuid,
    pub problem_id: Uuid,
    pub language: SubmissionLanguage,
    pub judge_mode: JudgeMode,
    pub source_code: String,
    pub time_limit_ms: u64,
    pub memory_limit_mb: u64,
}

/// Heartbeat message broadcast by persistent tyyoj-judger containers.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JudgerHeartbeat {
    pub worker_id: String,
    pub container_id: String,
    pub timestamp: DateTime<Utc>,
    pub is_busy: bool,
    pub current_job_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verdict_serialization() {
        let verdict = SubmissionVerdict::Accepted;
        let serialized = serde_json::to_string(&verdict).unwrap();
        assert_eq!(serialized, "\"Accepted\"");
    }

    #[test]
    fn test_language_serialization() {
        let lang = SubmissionLanguage::Cpp20;
        let serialized = serde_json::to_string(&lang).unwrap();
        assert_eq!(serialized, "\"cpp20\"");
    }

    #[test]
    fn test_dynamic_problem_limits_in_judge_task() {
        let task = JudgeTask {
            submission_id: Uuid::new_v4(),
            problem_id: Uuid::new_v4(),
            language: SubmissionLanguage::Cpp20,
            judge_mode: JudgeMode::Standard,
            source_code: "int main() {}".into(),
            time_limit_ms: 1500,
            memory_limit_mb: 512,
        };
        assert_eq!(task.time_limit_ms, 1500);
        assert_eq!(task.memory_limit_mb, 512);
    }
}
