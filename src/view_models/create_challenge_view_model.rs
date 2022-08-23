use serde::{Deserialize, Serialize};

use crate::helpers::string_helpers;

use super::validator::{ModelValidator, Validity};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChallengeViewModel {
    pub title: String,
    pub description: String,
}

impl ModelValidator for CreateChallengeViewModel {
    fn validate(&self) -> super::validator::Validity {
        let mut result = vec![];

        let empty = |string: &str| string_helpers::empty_string(string);
        let empty_title = empty(&self.title);

        if empty_title {
            result.push("Title should not be empty");
        }

        match empty_title {
            true => Validity::Invalid(result),
            false => Validity::Valid,
        }
    }
}
