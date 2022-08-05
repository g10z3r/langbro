use regex::Regex;

lazy_static! {
    pub static ref RE_NAME: Regex = Regex::new(r"^[\p{L}'][ \p{L}'-]*[\p{L}]$").unwrap();
    pub static ref RE_QUERY: Regex = Regex::new(r"^[^._ ](?:[\w-]|\.[\w-])+[^._ ]$").unwrap();
}