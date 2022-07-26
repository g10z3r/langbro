pub(crate) fn validate_query<'a>(query: &str) -> Result<(), validator::ValidationError> {
    if super::regex::RE_QUERY.is_match(query) {
        Ok(())
    } else {
        return Err(validator::ValidationError::new("query"));
    }
}
