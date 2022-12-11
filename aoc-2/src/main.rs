// A is rock - X 1
// B is paper - Y 2
// C is scissors - Z 3


// lose: 0
// draw: 3
// win: 6

use std::fs;


#[derive(Eq, PartialEq)]
enum Choice
{
    Rock,
    Paper,
    Scissors
}

#[derive(Eq, PartialEq)]
enum Outcome
{
    Win,
    Draw,
    Lose
}


struct Decider
{
    pub score: u32
}

impl Decider {
    fn new() -> Self { Self { score: 0 } }

    pub fn add_round_raw(&mut self, their_choice: &str, outcome: &str) -> () {
        let expected_outcome: Outcome = match outcome  {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("{}", outcome)
        };

        let opponent_choice: Choice = match their_choice {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!()
        };


        let my_choice = match (&expected_outcome, opponent_choice) {
            (Outcome::Win, Choice::Rock)        => Choice::Paper, 
            (Outcome::Win, Choice::Paper)       => Choice::Scissors,
            (Outcome::Win, Choice::Scissors)    => Choice::Rock,
            (Outcome::Draw, y)        => y,
            (Outcome::Lose, Choice::Rock)        => Choice::Scissors, 
            (Outcome::Lose, Choice::Paper)       => Choice::Rock,
            (Outcome::Lose, Choice::Scissors)    => Choice::Paper,
        };

        self.score += match expected_outcome
        {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };
        self.score += match my_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

}


struct Winner
{
    pub score: u32
}

impl Winner {

    fn new() -> Winner
    {
        Self { score: 0 }
    }
    
    fn get_outcome(&self, my_choice: &Choice, opponent_choice: &Choice) -> Outcome
    {
        if my_choice == opponent_choice {
            return Outcome::Draw;
        }

        match (my_choice, opponent_choice) {
            (Choice::Rock, Choice::Rock) => Outcome::Draw,
            (Choice::Rock, Choice::Paper) => Outcome::Lose,
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Paper, Choice::Paper) => Outcome::Draw,
            (Choice::Paper, Choice::Scissors) => Outcome::Lose,
            (Choice::Scissors, Choice::Rock) => Outcome::Lose,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
        }
    }

    pub fn add_round_raw(&mut self, my_choice: &str, opponent_choice: &str) 
    {
        let my_choice: Choice = match my_choice {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!()
        };

        let opponent_choice: Choice = match opponent_choice {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!()
        };

        self.add_round(&my_choice, &opponent_choice);
    }

    fn add_round(&mut self, my_choice: &Choice, opponent_choice: &Choice) -> ()
    {
        self.score += match self.get_outcome(&my_choice, &opponent_choice)
        {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };
        self.score += match my_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

fn main() {
    let mut winner = Winner::new();
    let mut decider = Decider::new();


    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file");


    for line in contents.lines()
    {
        let split: Vec<&str> = line.split_whitespace().collect();
        winner.add_round_raw(split[1], split[0]);
        decider.add_round_raw(split[0], split[1]);
    }

    println!("{}", winner.score);
    println!("{}", decider.score);
}
