use std::{fmt::Display, iter};

use serde::{Deserialize, Serialize};

use crate::{Content, Document, PreferenceKey, PreferenceValue, Reference, Series};

/// A liturgical template that can carry a set of possible preferences and
/// other metadata, as well as sub-documents.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Liturgy {
    pub preferences: LiturgyPreferences,
    pub evening: bool,
    pub body: Series,
}

impl Liturgy {
    #[must_use]
    pub fn evening(mut self, is_evening_liturgy: bool) -> Self {
        self.evening = is_evening_liturgy;
        self
    }

    #[must_use]
    pub fn preferences<T>(mut self, preferences: T) -> Self
    where
        T: Into<LiturgyPreferences>,
    {
        self.preferences = preferences.into();
        self
    }
}

impl From<Series> for Liturgy {
    fn from(body: Series) -> Self {
        Self {
            evening: false,
            preferences: LiturgyPreferences::default(),
            body,
        }
    }
}

impl<T, U> From<T> for Liturgy
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self {
            evening: false,
            preferences: LiturgyPreferences::default(),
            body: Series::from(items),
        }
    }
}

/// The set of preferences and decisions that can be set for a given [Liturgy],
/// including their descriptions and all the possible options for them.
#[derive(Clone, Default, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiturgyPreferences(Vec<LiturgyPreference>);

impl LiturgyPreferences {
    pub fn iter(&self) -> impl Iterator<Item = &LiturgyPreference> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // allow this because if we try to actually implement IntoIter, it causes a conflict with the `From` implementation
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> impl Iterator<Item = LiturgyPreference> {
        self.0.into_iter()
    }

    pub fn default_value_for_key(&self, key: &PreferenceKey) -> Option<&PreferenceValue> {
        let pref = self.0.iter().find(|pref| pref.key == *key);
        pref.map(|pref| {
            pref.default_value
                .as_ref()
                .unwrap_or(&pref.first_choice.value)
        })
    }
}

impl<T> From<T> for LiturgyPreferences
where
    T: IntoIterator<Item = LiturgyPreference>,
{
    fn from(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// A particular preference or decision that can be set for a given [Liturgy],
/// including its description and any of the possible options for them.

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiturgyPreference {
    pub key: PreferenceKey,
    /// A descriptive name for this preference
    pub label: String,
    /// A default value for this preference, if the user has not chosen a value.
    pub default_value: Option<PreferenceValue>,
    /// Because the choices cannot be empty, it's helpful to separate the first field.
    /// This will be the default selection, if no default is provided and no value is chosen.
    first_choice: LiturgyPreferenceOption,
    /// All the other choices
    choices: Vec<LiturgyPreferenceOption>,
    /// The descriptive name of a category into which it may be grouped with other preferences.
    pub category: Option<String>,
    /// Descriptive text, if necessary for understanding what the preference is meant to do.
    pub description: Option<String>,
    /// A citation for further reading, if helpful.
    pub reference: Option<Reference>,
}

impl<L, C> From<(PreferenceKey, L, C)> for LiturgyPreference
where
    L: Display,
    C: IntoIterator<Item = LiturgyPreferenceOption>,
{
    fn from((key, label, choices): (PreferenceKey, L, C)) -> Self {
        let label = label.to_string();
        let mut choices = choices.into_iter();
        let first_choice = choices
            .next()
            .expect("Must include at least one LiturgyPreferenceOption");
        let choices = choices.collect();
        Self {
            key,
            label,
            first_choice,
            choices,
            default_value: None,
            category: None,
            description: None,
            reference: None,
        }
    }
}

impl LiturgyPreference {
    pub fn choices(&self) -> impl Iterator<Item = &LiturgyPreferenceOption> {
        iter::once(&self.first_choice).chain(self.choices.iter())
    }

    pub fn only_one_choice(&self) -> bool {
        self.choices.is_empty()
    }

    #[must_use]
    pub fn category<T>(mut self, category: T) -> Self
    where
        T: Display,
    {
        self.category = Some(category.to_string());
        self
    }

    #[must_use]
    pub fn default_value(mut self, default_value: PreferenceValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    #[must_use]
    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Display,
    {
        self.description = Some(description.to_string());
        self
    }

    #[must_use]
    pub fn reference(mut self, reference: Reference) -> Self {
        self.reference = Some(reference);
        self
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiturgyPreferenceOption {
    pub label: String,
    pub value: PreferenceValue,
    /// A fallback value for this preference, which can be used if a satisfactory result is not found
    /// using the previous value. (This is used, for example, in canticle tables.)
    pub fallback_value: Option<PreferenceValue>,
}

impl LiturgyPreferenceOption {
    #[must_use]
    pub fn fallback_value(mut self, fallback_value: Option<PreferenceValue>) -> Self {
        self.fallback_value = fallback_value;
        self
    }
}

impl<T> From<(T, PreferenceValue)> for LiturgyPreferenceOption
where
    T: Display,
{
    fn from((label, value): (T, PreferenceValue)) -> Self {
        Self {
            label: label.to_string(),
            value,
            fallback_value: None,
        }
    }
}

impl<T> From<T> for LiturgyPreferenceOption
where
    T: Display + Into<PreferenceValue>,
{
    fn from(option: T) -> Self {
        Self {
            label: option.to_string(),
            value: option.into(),
            fallback_value: None,
        }
    }
}

// Conversions
impl From<Content> for Liturgy {
    fn from(content: Content) -> Self {
        match content {
            Content::Liturgy(c) => c,
            Content::Series(c) => Self::from(c),
            _ => Self::from(Series::from(vec![Document::from(content)])),
        }
    }
}
