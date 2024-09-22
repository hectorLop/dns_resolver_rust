pub struct ResourceRecord<'a> {
    pub name: &'a [u8],
    pub data_type: &'a [u8],
    pub class: &'a [u8],
    pub ttl: &'a [u8],
    pub length: &'a [u8],
    pub data: &'a [u8],
}

impl<'a> From<&'a [u8]> for ResourceRecord<'a> {
    fn from(value: &'a [u8]) -> Self {
        let name_delimiter = extract_name_last_idx(value);
        let name = &value[..name_delimiter];
        let data_type = &value[name_delimiter..name_delimiter + 2];
        let class = &value[name_delimiter + 2..name_delimiter + 4];
        let ttl = &value[name_delimiter + 4..name_delimiter + 8];
        let length = &value[name_delimiter + 8..name_delimiter + 10];
        let data = &value[name_delimiter + 10..];

        Self {
            name,
            data_type,
            class,
            ttl,
            length,
            data,
        }
    }
}

fn extract_name_last_idx(data: &[u8]) -> usize {
    for (idx, byte) in data.iter().enumerate() {
        if *byte == 0 {
            return idx + 1;
        }
    }
    panic!("There must be a null byte delimiting the name");
}
