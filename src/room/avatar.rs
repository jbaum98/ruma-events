//! Types for the *m.room.avatar* event.

use ruma_events_macros::{FromRaw, StateEventContent};
use serde::Serialize;
use serde_json::value::RawValue as RawJsonValue;

use super::ImageInfo;
use crate::{
    error::{InvalidEvent, InvalidEventKind},
    EventContent, EventJson, RoomEventContent, StateEventContent,
};

/// A picture that is associated with the room.
///
/// This can be displayed alongside the room information.
#[derive(Clone, Debug, Serialize, FromRaw, StateEventContent)]
#[ruma_event(type = "m.room.avatar")]
pub struct AvatarEventContent {
    /// Information about the avatar image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ImageInfo>,

    /// Information about the avatar thumbnail image.
    /// URL of the avatar image.
    pub url: String,
}
