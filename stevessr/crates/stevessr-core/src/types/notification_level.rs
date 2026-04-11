use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum NotificationLevel {
    Muted = 0,
    Regular = 1,
    Tracking = 2,
    Watching = 3,
    WatchingFirstPost = 4,
}

impl NotificationLevel {
    pub fn from_i16(v: i16) -> Option<Self> {
        match v {
            0 => Some(Self::Muted),
            1 => Some(Self::Regular),
            2 => Some(Self::Tracking),
            3 => Some(Self::Watching),
            4 => Some(Self::WatchingFirstPost),
            _ => None,
        }
    }

    pub fn as_i16(self) -> i16 {
        self as i16
    }

    pub fn should_notify_on_new_post(self) -> bool {
        matches!(self, Self::Watching)
    }

    pub fn should_notify_on_first_post(self) -> bool {
        matches!(self, Self::Watching | Self::WatchingFirstPost)
    }

    pub fn should_track(self) -> bool {
        self >= Self::Tracking
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i16)]
pub enum NotificationType {
    Mentioned = 1,
    Replied = 2,
    Quoted = 3,
    Edited = 4,
    Liked = 5,
    PrivateMessage = 6,
    InvitedToPrivateMessage = 7,
    InviteeAccepted = 8,
    Posted = 9,
    MovedPost = 10,
    Linked = 11,
    GrantedBadge = 12,
    InvitedToTopic = 13,
    Custom = 14,
    GroupMentioned = 15,
    GroupMessageSummary = 16,
    WatchingFirstPost = 17,
    TopicReminder = 18,
    LikedConsolidated = 19,
    PostApproved = 20,
    CodeReviewCommitApproved = 21,
    MembershipRequestAccepted = 22,
    MembershipRequestConsolidated = 23,
    BookmarkReminder = 24,
    Reaction = 25,
    VotesReleased = 26,
    EventReminder = 27,
    EventInvitation = 28,
    ChatMention = 29,
    ChatMessage = 30,
    ChatInvitation = 31,
    ChatGroupMention = 32,
    ChatWatchedThread = 33,
    AssignedToPost = 34,
    NewFeatures = 35,
    AdminProblems = 36,
}

impl NotificationType {
    pub fn from_i16(v: i16) -> Option<Self> {
        // Safety: we check bounds
        if (1..=36).contains(&v) {
            Some(unsafe { std::mem::transmute(v) })
        } else {
            None
        }
    }

    pub fn as_i16(self) -> i16 {
        self as i16
    }

    pub fn is_high_priority(self) -> bool {
        matches!(
            self,
            Self::PrivateMessage
                | Self::BookmarkReminder
                | Self::AdminProblems
                | Self::ChatMention
                | Self::ChatMessage
        )
    }
}
