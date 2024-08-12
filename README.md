# Boggle Solver in Rust

## Overview

This project implements a Boggle solver using Rust. The solver searches for words from a given dictionary within a Boggle board, utilizing depth-first search (DFS) to find valid paths. The core functionality revolves around navigating a grid, checking for word matches, and recording paths for words found.

## Key Features

- **Depth-First Search (DFS):** Implements a DFS algorithm to explore all possible paths on the Boggle board for word search.
- **Dynamic Grid Handling:** Supports boards of various sizes by dynamically managing the grid based on input.
- **Word Lookup Optimization:** Uses a hash map to efficiently look up words based on their starting character, reducing unnecessary searches.
- **Path Tracking:** Records the path of each found word, ensuring accuracy in the path representation.

## Code Explanation

### `dfs` Function

The `dfs` function performs a recursive depth-first search to find a word on the Boggle board. It explores all possible directions from the current position, ensuring that no cell is visited more than once in the same word path.

**Parameters:**
- `grid`: The Boggle board represented as a 2D vector of bytes.
- `word`: The word to be searched.
- `path`: A mutable vector tracking the current path.
- `(x, y)`: The current position on the grid.

### `boggle` Function

The `boggle` function initializes the Boggle board and iterates over each cell to start a DFS for each word in the dictionary that begins with the character at that cell. It uses a hash map to optimize lookups and ensure that words are only searched when necessary.

**Parameters:**
- `board`: The Boggle board represented as a slice of strings.
- `words`: A vector of words to search for in the board.

**Returns:**
- A hash map where keys are words found on the board, and values are vectors of cell coordinates representing the path of the word.

## How to Use

1. **Set Up the Board:**
   Provide the Boggle board as a vector of strings, where each string represents a row of the board.

2. **Provide a Word List:**
   Supply a vector of words you want to search for within the board.

3. **Run the Solver:**
   Call the `boggle` function with the board and word list to get a hash map of found words and their paths.

   ```rust
   fn main() {
       let board = vec![
           "abcd".to_string(),
           "efgh".to_string(),
           "ijkl".to_string(),
           "mnop".to_string(),
       ];
       let words = vec!["abcd".to_string(), "efgh".to_string()];
       let result = boggle(&board.iter().map(|s| &s[..]).collect::<Vec<&str>>(), &words);
       println!("{:?}", result);
   }
   ```
## What You Can Learn
- **Recursive Algorithm:** Understand how recursion can be used to explore possible solutions in a grid based problem.
- **Optimizations:** Learn about efficient ways to handle and search large sets of data using hash maps.
- **Grid Navigation:** Gain insights into navigating multi-dimensional arrays and managing pathfinding challenges
- **Rust Programming:** Experience Rust's strong type system and ownership model in practical applications.

## Running Tests
The project includes tests to ensure the correctness of the implementation. You can run the tests using the following command:

```bash
cargo test
```
