//! Types for the *m.room.name* event.

use std::time::SystemTime;

use ruma_identifiers::{EventId, RoomId, UserId};
use serde::{Deserialize, Serialize};

use crate::{InvalidInput, TryFromRaw, UnsignedData};

/// The payload for `NameEvent`.
#[derive(Clone, Debug, Serialize)]
pub struct NameEventContent {
    /// The name of the room. This MUST NOT exceed 255 bytes.
    pub(crate) name: Option<String>,
}

impl TryFromRaw for NameEventContent {
    type Raw = raw::NameEventContent;

    type Err = InvalidInput;

    fn try_from_raw(raw: raw::NameEventContent) -> Result<Self, Self::Err> {
        match raw.name {
            None => Ok(NameEventContent { name: None }),
            Some(name) => NameEventContent::new(name),
        }
    }
}

impl NameEventContent {
    /// Create a new `NameEventContent` with the given name.
    ///
    /// # Errors
    ///
    /// `InvalidInput` will be returned if the name is more than 255 bytes.
    pub fn new(name: String) -> Result<Self, InvalidInput> {
        match name.len() {
            0 => Ok(Self { name: None }),
            1..=255 => Ok(Self { name: Some(name) }),
            _ => Err(InvalidInput(
                "a room name cannot be more than 255 bytes".to_string(),
            )),
        }
    }

    /// The name of the room, if any.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_ref)
    }
}

pub(crate) mod raw {
    use super::*;

    /// The payload of a `NameEvent`.
    #[derive(Clone, Debug, Deserialize)]
    pub struct NameEventContent {
        /// The name of the room. This MUST NOT exceed 255 bytes.
        // The spec says "A room with an m.room.name event with an absent, null, or empty name field
        // should be treated the same as a room with no m.room.name event."
        #[serde(default, deserialize_with = "ruma_serde::empty_string_as_none")]
        pub(crate) name: Option<String>,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        convert::TryFrom,
        iter::FromIterator,
        time::{Duration, UNIX_EPOCH},
    };

    use js_int::Int;
    use matches::assert_matches;
    use ruma_identifiers::{EventId, RoomId, UserId};
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use crate::{EventJson, UnsignedData};

    use super::NameEventContent;

    #[test]
    fn serialization_with_optional_fields_as_none() {
        let name_event = NameEvent {
            content: NameEventContent {
                name: Some("The room name".to_string()),
            },
            event_id: EventId::try_from("$h29iv0s8:example.com").unwrap(),
            origin_server_ts: UNIX_EPOCH + Duration::from_millis(1),
            prev_content: None,
            room_id: None,
            sender: UserId::try_from("@carl:example.com").unwrap(),
            state_key: "".to_string(),
            unsigned: UnsignedData::default(),
        };

        let actual = to_json_value(&name_event).unwrap();
        let expected = json!({
            "content": {
                "name": "The room name"
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn serialization_with_all_fields() {
        let name_event = NameEvent {
            content: NameEventContent {
                name: Some("The room name".to_string()),
            },
            event_id: EventId::try_from("$h29iv0s8:example.com").unwrap(),
            origin_server_ts: UNIX_EPOCH + Duration::from_millis(1),
            prev_content: Some(NameEventContent {
                name: Some("The old name".to_string()),
            }),
            room_id: Some(RoomId::try_from("!n8f893n9:example.com").unwrap()),
            sender: UserId::try_from("@carl:example.com").unwrap(),
            state_key: "".to_string(),
            unsigned: UnsignedData {
                age: Some(Int::from(100)),
                ..UnsignedData::default()
            },
        };

        let actual = to_json_value(&name_event).unwrap();
        let expected = json!({
            "content": {
                "name": "The room name"
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "prev_content": { "name": "The old name" },
            "room_id": "!n8f893n9:example.com",
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name",
            "unsigned": {
                "age": 100
            }
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn absent_field_as_none() {
        let json_data = json!({
            "content": {},
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });
        assert_eq!(
            from_json_value::<EventJson<NameEvent>>(json_data)
                .unwrap()
                .deserialize()
                .unwrap()
                .content
                .name,
            None
        );
    }

    #[test]
    fn name_fails_validation_when_too_long() {
        // "XXXX .." 256 times
        let long_string: String = String::from_iter(std::iter::repeat('X').take(256));
        assert_eq!(long_string.len(), 256);

        let long_content_json = json!({ "name": &long_string });
        let from_raw: EventJson<NameEventContent> = from_json_value(long_content_json).unwrap();

        let result = from_raw.deserialize();
        assert!(result.is_err(), "Result should be invalid: {:?}", result);
    }

    #[test]
    fn json_with_empty_name_creates_content_as_none() {
        let long_content_json = json!({ "name": "" });
        let from_raw: EventJson<NameEventContent> = from_json_value(long_content_json).unwrap();
        assert_matches!(
            from_raw.deserialize().unwrap(),
            NameEventContent { name: None }
        );
    }

    #[test]
    fn new_with_empty_name_creates_content_as_none() {
        assert_matches!(
            NameEventContent::new(String::new()).unwrap(),
            NameEventContent { name: None }
        );
    }

    #[test]
    fn null_field_as_none() {
        let json_data = json!({
            "content": {
                "name": null
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });
        assert_eq!(
            from_json_value::<EventJson<NameEvent>>(json_data)
                .unwrap()
                .deserialize()
                .unwrap()
                .content
                .name,
            None
        );
    }

    #[test]
    fn empty_string_as_none() {
        let json_data = json!({
            "content": {
                "name": ""
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });
        assert_eq!(
            from_json_value::<EventJson<NameEvent>>(json_data)
                .unwrap()
                .deserialize()
                .unwrap()
                .content
                .name,
            None
        );
    }

    #[test]
    fn nonempty_field_as_some() {
        let name = Some("The room name".to_string());
        let json_data = json!({
            "content": {
                "name": "The room name"
            },
            "event_id": "$h29iv0s8:example.com",
            "origin_server_ts": 1,
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.name"
        });

        assert_eq!(
            from_json_value::<EventJson<NameEvent>>(json_data)
                .unwrap()
                .deserialize()
                .unwrap()
                .content
                .name,
            name
        );
    }
}
