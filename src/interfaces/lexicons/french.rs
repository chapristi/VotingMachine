use crate::interfaces::lexicon::Lexicon;

impl Lexicon {
    pub fn french() -> Self {
        Self {
            blank: "Blanc",
            candidate: "Candidat",
            voter: "Votant",
            has_voted_null: "a voté nul",
            has_voted_blank: "a voté blanc",
            has_already_voted: "a déjà voté",
            has_voted_for: "a voté pour",
            actual_score: "Scores actuels",
            menu: r#"
Il y a 4 commandes disponibles :
1) voter Tux Nixos -> Voter pour Nixos en tant que Tux
2) voter Tux -> Voter blanc en tant que Tux
3) votants -> Afficher la liste des votants
4) scores -> Afficher les scores des candidats
"#,
            invalid_command_vote: "Commande 'voter' invalide, veuillez spécifier un électeur.",
            unokwn_command: "Commande inconnue. Tapez une commande valide.",
            scores: "Scores",
        }
    }
}
