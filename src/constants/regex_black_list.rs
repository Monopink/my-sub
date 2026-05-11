use lazy_static::lazy_static;

lazy_static! {   
    pub static ref REGEX_BLACK_LIST: Vec<String> = vec!["(.*)*".to_owned()];
}
