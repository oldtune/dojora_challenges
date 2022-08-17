use serde::Serialize;

#[derive(Serialize)]
pub struct ChallengeViewModel {
    pub title: String,
    pub description: String,
}
