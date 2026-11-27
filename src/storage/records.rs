use std::io::{Cursor, Read};

// Represent a value stored in a database record
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

// Represents one database record
pub struct Record {
    pub values: Vec<Value>,
}

impl Record {
    // Convert the record into bytes
    pub fn serialize(&self) -> Vec<u8> {
        // The byte array
        let mut bytes = Vec::new();

        // Store how many values this row contains
        let count = self.values.len() as u32;
        bytes.extend_from_slice(&count.to_le_bytes());

        // Serialize each value
        for value in &self.values {
            match value {
                Value::Integer(number) => {
                    bytes.push(1);
                    bytes.extend_from_slice(&number.to_le_bytes());
                }

                Value::Float(number) => {
                    bytes.push(1);
                    bytes.extend_from_slice(&number.to_le_bytes());
                }

                Value::Text(text) => {
                    bytes.push(3);

                    let text_bytes = text.as_bytes();
                    let length = text_bytes.len() as u32;

                    bytes.extend_from_slice(&length.to_le_bytes());
                    bytes.extend_from_slice(text_bytes);
                }

                Value::Boolean(value) => {
                    bytes.push(4);
                    bytes.push(*value as u8);
                }

                Value::Null => {
                    bytes.push(5);
                }
            }
        }

        bytes
    }

    // Convert serialized bytes back into a database record
    pub fn deserialize(bytes: &[u8]) -> std::io::Result<Record> {
        // Read bytes sequentially
        let mut cursor = Cursor::new(bytes);

        // Read the number of values in this row
        let mut count_bytes = [0u8; 4];
        cursor.read_exact(&mut count_bytes)?;

        let count = u32::from_le_bytes(count_bytes) as usize;

        // Store the values recovered from the bytes
        let mut values = Vec::new();

        // Read each value.
        for _ in 0..count {
            // Read the value's type marker
            let mut marker = [0u8; 1];
            cursor.read_exact(&mut marker)?;

            let value = match marker[0] {
                // Integer
                1 => {
                    let mut data = [0u8; 8];
                    cursor.read_exact(&mut data)?;

                    Value::Integer(i64::from_le_bytes(data))
                }

                // Float
                2 => {
                    let mut data = [0u8; 8];
                    cursor.read_exact(&mut data)?;

                    Value::Float(f64::from_le_bytes(data))
                }

                // Text
                3 => {
                    let mut length_bytes = [0u8; 4];
                    cursor.read_exact(&mut length_bytes)?;

                    let length = u32::from_le_bytes(length_bytes) as usize;

                    // Check that the text fits in the remaining data
                    let remaining = bytes.len() as u64 - cursor.position();

                    if length as u64 > remaining {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Text length exceeds available data",
                        ));
                    }

                    let mut text_bytes = vec![0u8; length];
                    cursor.read_exact(&mut text_bytes)?;

                    let text = String::from_utf8(text_bytes).map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 text")
                    })?;

                    Value::Text(text)
                }

                // Boolean
                4 => {
                    let mut data = [0u8; 1];
                    cursor.read_exact(&mut data)?;

                    match data[0] {
                        0 => Value::Boolean(false),
                        1 => Value::Boolean(true),
                        _ => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid boolean value",
                            ));
                        }
                    }
                }

                // Null
                5 => Value::Null,

                // Unknown type marker
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unknown value type",
                    ));
                }
            };

            values.push(value);
        }

        // Reject unexpected trailing bytes
        if cursor.position() != bytes.len() as u64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unexpected trailing data",
            ));
        }

        Ok(Record { values })
    }
}
