use dns_resolver_rust::header::{Flags, Header, QueryType};

fn main() {
    let header_flags = Flags::default()
        .query_or_response(false)
        .kind_of_query(QueryType::Standard)
        .recursion_desired(true);
    let _header = Header::new(12345, header_flags, 1, 0, 0, 0);
}
