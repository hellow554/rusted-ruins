
use std::collections::HashMap;
use std::borrow::Cow;

/// Hold data of one taliking
#[derive(Serialize, Deserialize)]
pub struct TalkScriptObject {
    pub id: String,
    pub sections: HashMap<String, TalkSection>,
}

impl TalkScriptObject {
    /// Get text id of given section
    pub fn get_section_text<'a>(&'a self, section: &str) -> Option<Cow<'a, str>> {
        match self.sections[section] {
            TalkSection::Normal { ref text, .. } =>  {
                let s = if let Some(ref text) = *text {
                    Cow::Borrowed(text.as_ref())
                } else {
                    Cow::Owned(format!("{}.{}", self.id, section))
                };
                Some(s)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkSection {
    Normal {
        text: Option<String>,
        answer_texts: Vec<String>,
        dest_sections: Vec<String>,
        default_dest_section: Option<String>,
    },
    Reaction {
        reaction: TalkReaction,
        next_section: String,
    },
    Special {
        special: SpecialTalkSection,
        next_section: String,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub enum TalkSectionKind {
    Normal, Reaction, Special,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TalkReaction {
}

/// This holds data to represent special talk section.
#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialTalkSection {
    /// Taught new ruins and dungeons locations by the informant
    InformantRuins,
}

