mod message;
pub mod query_builder;
pub mod response_parser;

pub use query_builder::query_builder::build_query;
pub use response_parser::parser::{get_header_info, parse_resource_record};
