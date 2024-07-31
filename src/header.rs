pub struct Header {
    id: u16,
    flags: Flags,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl Header {
    pub fn new(
        id: u16,
        flags: Flags,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Self {
        Self {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }
}

pub struct Flags(u16);

impl Flags {
    pub fn new() -> Self {
        Flags(0)
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

    pub fn recursion_desired(mut self, value: bool) -> Self {
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

pub enum QueryType {
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
