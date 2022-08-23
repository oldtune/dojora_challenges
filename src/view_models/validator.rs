pub trait ModelValidator {
    fn validate(&self) -> Validity;
}

pub type ErrorMessages<'a> = Vec<&'a str>;

pub enum Validity<'a> {
    Valid,
    Invalid(ErrorMessages<'a>),
}
