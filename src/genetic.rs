 
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
use recombinationgenerator::RecombinationGenerator;
use std::marker::PhantomData;

pub struct GeneticAlgorithm<'a, S : Clone + 'a>{
    resource_type : PhantomData<S>,
    max_iterations : u64,
    recombination_generator : &'a RecombinationGenerator<S>,
    random_solution_generator : &'a RandomSolutionGenerator<S>,
    fitness_function : &'a FitnessFunction<S>
}

/// The implementation of the constructor for GDA
impl <'a,S> GeneticAlgorithm <'a, S> where S: Clone{
     pub fn new(random_solution_generator : &'a RandomSolutionGenerator<S>, fitness_function : &'a FitnessFunction<S>, recombination_generator : &'a RecombinationGenerator<S>,max_iterations : u64) -> GeneticAlgorithm<'a, S>{
        GeneticAlgorithm{resource_type: PhantomData,
            max_iterations : max_iterations,
            recombination_generator : recombination_generator,
            random_solution_generator : random_solution_generator,
            fitness_function : fitness_function,
        }
    }
}

/// Implementation of the Metaheuristic trait
impl<'a, S> Metaheuristic<S> for GeneticAlgorithm<'a,S> where S: Clone{
    ///Implementation of the find-method
    fn find(&self)-> S{
        let mut current : S;
        current = self.random_solution_generator.generate_random();
        current
    }
}
