use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
    pub fn english() -> Self {
        Self {
            blank: "Blank",
            candidate: "Candidate",
            voter: "Voter",
            has_voted_null: "has cast a null vote",
            has_voted_blank: "has cast a blank vote",
            has_already_voted: "has already voted",
            has_voted_for: "has voted for",
            actual_score: "Current scores",
            menu: r#"
There are 4 available commands:
1) voter Tux Nixos -> Vote for Nixos as Tux
2) voter Tux -> Vote blank as Tux
3) votants -> Show the list of voters
4) scores -> Display candidate scores
"#,
            invalid_command_vote: "Invalid 'voter' command, please specify a voter.",
            unokwn_command: "Unknown command. Please enter a valid command.",
            scores: "Scores",
        }
    }
}
