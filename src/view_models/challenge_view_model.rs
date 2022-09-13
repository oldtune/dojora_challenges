use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ChallengeViewModel {
    id: String,
    title: String,
    description: Option<String>,
}

impl ChallengeViewModel {
    pub fn new(challenge: crate::model::Challenge) -> Self {
        Self {
            id: challenge.id.to_string(),
            title: challenge.title,
            description: challenge.description,
        }
    }
}
