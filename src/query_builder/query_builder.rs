use crate::message::message::{
    DnsQuery, Flags, Header, QueryType, Question, QuestionClass, QuestionType,
};

pub fn build_query(domain_name: &str) -> DnsQuery {
    let header_flags = Flags::default()
        .query_or_response(false)
        .kind_of_query(QueryType::Standard)
        .recursion_desired(true);
    let header = Header::new(12345, header_flags, 1, 0, 0, 0);
    let question = Question::new(domain_name, QuestionType::Address, QuestionClass::Internet);

    DnsQuery { header, question }
}
