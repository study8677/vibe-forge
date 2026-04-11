use stevessr_core::error::{Error, Result, ValidationErrors};

/// Maximum upload size in bytes (default 10MB).
const MAX_UPLOAD_SIZE: i64 = 10 * 1024 * 1024;
/// Maximum image upload size (default 20MB).
const MAX_IMAGE_SIZE: i64 = 20 * 1024 * 1024;

/// Allowed file extensions.
const ALLOWED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "svg", "ico",
    "heic", "heif", "avif",
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
    "txt", "md", "csv", "json", "xml",
    "zip", "tar", "gz", "7z", "rar",
    "mp3", "mp4", "webm", "ogg", "wav",
];

/// Blocked file extensions that should never be uploaded.
const BLOCKED_EXTENSIONS: &[&str] = &[
    "exe", "bat", "cmd", "com", "msi", "scr", "pif",
    "js", "vbs", "wsf", "ps1", "sh", "bash",
    "dll", "sys", "drv",
    "php", "asp", "aspx", "jsp", "cgi",
];

pub struct UploadValidator;

impl UploadValidator {
    pub fn validate(filename: &str, content_type: &str, file_size: i64) -> Result<()> {
        let mut errors = ValidationErrors::new();

        // Check file size
        let max_size = if content_type.starts_with("image/") {
            MAX_IMAGE_SIZE
        } else {
            MAX_UPLOAD_SIZE
        };

        if file_size > max_size {
            errors.add("file_size", format!(
                "file is too large ({} bytes); maximum is {} bytes",
                file_size, max_size
            ));
        }

        if file_size == 0 {
            errors.add("file_size", "file is empty");
        }

        // Check extension
        let extension = filename
            .rsplit('.')
            .next()
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if extension.is_empty() {
            errors.add("filename", "file must have an extension");
        } else if BLOCKED_EXTENSIONS.contains(&extension.as_str()) {
            errors.add("filename", format!("files with .{} extension are not allowed", extension));
        } else if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
            errors.add("filename", format!("files with .{} extension are not allowed", extension));
        }

        // Validate content type is not empty
        if content_type.is_empty() {
            errors.add("content_type", "content type must be specified");
        }

        // Validate filename length
        if filename.len() > 255 {
            errors.add("filename", "filename is too long (max 255 characters)");
        }

        // Check for path traversal attempts
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            errors.add("filename", "filename contains invalid characters");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }
}
