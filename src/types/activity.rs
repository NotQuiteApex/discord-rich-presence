//! Provides an interface for building activities to send
//! to Discord via [`DiscordIpc::set_activity`](crate::DiscordIpc::set_activity).

use serde_derive::Serialize;
use serde_repr::Serialize_repr;

/// A struct representing a Discord rich presence activity.
///
/// See [Activity Structure](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-structure).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
pub struct Activity {
    /// Current party status or text for a custom status
    pub state: Option<String>,
    /// Details about the player in the activity
    pub details: Option<String>,
    /// Timestamps for the activity
    pub timestamps: Option<Timestamps>,
    /// Information about the current party of the player
    pub party: Option<Party>,
    /// Images for the presence (and their associated hover text)
    pub assets: Option<Assets>,
    /// Secrets associated with the Activity
    pub secrets: Option<Secrets>,
    /// Button(s) settings for the Activity (max: 2)
    pub buttons: Option<Vec<Button>>,
    #[serde(rename = "type")]
    /// Activity type setting
    pub activity_type: Option<ActivityType>,
}
impl Activity {
    /// Creates a new `Activity`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the state of the activity.
    pub fn state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets the details of the activity.
    pub fn details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Add a `Timestamps` to this activity.
    pub fn timestamps(mut self, timestamps: Timestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    /// Add a `Party` to this activity.
    pub fn party(mut self, party: Party) -> Self {
        self.party = Some(party);
        self
    }

    /// Add an `Assets` to this activity.
    pub fn assets(mut self, assets: Assets) -> Self {
        self.assets = Some(assets);
        self
    }

    /// Add a `Secrets` to this activity.
    pub fn secrets(mut self, secrets: Secrets) -> Self {
        self.secrets = Some(secrets);
        self
    }

    /// Add a `Vec` of `Button`s to this activity.
    ///
    /// An activity may contain no more than 2 buttons
    pub fn buttons(mut self, buttons: Vec<Button>) -> Self {
        // API call fails if the array is empty, so we skip serialization
        // entirely if this is the case
        if buttons.is_empty() {
            return self;
        }

        self.buttons = Some(buttons);
        self
    }

    /// Add an `ActivityType` to this activity.
    pub fn activity_type(mut self, activity_type: ActivityType) -> Self {
        self.activity_type = Some(activity_type);
        self
    }
}

/// A struct representing an `Activity`'s timestamps.
///
/// See [Acitivity Timestamps](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-timestamps).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
pub struct Timestamps {
    /// Unix timestamp for the start of the activity
    pub start: Option<i64>,
    /// Unix timestamp for the end of the activity
    pub end: Option<i64>,
}
impl Timestamps {
    /// Creates a new `Timestamps`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the start time.
    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the end time.
    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }
}

/// A struct representing an `Activity`'s game party.
///
/// See [Activity Party](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-party).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
pub struct Party {
    /// ID of the party
    pub id: Option<String>,
    /// Size of the party (current and max)
    pub size: Option<(i32, i32)>,
}
impl Party {
    /// Creates a new `Party`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the ID of the party.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the size of the party (current and maximum).
    pub fn size(mut self, current_size: i32, max_size: i32) -> Self {
        self.size = Some((current_size, max_size));
        self
    }
}

/// A struct representing the art assets and hover text used by an `Activity`.
///
/// See [Activity Assets](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-party).
///
/// Images must be provided in a specific form, see [Activity Asset Image](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-asset-image).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
pub struct Assets {
    /// Image ID or media proxy for the large image
    pub large_image: Option<String>,
    /// Text displayed when hovering over the large image
    pub large_text: Option<String>,
    /// Image ID or media proxy for the small image
    pub small_image: Option<String>,
    /// Text displayed when hovering over the small image
    pub small_text: Option<String>,
}
impl Assets {
    /// Creates a new `Assets`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name or URL of the art asset to be used as the large image.
    pub fn large_image(mut self, large_image: String) -> Self {
        self.large_image = Some(large_image);
        self
    }

    /// Sets the text to be shown when hovering over the large image.
    pub fn large_text(mut self, large_text: String) -> Self {
        self.large_text = Some(large_text);
        self
    }

    /// Sets the name or URL of the art asset to be used as the small image.
    pub fn small_image(mut self, small_image: String) -> Self {
        self.small_image = Some(small_image);
        self
    }

    /// Sets the text that is shown when hovering over the small image.
    pub fn small_text(mut self, small_text: String) -> Self {
        self.small_text = Some(small_text);
        self
    }
}

/// A struct representing the secrets used by an `Activity`.
///
/// See [Activity Secrets](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-secrets).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
pub struct Secrets {
    /// Secret for joining a party
    pub join: Option<String>,
    /// Secret for spectating a game
    pub spectate: Option<String>,
    /// Secret for a specific instanced match
    pub r#match: Option<String>,
}
impl Secrets {
    /// Creates a new `Secrets`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the secret for joining a game party.
    pub fn join(mut self, join: String) -> Self {
        self.join = Some(join);
        self
    }

    /// Sets the secret for spectating a match.
    pub fn spectate(mut self, spectate: String) -> Self {
        self.spectate = Some(spectate);
        self
    }

    /// Sets the secret for a specific, instanced match.
    pub fn r#match(mut self, r#match: String) -> Self {
        self.r#match = Some(r#match);
        self
    }
}

/// A struct representing the buttons that are attached to an `Activity`.
///
/// An activity may have a maximum of 2 buttons.
///
/// See [Activity Buttons](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-buttons).
#[derive(Serialize, Clone)]
pub struct Button {
    /// Text shown on the button (min: 1, max: 32 characters)
    pub label: String,
    /// URL to open when button is clicked (min: 1, max: 512 characters)
    pub url: String,
}
impl Button {
    /// Creates a new `Button` with the given label and
    /// URL
    ///
    /// The label must be 1-32 characters long
    ///
    /// The URL must be 1-512 characters long
    pub fn new(label: String, url: String) -> Self {
        Button { label, url }
    }
}

/// An enum representing the Activity Type of the `Activity`
#[derive(Serialize_repr, Clone)]
#[repr(u8)]
pub enum ActivityType {
    /// Activity type "Playing X"
    Playing = 0,
    /// Activity type "Listening to X"
    Listening = 2,
    /// Activity type "Watching X"
    Watching = 3,
    /// Activity type "Competing in X"
    Competing = 5,
}
