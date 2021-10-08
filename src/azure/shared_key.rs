use std::error::Error;

pub struct SharedKeyLiteSignatureOptions {
    verb: String,
    content_md5: String,
    content_type: String,
    date: String,
    canonicalized_headers: String,
    canonicalized_resource: String,
}

/// For more info visit  
/// https://docs.microsoft.com/en-us/rest/api/storageservices/authorize-with-shared-key
pub fn generate_shared_key_token(account_name: &str, options: &SharedKeyLiteSignatureOptions) -> Result<String, Box<dyn Error>> {
    let signature = generate_shared_key_signature(options)?;

    let token = format!("SharedKeyLite {}:{}", account_name, signature);

    Ok(token)
}

fn generate_shared_key_signature(options: &SharedKeyLiteSignatureOptions) -> Result<String, Box<dyn Error>> {
    let value_to_sign = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n", 
        options.verb, 
        options.content_md5, 
        options.content_type, 
        options.date, 
        options.canonicalized_headers, 
        options.canonicalized_resource
    );



    Ok("".into())
}
