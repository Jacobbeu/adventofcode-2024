use crate::{Solution, SolutionPair};
use std::error::Error;

pub fn solve() -> SolutionPair {

    let contents = include_str!("../../input/day04_input.txt");

    let solution1 = xmas_word_search_2(&contents).expect("Failed to parse word search.");
    let solution2 = 0;

    (Solution::from(solution1), Solution::from(solution2))
}

#[derive(Debug)]
struct Token {
    letter: char,
    line: i32,
    position: i32,
}

impl Token {
    fn token_exists(&self, dest_token: &Token, direction: &Direction) -> bool {

        let mut target_line = self.line;
        let mut target_position = self.position;

        match direction {
            Direction::Forward => target_position += 1,
            Direction::Backwards => target_position -= 1,
            Direction::Up => target_line -= 1,
            Direction::Down => target_line += 1,
            Direction::UpLeft => { target_line -= 1; target_position -= 1; },
            Direction::UpRight => { target_line -= 1; target_position += 1; },
            Direction::DownLeft => { target_line += 1; target_position -= 1; },
            Direction::DownRight => { target_line += 1; target_position += 1; },
        }

        if dest_token.line == target_line && dest_token.position == target_position {
            true
        }
        else {
            false
        }
    }
}

enum Direction {
    Forward,
    Backwards,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn xmas_word_search_2(data: &str) -> Result<i32, Box<dyn Error>> {

    let mut word_search = Vec::new();

    let mut line_counter = 0;

    for line in data.lines() {
        let mut char_counter = 0;

        let mut row = Vec::new();
        for char in line.chars() {
            row.push(Token { letter: char, line: line_counter, position: char_counter});
            char_counter += 1;
        }
        word_search.push(row);
        line_counter += 1;
    }

    let mut total = 0;
    for row in word_search.iter() {
        for token in row.iter() {
            if xmas_search(token, &word_search, Direction::Forward) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::Backwards) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::Up) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::Down) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::UpLeft) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::UpRight) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::DownLeft) {
                total += 1;
            }
            if xmas_search(token, &word_search, Direction::DownRight) {
                total += 1;
            }
        }
    }        

    Ok(total)
}

fn xmas_search(token: &Token, word_search: &Vec<Vec<Token>>, direction: Direction) -> bool {

    if token.letter.ne(&'X') {
        return false;
    }

    let line = token.line;
    let position = token.position;

    let new_position = move_coord(&(line, position), &direction);    

    let Some(row) = word_search.get(new_position.0 as usize) else {
        return false;
    };

    let Some(dest_token) = row.get(new_position.1 as usize) else {
        return false;
    };

    if dest_token.letter.ne(&'M') {
        return false;
    }

    let new_position = move_coord(&(new_position.0, new_position.1), &direction);

    let Some(row) = word_search.get(new_position.0 as usize) else {
        return false;
    };

    let Some(dest_token) = row.get(new_position.1 as usize) else {
        return false;
    };

    if dest_token.letter.ne(&'A') {
        return false;
    }

    let new_position = move_coord(&(new_position.0, new_position.1), &direction);

    let Some(row) = word_search.get(new_position.0 as usize) else {
        return false;
    };

    let Some(dest_token) = row.get(new_position.1 as usize) else {
        return false;
    };

    if dest_token.letter.ne(&'S') {
        return false;
    }

    true
}


fn an_actual_x_mas_word_search(data: &str) -> Result<i32, Box<dyn Error>> {

    let mut word_search = Vec::new();

    let mut line_counter = 0;

    for line in data.lines() {
        let mut char_counter = 0;

        let mut row = Vec::new();
        for char in line.chars() {
            row.push(Token { letter: char, line: line_counter, position: char_counter});
            char_counter += 1;
        }
        word_search.push(row);
        line_counter += 1;
    }

    let mut total = 0;
    for row in word_search.iter() {
        for token in row.iter() {
            total += an_actual_x_mas_search(token, &word_search);
        }
    }

    Ok(total)
}

fn an_actual_x_mas_search(token: &Token, word_search: &Vec<Vec<Token>>) -> i32 {

    if token.letter.ne(&'M') {
        return 0;
    }

    let mut total = 0;

    //check for neighboring M.M pattern
    if check_char(token.line, token.position + 2, &'M', word_search) {
        // Check Above for Ss
        if check_char(token.line - 2, token.position, &'S', word_search) &&
            check_char(token.line - 2, token.position + 2, &'S', word_search) &&
            check_char(token.line - 1, token.position + 1, &'A', word_search) {
            total += 1;
        }

        // Check Below for Ss
        if check_char(token.line + 2, token.position, &'S', word_search) &&
            check_char(token.line + 2, token.position + 2, &'S', word_search) &&
            check_char(token.line + 1, token.position + 1, &'A', word_search) {
            total += 1;
        }

        return total;
    }

    //check for above M.M pattern
    if check_char(token.line - 2, token.position, &'M', word_search) {
        // Check for Left Ss
        if check_char(token.line, token.position - 2, &'S', word_search) &&
            check_char(token.line - 2, token.position - 2, &'S', word_search) &&
            check_char(token.line - 1, token.position - 1, &'A', word_search) {
            total += 1;
        }
        // Check for Right Ss
        if check_char(token.line, token.position + 2, &'S', word_search) &&
            check_char(token.line - 2, token.position + 2, &'S', word_search) &&
            check_char(token.line - 1, token.position + 1, &'A', word_search) {
            total += 1;
        }

        return total;
    }

    //check for below M.M pattern
    if check_char(token.line + 2, token.position, &'M', word_search) {
        // Check for Left Ss
        if check_char(token.line, token.position - 2, &'S', word_search) &&
            check_char(token.line + 2, token.position - 2, &'S', word_search) &&
            check_char(token.line + 1, token.position - 1, &'A', word_search) {
            total += 1;
        }
        // Check for Right Ss
        if check_char(token.line, token.position + 2, &'S', word_search) &&
            check_char(token.line + 2, token.position + 2, &'S', word_search) &&
            check_char(token.line + 1, token.position + 1, &'A', word_search) {
            total += 1;
        }

        return total;
    }

    total
}

