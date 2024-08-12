use std::collections::HashMap;

fn dfs(grid: &Vec<Vec<u8>>, word: String, path: &mut Vec<(u8, u8)>, (x, y): (u8, u8)) {
    path.push((x, y));
    if word.is_empty() {
        return;
    }
    let elem = word.as_bytes()[0];
    if x > 0 {
        if y > 0
            && elem == grid[(x - 1) as usize][(y - 1) as usize]
            && !path.contains(&(x - 1, y - 1))
        {
            dfs(grid, word.chars().skip(1).collect(), path, (x - 1, y - 1));
            if !path.is_empty() {
                return;
            }
        }
        if y < (grid.len() as u8) - 1
            && elem == grid[(x - 1) as usize][(y + 1) as usize]
            && !path.contains(&(x - 1, y + 1))
        {
            dfs(grid, word.chars().skip(1).collect(), path, (x - 1, y + 1));
            if !path.is_empty() {
                return;
            }
        }
        if elem == grid[(x - 1) as usize][y as usize] && !path.contains(&(x - 1, y)) {
            dfs(grid, word.chars().skip(1).collect(), path, (x - 1, y));
            if !path.is_empty() {
                return;
            }
        }
    }
    if x < (grid.len() as u8) - 1 {
        if y > 0
            && elem == grid[(x + 1) as usize][(y - 1) as usize]
            && !path.contains(&(x + 1, y - 1))
        {
            dfs(grid, word.chars().skip(1).collect(), path, (x + 1, y - 1));
            if !path.is_empty() {
                return;
            }
        }
        if y < (grid.len() as u8) - 1
            && elem == grid[(x + 1) as usize][(y + 1) as usize]
            && !path.contains(&(x + 1, y + 1))
        {
            dfs(grid, word.chars().skip(1).collect(), path, (x + 1, y + 1));
            if !path.is_empty() {
                return;
            }
        }
        if elem == grid[(x + 1) as usize][y as usize] && !path.contains(&(x + 1, y)) {
            dfs(grid, word.chars().skip(1).collect(), path, (x + 1, y));
            if !path.is_empty() {
                return;
            }
        }
    }
    if y > 0 && grid[x as usize][(y - 1) as usize] == elem && !path.contains(&(x, y - 1)) {
        dfs(grid, word.chars().skip(1).collect(), path, (x, y - 1));
        if !path.is_empty() {
            return;
        }
    }
    if y < (grid.len() as u8) - 1
        && grid[x as usize][(y + 1) as usize] == elem
        && !path.contains(&(x, y + 1))
    {
        dfs(grid, word.chars().skip(1).collect(), path, (x, y + 1));
        if !path.is_empty() {
            return;
        }
    }
    path.pop();
}

fn boggle(board: &[&str], words: &Vec<String>) -> HashMap<String, Vec<(u8, u8)>> {
    let mut found: HashMap<String, Vec<(u8, u8)>> = HashMap::new();
    let grid: Vec<Vec<u8>> = board.iter().fold(
        Vec::with_capacity(board.len() * board.len()),
        |mut gr, row| {
            gr.push(Vec::from(row.as_bytes()));
            gr
        },
    );
    let starting_char_dct: HashMap<u8, Vec<&String>> =
        words.iter().fold(HashMap::new(), |mut map, word| {
            if !word.is_empty() && word.len() <= grid.len() * grid.len() {
                map.entry(word.as_bytes()[0])
                    .or_insert_with(Vec::new)
                    .push(word);
            }
            map
        });
    for row in 0..grid.len() {
        for col in 0..grid.len() {
            if let Some(words_elem) = starting_char_dct.get(&grid[row][col]) {
                for &word in words_elem.iter() {
                    let mut path: Vec<(u8, u8)> = Vec::with_capacity(word.len());
                    dfs(
                        &grid,
                        word.chars().skip(1).collect(),
                        &mut path,
                        (row as u8, col as u8),
                    );
                    if !path.is_empty() && path.len() == word.len() {
                        found.insert(word.to_string(), path.clone());
                    }
                }
            }
        }
    }
    found
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
