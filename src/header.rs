#![allow(unused)]

pub struct DnsQuery {
    header: Header,
    question: Question,
}

impl DnsQuery {
    pub fn build_query(domain_name: &str) -> Self {
        let header_flags = Flags::default()
            .query_or_response(false)
            .kind_of_query(QueryType::Standard)
            .recursion_desired(true);
        let header = Header::new(12345, header_flags, 1, 0, 0, 0);
        let question = Question::new(domain_name, QuestionType::Address, QuestionClass::Internet);

        Self { header, question }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut query = self.header.to_bytes();
        query.extend(self.question.to_bytes());
        query
    }
}

struct Header {
    id: u16,
    flags: Flags,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

impl Header {
    fn new(
        id: u16,
        flags: Flags,
        num_questions: u16,
        num_answers: u16,
        num_authorities: u16,
        num_additionals: u16,
    ) -> Self {
        Self {
            id,
            flags,
            num_questions,
            num_answers,
            num_authorities,
            num_additionals,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend(self.id.to_be_bytes());
        bytes.extend(self.flags.to_be_bytes());
        bytes.extend(self.num_questions.to_be_bytes());
        bytes.extend(self.num_answers.to_be_bytes());
        bytes.extend(self.num_authorities.to_be_bytes());
        bytes.extend(self.num_additionals.to_be_bytes());
        bytes
    }
}

#[derive(Default)]
struct Flags(u16);

impl Flags {
    fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    fn query_or_response(mut self, is_response: bool) -> Self {
        self.0 |= (is_response as u16) << 15;
        self
    }

    fn kind_of_query(mut self, query_type: QueryType) -> Self {
        let mask = match query_type {
            QueryType::Standard => 0b0000_0000_0000_0000,
            QueryType::Inverse => 0b0000_1000_0000_0000,
            QueryType::ServerStatusRequest => 0b0001_0000_0000_0000,
            QueryType::Reserved => 0b0001_1000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    fn authoritative_answer(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0100_0000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    fn truncation(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0010_0000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    fn recursion_desired(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0001_0000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    fn recursion_available(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0000_1000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    fn response_code(mut self, response_code: ResponseCode) -> Self {
        let mask = match response_code {
            ResponseCode::Success => 0b0000_0000_0000_0000,
            ResponseCode::FormatError => 0b0000_0000_0000_0001,
            ResponseCode::ServerFailure => 0b0000_0000_0000_0010,
            ResponseCode::NameError => 0b0000_0000_0000_0011,
            ResponseCode::NotImplemented => 0b0000_0000_0000_0100,
            ResponseCode::Refused => 0b0000_0000_0000_0101,
        };
        self.0 |= mask;
        self
    }
}

enum QueryType {
    Standard,
    Inverse,
    ServerStatusRequest,
    Reserved,
}

enum ResponseCode {
    Success,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}

struct Question {
    domain_name: Vec<u8>,
    question_type: QuestionType,
    question_class: QuestionClass,
}

impl Question {
    fn new(domain_name: &str, question_type: QuestionType, question_class: QuestionClass) -> Self {
        Self {
            domain_name: encode_domain_name(domain_name),
            question_type,
            question_class,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut encoded_question = Vec::new();
        encoded_question.extend(self.domain_name.clone());
        encoded_question.push(self.question_type.to_bytes());
        encoded_question.push(self.question_class.to_bytes());
        encoded_question
    }
}

enum QuestionType {
    Address,
}

impl QuestionType {
    fn to_bytes(&self) -> u8 {
        match self {
            Self::Address => 1,
        }
    }
}

enum QuestionClass {
    Internet,
}

impl QuestionClass {
    fn to_bytes(&self) -> u8 {
        match self {
            Self::Internet => 1,
        }
    }
}

fn encode_domain_name(domain_name: &str) -> Vec<u8> {
    let mut encoded_domain = Vec::new();
    for label in domain_name.split(".").map(|x| x.as_bytes()) {
        encoded_domain.push(label.len() as u8);
        encoded_domain.extend(label);
    }
    encoded_domain.push(0);
    encoded_domain
}

#[cfg(test)]
mod tests {
    use super::encode_domain_name;

    #[test]
    fn test_encode_domain_name() {
        let domain_name = "www.google.com";
        let expected = vec![
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        assert_eq!(encode_domain_name(domain_name), expected);
    }
}
