use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Archetype {
    Regular,
    PrivateMessage,
    Banner,
}

impl Archetype {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::PrivateMessage => "private_message",
            Self::Banner => "banner",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "regular" => Some(Self::Regular),
            "private_message" => Some(Self::PrivateMessage),
            "banner" => Some(Self::Banner),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum PostType {
    Regular = 1,
    ModeratorAction = 2,
    SmallAction = 3,
    Whisper = 4,
}

impl PostType {
    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            1 => Some(Self::Regular),
            2 => Some(Self::ModeratorAction),
            3 => Some(Self::SmallAction),
            4 => Some(Self::Whisper),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum PostActionTypeId {
    Bookmark = 1,
    Like = 2,
    OffTopic = 3,
    Inappropriate = 4,
    Spam = 6,
    Vote = 5,
    Notify_User = 7,
    Notify_Moderators = 8,
}

impl PostActionTypeId {
    pub fn is_flag(self) -> bool {
        matches!(
            self,
            Self::OffTopic | Self::Inappropriate | Self::Spam | Self::Notify_User | Self::Notify_Moderators
        )
    }

    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            1 => Some(Self::Bookmark),
            2 => Some(Self::Like),
            3 => Some(Self::OffTopic),
            4 => Some(Self::Inappropriate),
            5 => Some(Self::Vote),
            6 => Some(Self::Spam),
            7 => Some(Self::Notify_User),
            8 => Some(Self::Notify_Moderators),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum CategoryPermission {
    Full = 1,
    CreatePost = 2,
    ReadOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum TopicTimerType {
    Close = 1,
    Open = 2,
    PublishToCategory = 3,
    Delete = 4,
    Reminder = 5,
    BumpTopic = 6,
    DeleteReplies = 7,
    Silent = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum ReviewableStatus {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
    Ignored = 3,
    Deleted = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum ChatChannelStatus {
    Open = 0,
    ReadOnly = 1,
    Closed = 2,
    Archived = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum PollType {
    Regular = 0,
    Multiple = 1,
    Number = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum PollStatus {
    Open = 0,
    Closed = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum PollResults {
    Always = 0,
    OnVote = 1,
    OnClose = 2,
    StaffOnly = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum DirectoryPeriod {
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
    Quarterly = 4,
    Yearly = 5,
    All = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum SecondFactorMethod {
    Totp = 1,
    SecurityKey = 2,
    Passkey = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum EmailTokenScope {
    Signup = 1,
    PasswordReset = 2,
    EmailLogin = 3,
    EmailUpdate = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum CookMethod {
    Regular = 1,
    RawHtml = 2,
    Email = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum UploadVerificationStatus {
    Unchecked = 1,
    Verified = 2,
    InvalidEtag = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum BookmarkAutoDeletePreference {
    Never = 0,
    WhenReminderSent = 1,
    OnOwnerReply = 2,
    Clear = 3,
    AfterMarkedRead = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum WatchedWordAction {
    Block = 0,
    Censor = 1,
    RequireApproval = 2,
    Flag = 3,
    Link = 4,
    Replace = 5,
    Tag = 6,
    Silence = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum ScreenedActionType {
    Block = 1,
    AllowAdmin = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum GroupVisibility {
    Public = 0,
    LoggedOn = 1,
    Members = 2,
    Staff = 3,
    Owners = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum UserActionType {
    Like = 1,
    WasLiked = 2,
    Bookmark = 3,
    NewTopic = 4,
    Reply = 5,
    Response = 6,
    Mention = 7,
    Quote = 9,
    Edit = 11,
    NewPrivateMessage = 12,
    GotPrivateMessage = 13,
    SolvedProblem = 15,
    AssignedToUser = 16,
}
