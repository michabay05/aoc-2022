use crate::read_file;

pub fn solution() {
    let contents = read_file("inputs/day2.txt");
    let split_words = contents.trim().split("\n");

    let mut total_score = 0;
    for word in split_words {
        let mut choices = word.split_ascii_whitespace();
        let opp_choice = choices.next().unwrap();
        let expected_outcome = choices.next().unwrap();
        let mut my_expected_choice = "";
        
        if opp_choice == "A" {
            if expected_outcome == "X" {
                my_expected_choice = "Z";
            } else if expected_outcome == "Y" {
                my_expected_choice = "X";
            } else if expected_outcome == "Z" {
                my_expected_choice = "Y";
            }
        } else if opp_choice == "B" {
            if expected_outcome == "X" {
                my_expected_choice = "X";
            } else if expected_outcome == "Y" {
                my_expected_choice = "Y";
            } else if expected_outcome == "Z" {
                my_expected_choice = "Z";
            }
        } else if opp_choice == "C" {
            if expected_outcome == "X" {
                my_expected_choice = "Y";
            } else if expected_outcome == "Y" {
                my_expected_choice = "Z";
            } else if expected_outcome == "Z" {
                my_expected_choice = "X";
            }
        }

        total_score += match my_expected_choice {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0
        };

        // Win or Lose score
        if (opp_choice == "A" && my_expected_choice == "Y") || (opp_choice == "B" && my_expected_choice == "Z") || (opp_choice == "C" && my_expected_choice == "X") {
            total_score += 6;
        } else if (opp_choice == "A" && my_expected_choice == "X") || (opp_choice == "B" && my_expected_choice == "Y") || (opp_choice == "C" && my_expected_choice == "Z") {
            total_score += 3;
        }
    }
    println!("My total score: {}", total_score);
}
