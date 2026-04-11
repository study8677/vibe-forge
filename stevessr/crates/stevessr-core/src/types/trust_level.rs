use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum TrustLevel {
    New = 0,
    Basic = 1,
    Member = 2,
    Regular = 3,
    Leader = 4,
}

impl TrustLevel {
    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(Self::New),
            1 => Some(Self::Basic),
            2 => Some(Self::Member),
            3 => Some(Self::Regular),
            4 => Some(Self::Leader),
            _ => None,
        }
    }

    pub fn as_i16(self) -> i16 {
        self as i16
    }

    pub fn can_create_topic(self) -> bool {
        self >= Self::New
    }

    pub fn can_reply(self) -> bool {
        self >= Self::New
    }

    pub fn can_flag(self) -> bool {
        self >= Self::Basic
    }

    pub fn can_send_pm(self) -> bool {
        self >= Self::Basic
    }

    pub fn can_upload_images(self) -> bool {
        self >= Self::Basic
    }

    pub fn can_post_links(self) -> bool {
        self >= Self::New
    }

    pub fn can_edit_wiki(self) -> bool {
        self >= Self::Basic
    }

    pub fn can_tag_topics(self) -> bool {
        self >= Self::Member
    }

    pub fn can_invite(self) -> bool {
        self >= Self::Member
    }

    pub fn can_create_group_pm(self) -> bool {
        self >= Self::Member
    }

    pub fn is_leader(self) -> bool {
        self >= Self::Leader
    }
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::New => write!(f, "new user"),
            Self::Basic => write!(f, "basic user"),
            Self::Member => write!(f, "member"),
            Self::Regular => write!(f, "regular"),
            Self::Leader => write!(f, "leader"),
        }
    }
}
