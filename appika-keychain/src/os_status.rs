pub type OSStatus = i32;

fn get_description(status: OSStatus) -> &'static str {
    match status {
        1 => "Unspecified error",
        2 => "Memory error",
        3 => "Param error",
        4 => "Invalid handle",
        5 => "Not found",
        6 => "Access denied",
        7 => "Data invalid",
        8 => "Authentication failed",
        _ => "Unknown error",
    }
}

pub fn error_description(status: OSStatus) -> String {
    format!(
        "Operation failed with OS status code {}: {}",
        status,
        get_description(status)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unspecified_error() {
        let status: OSStatus = 1;
        let desc: &str = "Unspecified error";
        let error: String = format!("Operation failed with OS status code 1: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_memory_error() {
        let status: OSStatus = 2;
        let desc: &str = "Memory error";
        let error: String = format!("Operation failed with OS status code 2: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_parameter_error() {
        let status: OSStatus = 3;
        let desc: &str = "Param error";
        let error: String = format!("Operation failed with OS status code 3: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_invalid_handle_error() {
        let status: OSStatus = 4;
        let desc: &str = "Invalid handle";
        let error: String = format!("Operation failed with OS status code 4: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_not_found_error() {
        let status: OSStatus = 5;
        let desc: &str = "Not found";
        let error: String = format!("Operation failed with OS status code 5: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_access_denied_error() {
        let status: OSStatus = 6;
        let desc: &str = "Access denied";
        let error: String = format!("Operation failed with OS status code 6: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_invalid_data_error() {
        let status: OSStatus = 7;
        let desc: &str = "Data invalid";
        let error: String = format!("Operation failed with OS status code 7: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_authentication_error() {
        let status: OSStatus = 8;
        let desc: &str = "Authentication failed";
        let error: String = format!("Operation failed with OS status code 8: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }

    #[test]
    fn test_unknown_error() {
        let status: OSStatus = 100;
        let desc: &str = "Unknown error";
        let error: String = format!("Operation failed with OS status code 100: {}", desc);
        assert_eq!(get_description(status), desc);
        assert_eq!(error_description(status), error);
    }
}
