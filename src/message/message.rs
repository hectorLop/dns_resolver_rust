#![allow(unused)]

pub struct DnsQuery {
    pub header: Header,
    pub question: Question,
}

impl DnsQuery {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut query = self.header.to_bytes();
        query.extend(self.question.to_bytes());
        query
    }
}

pub struct Header {
    pub id: u16,
    pub flags: Flags,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl From<&[u8]> for Header {
    fn from(value: &[u8]) -> Self {
        assert!(
            value.len() == 12,
            "Error parsing a header. Contains more than 12 bytes"
        );
        let id = ((value[0] as u16) << 8) | value[1] as u16;
        let flags = Flags::from(&value[2..4]);
        let num_questions = ((value[4] as u16) << 8) | value[5] as u16;
        let num_answers = ((value[6] as u16) << 8) | value[7] as u16;
        let num_authorities = ((value[8] as u16) << 8) | value[9] as u16;
        let num_additionals = ((value[10] as u16) << 8) | value[10] as u16;

        Self {
            id,
            flags,
            num_questions,
            num_answers,
            num_authorities,
            num_additionals,
        }
    }
}

impl Header {
    pub fn new(
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
pub struct Flags(pub u16);

impl From<&[u8]> for Flags {
    fn from(value: &[u8]) -> Self {
        let value_u16 = ((value[0] as u16) << 8) | value[1] as u16;
        let qr = value_u16 & 0b1000_0000_0000_0000;
        let opcode = QueryType::from(value_u16 & 0b0111_1000_0000_0000);
        let auth_answer = value_u16 & 0b0000_0100_0000_0000;
        let truncation = value_u16 & 0b0000_0010_0000_0000;
        let recursion_available = value_u16 & 0b0000_0001_0000_0000;
        let recursion_desired = value_u16 & 0b0000_0000_1000_0000;
        let response_code = ResponseCode::from(value_u16 & 0b0000_0000_0000_1111);

        Self(value_u16)
    }
}

impl Flags {
    fn to_be_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    pub fn query_or_response(mut self, is_response: bool) -> Self {
        self.0 |= (is_response as u16) << 15;
        self
    }

    pub fn kind_of_query(mut self, query_type: QueryType) -> Self {
        let mask = match query_type {
            QueryType::Standard => 0b0000_0000_0000_0000,
            QueryType::Inverse => 0b0000_1000_0000_0000,
            QueryType::ServerStatusRequest => 0b0001_0000_0000_0000,
            QueryType::Reserved => 0b0001_1000_0000_0000,
            QueryType::Unknown => panic!("Unknown query type"),
        };
        self.0 |= mask;
        self
    }

    pub fn authoritative_answer(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0100_0000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    pub fn truncation(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0010_0000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    pub fn recursion_desired(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0001_0000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    pub fn recursion_available(mut self, value: bool) -> Self {
        let mask = match value {
            true => 0b0000_0000_1000_0000,
            false => 0b0000_0000_0000_0000,
        };
        self.0 |= mask;
        self
    }

    pub fn response_code(mut self, response_code: ResponseCode) -> Self {
        let mask = match response_code {
            ResponseCode::Success => 0b0000_0000_0000_0000,
            ResponseCode::FormatError => 0b0000_0000_0000_0001,
            ResponseCode::ServerFailure => 0b0000_0000_0000_0010,
            ResponseCode::NameError => 0b0000_0000_0000_0011,
            ResponseCode::NotImplemented => 0b0000_0000_0000_0100,
            ResponseCode::Refused => 0b0000_0000_0000_0101,
            ResponseCode::Unknown => panic!("Unknown response code"),
        };
        self.0 |= mask;
        self
    }
}

#[derive(Debug)]
pub enum QueryType {
    Standard,
    Inverse,
    ServerStatusRequest,
    Reserved,
    Unknown,
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            0b0000_0000_0000_0000 => Self::Standard,
            0b0000_1000_0000_0000 => Self::Inverse,
            0b0001_0000_0000_0000 => Self::ServerStatusRequest,
            0b0001_1000_0000_0000 => Self::Reserved,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub enum ResponseCode {
    Success,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    Unknown,
}

impl From<u16> for ResponseCode {
    fn from(value: u16) -> Self {
        match value {
            0b0000_0000_0000_0000 => ResponseCode::Success,
            0b0000_0000_0000_0001 => ResponseCode::FormatError,
            0b0000_0000_0000_0010 => ResponseCode::ServerFailure,
            0b0000_0000_0000_0011 => ResponseCode::NameError,
            0b0000_0000_0000_0100 => ResponseCode::NotImplemented,
            0b0000_0000_0000_0101 => ResponseCode::Refused,
            _ => ResponseCode::Unknown,
        }
    }
}

pub struct Question {
    domain_name: Vec<u8>,
    question_type: QuestionType,
    question_class: QuestionClass,
}

impl Question {
    pub fn new(
        domain_name: &str,
        question_type: QuestionType,
        question_class: QuestionClass,
    ) -> Self {
        Self {
            domain_name: encode_domain_name(domain_name),
            question_type,
            question_class,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut encoded_question = Vec::new();
        encoded_question.extend(self.domain_name.clone());
        encoded_question.extend(self.question_type.to_bytes());
        encoded_question.extend(self.question_class.to_bytes());
        encoded_question
    }
}

pub enum QuestionType {
    Address,
}

impl QuestionType {
    fn to_bytes(&self) -> [u8; 2] {
        match self {
            Self::Address => [0, 1],
        }
    }
}

pub enum QuestionClass {
    Internet,
}

impl QuestionClass {
    fn to_bytes(&self) -> [u8; 2] {
        match self {
            Self::Internet => [0, 1],
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
