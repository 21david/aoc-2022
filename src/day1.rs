use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day1 {
    ans: i32
}

impl FromInput for Day1 {
   /*  fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut max = 0;
        let mut counter = 0;
        for line in lines {
            let to_int = line.parse::<i32>();

            match to_int {
                Result::Ok(n) => {
                    counter += n;
                },
                
                Result::Err(_e) => {
                    if counter > max {
                        max = counter;
                    }
                    println!("{}", counter);
                    counter = 0;
                    continue;
                }
            }

            // the last batch
            if counter > max {
                max = counter;
            }
            println!("{}", counter);

        }

        Day1 {
            max
        }
    } */

    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut max = 0;
        let mut counter = 0;
        let mut vec = Vec::new();

        for line in lines {
            let to_int = line.parse::<i32>();

            match to_int {
                Result::Ok(n) => {
                    counter += n;
                },
                
                Result::Err(_e) => {
                    vec.push(counter);
                    counter = 0;
                    continue;
                }
            }
            
            
        }
        
        // the last batch
        vec.push(counter);

        vec.sort();
        
        vec.reverse();

        let top_3 = vec[0] + vec[1] + vec[2];

        Day1 {
            ans: top_3
        }
    
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        // format!("{}", self.max)
        " ".to_string()
    }
    
    fn part_two(&self) -> String {
        format!("{}", self.ans)
    }
}
