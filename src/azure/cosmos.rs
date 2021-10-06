use std::error::Error;

use chrono::Utc;
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

pub enum ResourceType {
    Databases,
    Collections,
    Documents
}

pub enum Verb {
    Get,
    Post,
    Put,
    Delete,
}

pub fn generate_cosmos_token(verb: Verb, resource_type: ResourceType, resource_id: &str, key: &str, key_type: &str, token_version: &str) -> Result<String, Box<dyn Error>> {
    let signature = generate_signature_to_encode(verb, resource_type, resource_id)?;
    let encoded_signature = encode_signature(key, &signature)?;
    let token = generate_token(key_type, token_version, &encoded_signature)?;

    Ok(token)
}

fn generate_signature_to_encode(verb: Verb, resource_type: ResourceType, resource_id: &str) -> Result<String, Box<dyn Error>> {
    let verb = match verb {
        Verb::Delete => "DELETE",
        Verb::Get => "GET",
        Verb::Post => "POST",
        Verb::Put => "PUT",
    };

    let resource_type = match resource_type {
        ResourceType::Collections => "colls",
        ResourceType::Databases => "dbs",
        ResourceType::Documents => "docs",
    };

    let date = Utc::now().to_rfc2822();

    let signature = format!("{}\n{}\n{}\n{}\n\n", verb, resource_type, resource_id, date);

    Ok(signature)
}

fn encode_signature(key: &str, signature: &str) -> Result<String, Box<dyn Error>> {
    let key_bytes = base64::decode(key)?;

    let mut mac = match Hmac::<Sha256>::new_from_slice(&key_bytes) {
        Ok(value) => value,
        Err(_) => return Err(Box::from("Failed to create hmac-sha256")),
    };

    let signature_bytes = signature.as_bytes();

    mac.update(signature_bytes);

    let result = mac.finalize();

    let encoded_bytes = result.into_bytes().to_vec();

    let encoded = base64::encode(encoded_bytes);

    Ok(encoded)
}

fn generate_token(key_type: &str, token_version: &str, signature: &str) -> Result<String, Box<dyn Error>> {
    let token = format!("type={}&ver={}&sig={}", key_type, token_version, signature);
    let encoded_token = urlencoding::encode(&token).to_string();

    Ok(encoded_token)
}