use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::Debug;

/// The goal is to showcase how Genetic algorithms generically work
/// See: https://en.wikipedia.org/wiki/Genetic_algorithm for concepts

/// This is the definition of a Chromosome for a genetic algorithm
/// We can picture this as "one contending solution to our problem"
/// It is generic over:
/// * Eval, which could be a float, or any other totally ordered type, so that we can rank solutions to our problem
/// * Rng: a random number generator (could be thread rng, etc.)
pub trait Chromosome<Rng: rand::RngExt, Eval> {
    /// Mutates this Chromosome, changing its genes
    fn mutate(&mut self, rng: &mut Rng);

    /// Mixes this chromosome with another one
    fn crossover(&self, other: &Self, rng: &mut Rng) -> Self;

    /// How well this chromosome fits the problem we're trying to solve
    /// **The smaller the better it fits** (we could use abs(... - expected_value) for instance
    fn fitness(&self) -> Eval;
}

pub trait SelectionStrategy<Rng: rand::RngExt> {
    fn new(rng: Rng) -> Self;

    /// Selects a portion of the population for reproduction
    /// Could be totally random ones or the ones that fit best, etc.
    /// This assumes the population is sorted by how it fits the solution (the first the better)
    fn select<'a, Eval: Into<f64>, C: Chromosome<Rng, Eval>>(
        &mut self,
        population: &'a [C],
    ) -> (&'a C, &'a C);
}

/// A roulette wheel selection strategy
/// https://en.wikipedia.org/wiki/Fitness_proportionate_selection
#[allow(dead_code)]
pub struct RouletteWheel<Rng: rand::RngExt> {
    rng: Rng,
}
impl<Rng: rand::RngExt> SelectionStrategy<Rng> for RouletteWheel<Rng> {
    fn new(rng: Rng) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn select<'a, Eval: Into<f64>, C: Chromosome<Rng, Eval>>(
        &mut self,
        population: &'a [C],
    ) -> (&'a C, &'a C) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (select)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement select");
}
}

#[allow(dead_code)]
pub struct Tournament<const K: usize, Rng: rand::RngExt> {
    rng: Rng,
}
impl<const K: usize, Rng: rand::RngExt> SelectionStrategy<Rng> for Tournament<K, Rng> {
    fn new(rng: Rng) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn select<'a, Eval, C: Chromosome<Rng, Eval>>(
        &mut self,
        population: &'a [C],
    ) -> (&'a C, &'a C) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (select)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement select");
}
}

type Comparator<T> = Box<dyn FnMut(&T, &T) -> Ordering>;
pub struct GeneticAlgorithm<
    Rng: rand::RngExt,
    Eval: PartialOrd,
    C: Chromosome<Rng, Eval>,
    Selection: SelectionStrategy<Rng>,
> {
    rng: Rng, // will be used to draw random numbers for initial population, mutations and crossovers
    population: Vec<C>, // the set of random solutions (chromosomes)
    threshold: Eval, // Any chromosome fitting over this threshold is considered a valid solution
    max_generations: usize, // So that we don't loop infinitely
    mutation_chance: f64, // what's the probability a chromosome will mutate
    crossover_chance: f64, // what's the probability two chromosomes will cross-over and give birth to a new chromosome
    compare: Comparator<Eval>,
    selection: Selection, // how we will select parent chromosomes for crossing over, see `SelectionStrategy`
}

pub struct GenericAlgorithmParams {
    max_generations: usize,
    mutation_chance: f64,
    crossover_chance: f64,
}

impl<
        Rng: rand::RngExt,
        Eval: Into<f64> + PartialOrd + Debug,
        C: Chromosome<Rng, Eval> + Clone + Debug,
        Selection: SelectionStrategy<Rng>,
    > GeneticAlgorithm<Rng, Eval, C, Selection>
{
    pub fn init(
        rng: Rng,
        population: Vec<C>,
        threshold: Eval,
        params: GenericAlgorithmParams,
        compare: Comparator<Eval>,
        selection: Selection,
    ) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (init)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement init");
}

    pub fn solve(&mut self) -> Option<C> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (solve)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement solve");
}
}


