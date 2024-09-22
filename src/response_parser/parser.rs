use crate::message::{
    message::{Flags, Header, QueryType, ResponseCode},
    resource_record::ResourceRecord,
};

pub fn get_header_info(raw_data: &[u8]) {
    let header = Header::from(raw_data);
    println!("Num answers: {}", header.num_answers);
    get_flags_info(&header.flags);
}

fn get_flags_info(flags: &Flags) {
    let qr = (flags.0 & 0b1000_0000_0000_0000) >> 15;
    match qr {
        0 => println!("QR: Query"),
        1 => println!("QR: Response"),
        _ => panic!("QR can only be 0 or 1"),
    };

    let opcode = QueryType::from(flags.0 & 0b0111_1000_0000_0000);
    println!("OPCODE: {opcode:?}");

    let auth_answer = (flags.0 & 0b0000_0100_0000_0000) >> 10;
    match auth_answer {
        0 => println!("AA: Not authority"),
        1 => println!("AA: Authority response"),
        _ => panic!("AA can only be 0 or 1"),
    };

    let truncation = (flags.0 & 0b0000_0010_0000_0000) >> 9;
    match truncation {
        0 => println!("TC: Not Truncated"),
        1 => println!("TC: Truncated"),
        _ => panic!("TC can only be 0 or 1"),
    };

    let recursion_available = (flags.0 & 0b0000_0001_0000_0000) >> 8;
    match recursion_available {
        0 => println!("RA: Recursion not available"),
        1 => println!("RA: Recursion available"),
        _ => panic!("RA can only be 0 or 1"),
    };

    let recursion_desired = (flags.0 & 0b0000_0000_1000_0000) >> 7;
    match recursion_desired {
        0 => println!("RD: Recursion not desired"),
        1 => println!("RD: Recursion desired"),
        _ => panic!("RD can only be 0 or 1"),
    };

    let response_code = ResponseCode::from(flags.0 & 0b0000_0000_0000_1111);
    println!("Response code: {response_code:?}");
}

pub fn parse_resource_record(raw_data: &[u8]) {
    let resource_record = ResourceRecord::from(raw_data);
    let name = decode_name(resource_record.name);
    println!("Name: {name}");
}

fn decode_name(encoded_name: &[u8]) -> String {
    let mut ptr = 0;
    let mut labels = Vec::new();

    while encoded_name[ptr] != 0 {
        let length = encoded_name[ptr] as usize;
        let label = &encoded_name[ptr + 1..ptr + 1 + length];
        ptr += 1 + length;
        labels.push(std::str::from_utf8(label).expect("Failure parsing name into string"));
    }

    labels.join(".")
}

#[cfg(test)]
mod tests {
    use super::decode_name;

    #[test]
    fn test_decode_name() {
        let input = vec![
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        let expected = String::from("www.google.com");

        assert_eq!(decode_name(&input), expected);
    }
}
