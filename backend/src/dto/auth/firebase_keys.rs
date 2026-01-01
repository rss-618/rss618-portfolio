use jsonwebtoken::DecodingKey;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FirebaseKeys {
    pub keys: HashMap<String, DecodingKey>,
}
