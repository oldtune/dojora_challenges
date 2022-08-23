#[derive(serde::Serialize)]
pub struct ChallengeCreated {
    title: String,
    description: Option<String>,
}

impl ChallengeCreated {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self { title, description }
    }
}
