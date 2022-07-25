use regex::Regex;
use validator::ValidationError;

pub(crate) fn validate_query<'a>(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[^._ ](?:[\w-]|\.[\w-])+[^._ ]$").unwrap();
    if re.is_match(username) {
        Ok(())
    } else {
        return Err(ValidationError::new("username"));
    }
}
