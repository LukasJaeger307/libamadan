/*
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */
 
use metaheuristic::Metaheuristic;
use rndsolutiongenerator::RandomSolutionGenerator;
use fitnessfunction::FitnessFunction;
use std::marker::PhantomData;
 
/// Hillclimbing is the most basic metaheuristic. It starts with a random solution,
/// mutates it slightly and if it achieves a better fitness after mutation than before,
/// it replaces the initial solution. This is re-iterated until a given maximum number
/// of iterations is reached or if a given maximum number of failed iterations is exceeded.
/// This metaheuristic has no way of avoiding local maxima. If it gets stuck in a local 
/// maximum, it has no chance of ever leaving it again.
pub struct Hillclimbing<'a, S : Clone + 'a>{
    resource_type: PhantomData<S>,
    max_iterations: u64,
    max_failed_iterations: u64,
    random_solution_generator : &'a RandomSolutionGenerator<S>,
    fitness_function : &'a FitnessFunction<S>
}

/// The implementation of the constructor for hillclimbing
impl<'a, S> Hillclimbing<'a, S> where S: Clone{
    /// Constructor for hillclimbing. Requires the maximum number of
    /// overall iterations and the maximum number of failed iterations
    /// as a parameter.
    pub fn new(random_solution_generator : &'a RandomSolutionGenerator<S>, fitness_function : &'a FitnessFunction<S>, max_iterations : u64, max_failed_iterations : u64) -> Hillclimbing<'a, S>{
        Hillclimbing{resource_type: PhantomData,
            max_iterations : max_iterations,
            max_failed_iterations : max_failed_iterations,
            random_solution_generator : random_solution_generator,
            fitness_function : fitness_function,
        }
    }
}

/// Implementation of the Metaheuristic trait
impl<'a, S> Metaheuristic<S> for Hillclimbing<'a, S> where S: Clone{
    ///Implementation of the find-method
    fn find(&self) -> S{
        let mut current : S;
        let mut tmp : S;
        let mut current_fitness : f64;
        let mut tmp_fitness : f64;
        let mut iterations : u64 = 0;
        let mut failed_iterations : u64 = 0;
        current = self.random_solution_generator.generate_random();
        current_fitness = self.fitness_function.get_fitness(&current);
        while (iterations < self.max_iterations) & (failed_iterations < self.max_failed_iterations){
            tmp = self.random_solution_generator.mutate(&current);
            tmp_fitness = self.fitness_function.get_fitness(&tmp);
            if tmp_fitness > current_fitness{
                current = tmp;
                current_fitness = tmp_fitness;
                failed_iterations = 0;
            }
            else{
                failed_iterations += 1;
            }
            iterations +=1;
        }
        current
    }
}
