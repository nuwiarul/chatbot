use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

pub fn apply_system_policy(mut messages: Vec<ChatMessage>) -> Vec<ChatMessage> {
    let policy = ChatMessage {
        role: Role::System,
        content: system_policy_text().to_string(),
    };
    messages.insert(0, policy);
    messages
}

fn system_policy_text() -> &'static str {
    // Keep this short and practical; we can expand later with RAG + guardrails policies.
    r#"You are a helpful assistant.

Formatting requirements (must follow):
- Use Markdown.
- For any code, ALWAYS wrap it in fenced code blocks using triple backticks.
- Put a newline after the opening fence, e.g. ```python\n...code...\n```.
- Never remove spaces between words; write normal readable text.
"#
}

