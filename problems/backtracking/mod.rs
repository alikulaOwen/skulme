// Automatically generated category module

#[path = "all_combination_of_size_k/solution.rs"]
pub mod all_combination_of_size_k;

#[path = "graph_coloring/solution.rs"]
pub mod graph_coloring;

#[path = "hamiltonian_cycle/solution.rs"]
pub mod hamiltonian_cycle;

#[path = "knight_tour/solution.rs"]
pub mod knight_tour;

#[path = "n_queens/solution.rs"]
pub mod n_queens;

#[path = "parentheses_generator/solution.rs"]
pub mod parentheses_generator;

#[path = "permutations/solution.rs"]
pub mod permutations;

#[path = "rat_in_maze/solution.rs"]
pub mod rat_in_maze;

#[path = "subset_sum/solution.rs"]
pub mod subset_sum;

#[path = "sudoku/solution.rs"]
pub mod sudoku;


pub use all_combination_of_size_k::generate_all_combinations;
pub use graph_coloring::generate_colorings;
pub use hamiltonian_cycle::find_hamiltonian_cycle;
pub use knight_tour::find_knight_tour;
pub use n_queens::n_queens_solver;
pub use parentheses_generator::generate_parentheses;
pub use permutations::permute;
pub use rat_in_maze::find_path_in_maze;
pub use subset_sum::has_subset_with_sum;
pub use sudoku::sudoku_solver;
