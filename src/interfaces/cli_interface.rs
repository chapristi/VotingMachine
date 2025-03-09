use crate::{
    domain::{AttendenceSheet, Scoreboard, VoteOutcome}, 
    storage::Storage, 
    use_cases::{VoteForm, VotingController}
};


use super::lexicon::Lexicon;

fn show_vote_outcome(outcome: VoteOutcome, lexicon: &Lexicon) -> String {
    match outcome {
        VoteOutcome::InvalidVote(voter) => format!("{} {:?}", lexicon.has_voted_null, voter),
        VoteOutcome::BlankVote(voter) => format!("{} {:?}", lexicon.has_voted_blank, voter),
        VoteOutcome::HasAlreadyVoted(voter) => format!("{} {:?}", lexicon.has_already_voted, voter),
        VoteOutcome::AcceptedVote(voter, candidat) => format!("{} {:?} {:?}", lexicon.has_voted_for, voter, candidat),
    }
}

fn show_scoreboard(scoreboard: &Scoreboard, lexicon: &Lexicon) -> String {
    format!("{} : {:?}", lexicon.actual_score, scoreboard)
}

fn show_attendence_sheet(attendence_sheet: &AttendenceSheet, lexicon: &Lexicon) -> String {
    format!("{} : {:?}", lexicon.voter, attendence_sheet)
}

fn display_menu(lexicon: &Lexicon) -> String {
    lexicon.menu.to_string()
}

pub async fn handle_line<Store: Storage>(
    line: &str, 
    controller: &mut VotingController<Store>,
    lexicon: &Lexicon
) -> anyhow::Result<String> {
    let mut words = line.trim().split_whitespace();
    let voting_machine = controller.get_voting_machine().await?;

    let response = match words.next() {
        Some(command) => match command {
            "voter" => match words.next() {
                Some(voter) => {
                    let candidate = words.next().unwrap_or("").to_string();
                    let ballot_paper = VoteForm {
                        voter: voter.to_string(),
                        candidate,
                    };

                    let vote: VoteOutcome = controller.vote(ballot_paper).await?;
                    Ok(show_vote_outcome(vote, lexicon))
                }
                None => Ok(lexicon.invalid_command_vote.to_string()),
            },
            "scores" => Ok(show_scoreboard(voting_machine.get_scoreboard(), lexicon)),
            "votants" => Ok(show_attendence_sheet(voting_machine.get_voters(), lexicon)),
            _ => Ok(lexicon.unokwn_command.to_string()),
        },
        None => Ok(display_menu(lexicon)),
    };

    response
}


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{configuration::Language, domain::{Candidate, VotingMachine}, storages::memory::MemoryStore};
    use std::collections::BTreeMap as Map;
    use std::collections::BTreeSet as Set;
    use super::*;

    #[tokio::test]
    async fn test_display_menu_if_no_command()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!(r#"
Il y a 4 commandes disponibles :
1) voter Tux Nixos -> Voter pour Nixos en tant que Tux
2) voter Tux -> Voter blanc en tant que Tux
3) votants -> Afficher la liste des votants
4) scores -> Afficher les scores des candidats
"#,handle_line("", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_display_voters()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!("Votant : AttendenceSheet({})",handle_line("votants", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_display_scores()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!("Scores actuels : Scoreboard { scores: {Candidate(\"Louis\"): Score(0)}, blank_score: Score(0), invalid_score: Score(0) }",handle_line("scores", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_display_legit_vote()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!("a voté pour Voter(\"Louis\") Candidate(\"Louis\")",handle_line("voter Louis Louis", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }


    #[tokio::test]
    async fn test_display_blank_vote()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!("a voté blanc Voter(\"Louise\")",handle_line("voter Louise", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_vote_command_without_name()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!("Commande 'voter' invalide, veuillez spécifier un électeur.",handle_line("voter", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_vote_invalid_command()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Louis")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!("Commande inconnue. Tapez une commande valide.",handle_line("azertyuiop", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }


}