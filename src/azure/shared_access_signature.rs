use std::{
    error::Error, 
    time::{Duration, SystemTime}
};

use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

pub fn generate_sas_token(uri: &str, key_name: &str, key_value: &str, time_to_live: &Duration) -> Result<String, Box<dyn Error>> {
    let expire_time = calculate_token_expire_time(time_to_live)?;
    
    let signature = generate_signature_to_encode(&uri, &expire_time)?;

    let encoded_signature = encode_signature(&key_value, &signature)?;

    let token = generate_token(uri, &encoded_signature, &expire_time, key_name)?;

    Ok(token)
}

fn calculate_token_expire_time(time_to_live: &Duration) -> Result<String, Box<dyn Error>> {
    let time_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let expire_time = time_since_epoch + *time_to_live;
    let expire_time_string = expire_time.as_secs().to_string();

    Ok(expire_time_string)
}

fn generate_signature_to_encode(uri: &str, expire_time: &str) -> Result<String, Box<dyn Error>> {
    let signature = format!("{}\n{}", uri, expire_time);
    let url_encoded_signature = urlencoding::encode(&signature).to_string();

    Ok(url_encoded_signature)
}

fn encode_signature(key: &str, signature: &str) -> Result<String, Box<dyn Error>> {
    let key_bytes = key.as_bytes();

    let mut mac = match Hmac::<Sha256>::new_from_slice(key_bytes) {
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

fn generate_token(uri: &str, signature: &str, expire_time: &str, key_name: &str) -> Result<String, Box<dyn Error>> {
    let url_encoded_uri = urlencoding::encode(&uri).to_string();
    let token = format!("SharedAccessSignature sr={}&sig={}&se={}&skn={}", url_encoded_uri, signature, expire_time, key_name);

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_token_expire_time_should_work() -> Result<(), Box<dyn Error>> {
        let time_to_live = Duration::from_secs(15);
        let expire_time = calculate_token_expire_time(&time_to_live)?;
        Ok(())
    }

    #[test]
    fn generate_sas_token_should_work() -> Result<(), Box<dyn Error>> {
        let uri = String::from("https://test.com");
        let key_name = String::from("MasterKey");
        let key_value = String::from("VeryRealSecretThatIsSuperLegit");
        let time_to_live = Duration::from_secs(15);
        let sas_token = generate_sas_token(&uri, &key_name, &key_value, &time_to_live)?;
        Ok(())
    }
}