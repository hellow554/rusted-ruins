
use fnv::FnvHashMap;
use common::basic::SKILL_EXP_LVUP;
use common::gamedata::*;

pub trait SkillListEx {
    fn add_exp(&mut self, kind: SkillKind, add_exp: u32, base_level: u16) -> (bool, u32);
    fn learn_new_skill(&mut self, kind: SkillKind);
    fn set_skill_level(&mut self, kind: SkillKind, lv: u16);
}

impl SkillListEx for SkillList {
    /// Add exp to specified skill
    /// Returns level up result and actual added exp value
    fn add_exp(&mut self, kind: SkillKind, add_exp: u32, base_level: u16) -> (bool, u32) {
        if self.exp.is_none() { return (false, 0); }
        // Adjusting by base_level
        let skill_level = if let Some(skill_level) = self.skills.get(&kind) {
            *skill_level
        } else {
            return (false, 0);
        };
        let add_exp = (add_exp as f32 * search_adjust_coeff(base_level, skill_level)) as u32;

        // Add exp
        if let Some(ref mut exp) = self.exp {
            if let Some(skill_exp) = exp.get_mut(&kind) {
                let is_level_up;
                let add_exp = if add_exp > SKILL_EXP_LVUP as u32 { // Exp is limited per time
                    SKILL_EXP_LVUP as u32
                } else {
                    add_exp
                };
                let sum = *skill_exp as u32 + add_exp;
                *skill_exp = if sum >= SKILL_EXP_LVUP.into() { // Level up
                    if let Some(skill_level) = self.skills.get_mut(&kind) {
                        *skill_level += 1;
                    }
                    is_level_up = true;
                    0
                } else {
                    is_level_up = false;
                    sum as u16
                };
                return (is_level_up, add_exp);
            }
        }
        (false, 0)
    }

    /// Insert new skill slot
    fn learn_new_skill(&mut self, kind: SkillKind) {
        if !self.skills.contains_key(&kind) {
            self.skills.insert(kind, 1);
        }
        if self.exp.is_none() {
            self.exp = Some(FnvHashMap::default());
        }
        if let Some(ref mut exp) = self.exp {
            if !exp.contains_key(&kind) {
                exp.insert(kind, 0);
            }
        } else {
            unreachable!();
        }
    }
    
    /// Set skill level directly. Do not add exp.
    fn set_skill_level(&mut self, kind: SkillKind, lv: u16) {
        self.skills.insert(kind, lv);
    }
}

fn search_adjust_coeff(base_level: u16, skill_level: u16) -> f32 {
    use rules::RULES;
    let diff = skill_level as isize - base_level as isize;
    let i = RULES.exp.begin_adjust_coeff - diff;
    let adjust_coeff = &RULES.exp.adjust_coeff;
    assert!(!adjust_coeff.is_empty());
    if i < 0 {
        adjust_coeff[0]
    } else if i as usize >= RULES.exp.adjust_coeff.len() {
        *adjust_coeff.last().unwrap()
    } else {
        adjust_coeff[i as usize]
    }
}

