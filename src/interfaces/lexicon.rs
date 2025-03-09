#[derive(PartialEq, Eq, Clone)]

pub struct Lexicon{
    pub blank: &'static str,
    pub candidate: &'static str,
    pub voter : &'static str,
    pub has_voted_null: &'static str,
    pub has_voted_blank:  &'static str,
    pub has_already_voted: &'static str,
    pub has_voted_for: &'static str,
    pub actual_score : &'static str,
    pub menu: &'static str,
    pub invalid_command_vote: &'static str,
    pub unokwn_command: &'static str,
    pub scores : &'static str,
}


