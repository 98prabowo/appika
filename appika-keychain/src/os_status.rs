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
