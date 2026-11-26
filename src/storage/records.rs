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
}
