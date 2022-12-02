fn main() {
   let mut score_part1: i32 = 0;
   let mut score_part2: i32 = 0;

   for line in include_str!("input.txt").lines() {
        match line {
            "A X" => score_part1 += 1 + 3,
            "A Y" => score_part1 += 2 + 6,
            "A Z" => score_part1 += 3 + 0,
            "B X" => score_part1 += 1 + 0,
            "B Y" => score_part1 += 2 + 3,
            "B Z" => score_part1 += 3 + 6,
            "C X" => score_part1 += 1 + 6,
            "C Y" => score_part1 += 2 + 0,
            "C Z" => score_part1 += 3 + 3,

            _ => println!("Missing line"),
        }
   }

   for line in include_str!("input.txt").lines() {
    match line {
        "A X" => score_part2 += 3 + 0,
        "A Y" => score_part2 += 1 + 3,
        "A Z" => score_part2 += 2 + 6,
        "B X" => score_part2 += 1 + 0,
        "B Y" => score_part2 += 2 + 3,
        "B Z" => score_part2 += 3 + 6,
        "C X" => score_part2 += 2 + 0,
        "C Y" => score_part2 += 3 + 3,
        "C Z" => score_part2 += 1 + 6,

        _ => println!("Missing line"),
    }
}


   println!("Your total score for part 1 is: {}", score_part1);
   println!("Your total score for part 2 is: {}", score_part2);
}