fn check_char(line: i32, position: i32, letter: &char, word_search: &Vec<Vec<Token>>) -> bool {

    let Some(row) = word_search.get(line as usize) else {
        return false;
    };

    let Some(dest_token) = row.get(position as usize) else {
        return false;
    };
    
    dest_token.letter.eq(letter)
}

fn move_coord(coord: &(i32, i32), direction: &Direction) -> (i32, i32) {
    let mut line = coord.0;
    let mut position = coord.1;
    match direction {
        Direction::Forward => position += 1,
        Direction::Backwards => position -= 1,
        Direction::Up => line -= 1,
        Direction::Down => line += 1,
        Direction::UpLeft => { line -= 1; position -= 1; },
        Direction::DownLeft => { line += 1; position -= 1; },
        Direction::UpRight => { line -= 1; position += 1; },
        Direction::DownRight => { line += 1; position += 1; },
    }

    (line, position)
}

// this was terribly inefficient
fn xmas_word_search(data: &str) -> Result<i32, Box<dyn Error>> {

    let mut x_tokens = Vec::new();
    let mut m_tokens = Vec::new();
    let mut a_tokens = Vec::new();
    let mut s_tokens = Vec::new();

    let mut line_counter = 1;

    for line in data.lines() {
        let mut pos_counter = 1;

        for char in line.chars() {
            match char {
                'X' => x_tokens.push(Token { letter: 'X', line: line_counter, position: pos_counter }),
                'M' => m_tokens.push(Token { letter: 'M', line: line_counter, position: pos_counter }),
                'A' => a_tokens.push(Token { letter: 'A', line: line_counter, position: pos_counter }),
                'S' => s_tokens.push(Token { letter: 'S', line: line_counter, position: pos_counter }),
                _ => (),
            }

            pos_counter += 1;
        }

        line_counter += 1;
    }

    let mut total = 0;
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::Forward);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::Backwards);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::Up);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::Down);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::UpLeft);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::DownLeft);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::UpRight);
    total += token_search(&x_tokens, &m_tokens, &a_tokens, &s_tokens, &Direction::DownRight);

    Ok(total)
}

// this was terribly inefficient
fn token_search(x_tokens: &Vec<Token>, m_tokens: &Vec<Token>, a_tokens: &Vec<Token>, s_tokens: &Vec<Token>, direction: &Direction) -> i32 {

    let mut x_matches = Vec::new();

    for x in x_tokens {
        for m in m_tokens {
            if x.token_exists(m, direction) {
                x_matches.push(m);
            }
        }
    }

    let mut m_matches = Vec::new();

    for m in x_matches {
        for a in a_tokens {
            if m.token_exists(a, direction) {
                m_matches.push(a);
            }
        }
    }

    let mut a_matches = Vec::new();

    for a in m_matches {
        for s in s_tokens {
            if a.token_exists(s, direction) {
                a_matches.push(s);
            }
        }
    }

    a_matches.len() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_exists() {
        let x0 = Token { letter: 'X', line: 5, position: 10 };
        let x1 = Token { letter: 'X', line: 5, position: 10 };
        let x2 = Token { letter: 'X', line: 5, position: 11 };
        let x3 = Token { letter: 'X', line: 5, position: 9 };
        let x4 = Token { letter: 'X', line: 4, position: 10 };
        let x5 = Token { letter: 'X', line: 6, position: 10 };
        let x6 = Token { letter: 'X', line: 4, position: 9 };
        let x7 = Token { letter: 'X', line: 6, position: 9 };
        let x8 = Token { letter: 'X', line: 4, position: 11 };
        let x9 = Token { letter: 'X', line: 6, position: 11 };

        assert_eq!(x0.token_exists(&x1, &Direction::Forward), false);
        assert_eq!(x0.token_exists(&x2, &Direction::Forward), true);
        assert_eq!(x0.token_exists(&x3, &Direction::Backwards), true);
        assert_eq!(x0.token_exists(&x4, &Direction::Up), true);
        assert_eq!(x0.token_exists(&x5, &Direction::Down), true);
        assert_eq!(x0.token_exists(&x6, &Direction::UpLeft), true);
        assert_eq!(x0.token_exists(&x7, &Direction::DownLeft), true);
        assert_eq!(x0.token_exists(&x8, &Direction::UpRight), true);
        assert_eq!(x0.token_exists(&x9, &Direction::DownRight), true);
    }

    #[test]
    fn example() {
        let example = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = xmas_word_search_2(&example).expect("Failed to parse example.");
        let result2 = an_actual_x_mas_word_search(&example).expect("Failed to parse example.");

        assert_eq!(result, 18);
        assert_eq!(result2, 9);
    }
}