#[cfg(test)]
mod tests {
    use crate::general::genetic::{
        Chromosome, GenericAlgorithmParams, GeneticAlgorithm, RouletteWheel, SelectionStrategy,
        Tournament,
    };
    use rand::rngs::ThreadRng;
    use rand::{rng, RngExt};
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::ops::RangeInclusive;

    #[test]
    #[ignore] // Too long and not deterministic enough to be part of CI, more of an example than a test
    fn find_secret() {
        let chars = 'a'..='z';
        let secret = "thisistopsecret".to_owned();
        // Note: we'll pick genes (a, b, c) in the range -10, 10
        #[derive(Clone)]
        struct TestString {
            chars: RangeInclusive<char>,
            secret: String,
            genes: Vec<char>,
        }
        impl TestString {
            fn new(rng: &mut ThreadRng, secret: String, chars: RangeInclusive<char>) -> Self {
                let current = (0..secret.len())
                    .map(|_| rng.random_range(chars.clone()))
                    .collect::<Vec<_>>();

                Self {
                    chars,
                    secret,
                    genes: current,
                }
            }
        }
        impl Debug for TestString {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.genes.iter().collect::<String>())
            }
        }
        impl Chromosome<ThreadRng, i32> for TestString {
            fn mutate(&mut self, rng: &mut ThreadRng) {
                // let's assume mutations happen completely randomly, one "gene" at a time (i.e. one char at a time)
                let gene_idx = rng.random_range(0..self.secret.len());
                let new_char = rng.random_range(self.chars.clone());
                self.genes[gene_idx] = new_char;
            }

            fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
                // Let's not assume anything here, simply mixing random genes from both parents
                let genes = (0..self.secret.len())
                    .map(|idx| {
                        if rng.random_bool(0.5) {
                            // pick gene from self
                            self.genes[idx]
                        } else {
                            // pick gene from other parent
                            other.genes[idx]
                        }
                    })
                    .collect();
                Self {
                    chars: self.chars.clone(),
                    secret: self.secret.clone(),
                    genes,
                }
            }

            fn fitness(&self) -> i32 {
                // We are just counting how many chars are distinct from secret
                self.genes
                    .iter()
                    .zip(self.secret.chars())
                    .filter(|(char, expected)| expected != *char)
                    .count() as i32
            }
        }
        let mut rng = rng();
        let pop_count = 1_000;
        let mut population = Vec::with_capacity(pop_count);
        for _ in 0..pop_count {
            population.push(TestString::new(&mut rng, secret.clone(), chars.clone()));
        }
        let selection: Tournament<100, ThreadRng> = Tournament::new(rng.clone());
        let params = GenericAlgorithmParams {
            max_generations: 100,
            mutation_chance: 0.2,
            crossover_chance: 0.4,
        };
        let mut solver =
            GeneticAlgorithm::init(rng, population, 0, params, Box::new(i32::cmp), selection);
        let res = solver.solve();
        assert!(res.is_some());
        assert_eq!(res.unwrap().genes, secret.chars().collect::<Vec<_>>())
    }

    #[test]
    #[ignore] // Too long and not deterministic enough to be part of CI, more of an example than a test
    fn solve_mastermind() {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        enum ColoredPeg {
            Red,
            Yellow,
            Green,
            Blue,
            White,
            Black,
        }
        struct GuessAnswer {
            right_pos: i32, // right color at the right pos
            wrong_pos: i32, // right color, but at wrong pos
        }
        #[derive(Clone, Debug)]
        struct CodeMaker {
            // the player coming up with a secret code
            code: [ColoredPeg; 4],
            count_by_color: HashMap<ColoredPeg, usize>,
        }
        impl CodeMaker {
            fn new(code: [ColoredPeg; 4]) -> Self {
                let mut count_by_color = HashMap::with_capacity(4);
                for peg in &code {
                    *count_by_color.entry(*peg).or_insert(0) += 1;
                }
                Self {
                    code,
                    count_by_color,
                }
            }
            fn eval(&self, guess: &[ColoredPeg; 4]) -> GuessAnswer {
                let mut right_pos = 0;
                let mut wrong_pos = 0;
                let mut idx_by_colors = self.count_by_color.clone();
                for (idx, color) in guess.iter().enumerate() {
                    if self.code[idx] == *color {
                        right_pos += 1;
                        let count = idx_by_colors.get_mut(color).unwrap();
                        *count -= 1; // don't reuse to say "right color but wrong pos"
                        if *count == 0 {
                            idx_by_colors.remove(color);
                        }
                    }
                }
                for (idx, color) in guess.iter().enumerate() {
                    if self.code[idx] != *color {
                        // try to use another color
                        if let Some(count) = idx_by_colors.get_mut(color) {
                            *count -= 1;
                            if *count == 0 {
                                idx_by_colors.remove(color);
                            }
                            wrong_pos += 1;
                        }
                    }
                }
                GuessAnswer {
                    right_pos,
                    wrong_pos,
                }
            }
        }

        #[derive(Clone)]
        struct CodeBreaker {
            maker: CodeMaker, // so that we can ask the code maker if our guess is good or not
            guess: [ColoredPeg; 4],
        }
        impl Debug for CodeBreaker {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(format!("{:?}", self.guess).as_str())
            }
        }
        fn random_color(rng: &mut ThreadRng) -> ColoredPeg {
            match rng.random_range(0..=5) {
                0 => ColoredPeg::Red,
                1 => ColoredPeg::Yellow,
                2 => ColoredPeg::Green,
                3 => ColoredPeg::Blue,
                4 => ColoredPeg::White,
                _ => ColoredPeg::Black,
            }
        }
        fn random_guess(rng: &mut ThreadRng) -> [ColoredPeg; 4] {
            std::array::from_fn(|_| random_color(rng))
        }
        impl Chromosome<ThreadRng, i32> for CodeBreaker {
            fn mutate(&mut self, rng: &mut ThreadRng) {
                // change one random color
                let idx = rng.random_range(0..4);
                self.guess[idx] = random_color(rng);
            }

            fn crossover(&self, other: &Self, rng: &mut ThreadRng) -> Self {
                Self {
                    maker: self.maker.clone(),
                    guess: std::array::from_fn(|i| {
                        if rng.random::<f64>() < 0.5 {
                            self.guess[i]
                        } else {
                            other.guess[i]
                        }
                    }),
                }
            }

            fn fitness(&self) -> i32 {
                // Ask the code maker for the result
                let answer = self.maker.eval(&self.guess);
                // Remember: we need to have fitness return 0 if the guess is good, and the higher number we return, the further we are from a proper solution
                let mut res = 32; // worst case scenario, everything is wrong
                res -= answer.right_pos * 8; // count 8 points for the right item at the right spot
                res -= answer.wrong_pos; // count 1 point for having a right color
                res
            }
        }
        let code = [
            ColoredPeg::Red,
            ColoredPeg::Red,
            ColoredPeg::White,
            ColoredPeg::Blue,
        ];
        let maker = CodeMaker::new(code);
        let population_count = 10;
        let params = GenericAlgorithmParams {
            max_generations: 100,
            mutation_chance: 0.5,
            crossover_chance: 0.3,
        };
        let mut rng = rng();
        let mut initial_pop = Vec::with_capacity(population_count);
        for _ in 0..population_count {
            initial_pop.push(CodeBreaker {
                maker: maker.clone(),
                guess: random_guess(&mut rng),
            });
        }
        let selection = RouletteWheel { rng: rng.clone() };
        let mut solver =
            GeneticAlgorithm::init(rng, initial_pop, 0, params, Box::new(i32::cmp), selection);
        let res = solver.solve();
        assert!(res.is_some());
        assert_eq!(code, res.unwrap().guess);
    }
}
