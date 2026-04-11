use serde::{Deserialize, Serialize};

/// Plugin API version for backward compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
}

impl ApiVersion {
    pub const CURRENT: ApiVersion = ApiVersion { major: 1, minor: 0 };

    pub fn is_compatible(&self, required: &ApiVersion) -> bool {
        self.major == required.major && self.minor >= required.minor
    }

    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            1 => Some(ApiVersion {
                major: parts[0].parse().ok()?,
                minor: 0,
            }),
            2 => Some(ApiVersion {
                major: parts[0].parse().ok()?,
                minor: parts[1].parse().ok()?,
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
