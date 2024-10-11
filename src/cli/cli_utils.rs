pub fn validate_label(string: &str) -> Result<String, String> {
    if string.is_empty() {
        Err("labels if provided should not be empty".to_string())
    } else {
        let mut is_v = true;
        let s: Vec<&str> = string.split(',').collect();
        for kv in s {
            if !kv.contains(':') {
                is_v = false;
                break;
            }
        }
        if !is_v {
            return Err(format!(
                "Invalid input {} Labels must be of format key1:value1,key2:value2",
                string
            ));
        }
        Ok(string.to_string())
    }
}
