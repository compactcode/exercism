use std::collections::BTreeMap;

pub struct ScoreCard {
    wins:   usize,
    draws:  usize,
    losses: usize
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        ScoreCard {
            wins:   0,
            draws:  0,
            losses: 0
        }
    }

    pub fn add_win(&mut self) {
        self.wins += 1;
    }

    pub fn add_loss(&mut self) {
        self.losses += 1;
    }

    pub fn add_draw(&mut self) {
        self.draws += 1;
    }

    pub fn matches(&self) -> usize {
        self.wins + self.draws + self.losses
    }

    pub fn points(&self) -> usize {
        self.wins * 3 + self.draws
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams = BTreeMap::new();

    for match_line in match_results.lines() {
        let match_data: Vec<&str> = match_line.split(';').collect();

        match match_data[2] {
            "win" => {
                teams.entry(match_data[0]).or_insert(ScoreCard::new()).add_win();
                teams.entry(match_data[1]).or_insert(ScoreCard::new()).add_loss();
            },
            "loss" => {
                teams.entry(match_data[0]).or_insert(ScoreCard::new()).add_loss();
                teams.entry(match_data[1]).or_insert(ScoreCard::new()).add_win();
            },
            "draw" => {
                teams.entry(match_data[0]).or_insert(ScoreCard::new()).add_draw();
                teams.entry(match_data[1]).or_insert(ScoreCard::new()).add_draw();
            },
            _ => ()
        }
    }

    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");

    let mut ranked_teams: Vec<(&str, ScoreCard)> = teams.into_iter().collect();
    ranked_teams.sort_by(|(a, card_a), (b, card_b)| card_a.points().cmp(&card_b.points()).reverse().then_with(|| a.cmp(b)));

    for (name, score_card) in ranked_teams {
        result.push_str(
            &format!(
                "\n{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                name,
                score_card.matches(),
                score_card.wins,
                score_card.draws,
                score_card.losses,
                score_card.points()
            )
        );
    }

    return result;
}
