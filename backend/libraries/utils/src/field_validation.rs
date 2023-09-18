pub enum FieldType{
    Text,
    Url,
    Email,
}

pub fn validate_field_value(field_name: &str, required: bool, max_length: usize, value: &str, field_type: FieldType) -> Result<(), String> {
    if required && value.is_empty() {
        return Err(format!("This field is required."))
    }
    if value.len() > max_length {
        return Err(format!("{} should be less than {} characters.", field_name, max_length));
    }
    match field_type {
        FieldType::Email => {
            if !value.is_empty() && !email_address::EmailAddress::is_valid(value) {
                return Err(format!("Email address is invalid."));
            }
        },
        FieldType::Url => {
            if !value.is_empty() && url::Url::parse(value).is_err() {
                return Err(format!("Url is invalid."));
            }
        },
        FieldType::Text => {},
    }
    Ok(())
}