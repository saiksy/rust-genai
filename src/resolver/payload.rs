use serde_json::Value;

#[derive(Debug)]
pub struct Payload {
	pub payload: Value,
}

impl Payload {
	pub fn new(payload: Value) -> Self {
		Payload { payload }
	}
}
