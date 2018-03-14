
use common::talkscript::*;
use common::objholder::TalkScriptIdx;
use common::gamedata::chara::{CharaId, CharaTalk};
use common::gobj;
use game::{Game, DoPlayerAction};
use text;

/// Hold data for talk handling
pub struct TalkStatus {
    idx: TalkScriptIdx,
    /// The chara that player talks to
    cid: CharaId,
    current_section: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TalkResult {
    Continue, End,
}                    

impl TalkStatus {
    pub fn new(chara_talk: CharaTalk, cid: CharaId, game: &mut Game) -> Option<TalkStatus> {
        let idx = gobj::id_to_idx_checked(&chara_talk.id)?;
        let current_section = chara_talk.section.clone();

        let mut talk_status = TalkStatus {
            idx, cid, current_section,
        };
        talk_status.start(game);
        Some(talk_status)
    }

    pub fn get_text(&self) -> &'static str {
        let tso = gobj::get_obj(self.idx);
        if let Some(text) = tso.get_section_text(&self.current_section) {
            text::talk_txt(&text)
        } else {
            unreachable!()
        }
    }

    pub fn proceed(&mut self, pa: &mut DoPlayerAction, choice: Option<usize>) -> TalkResult {
        self.execute_action(pa.0, choice)
    }

    /// Execute action of current section
    /// If current section has choices, choice must be specified
    fn execute_action(&mut self, _game: &mut Game, _choice: Option<usize>) -> TalkResult {
        let section = self.get_current_section();
        match section {
            TalkReaction::End => { TalkResult::End }
            _ => unimplemented!()
        }
    }

    /// If the first section doesn't have text,
    /// executes action immediately
    fn start(&mut self, game: &mut Game) {
        let section = self.get_current_section();
        if section.text.is_some() { return; }
        self.execute_action(game, None);
    }

    fn get_current_section(&self) -> &'static TalkSection {
        let tso = gobj::get_obj(self.idx);
        &tso.sections[&self.current_section]
    }
}

