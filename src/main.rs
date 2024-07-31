mod header;

use crate::header as header_mod;

fn main() {
    println!("Hello, world!");
    let header_flags = header_mod::Flags::new()
        .query_or_response(false)
        .kind_of_query(header_mod::QueryType::Standard)
        .recursion_desired(true);
    let _header = header_mod::Header::new(12345, header_flags, 1, 0, 0, 0);
}
