 
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
}

/// The implementation of the constructor for GDA
impl <'a,S> GeneticAlgorithm <'a, S> where S: Clone{
    pub fn new(max_iterations : u64, recombination_generator : &RecombinationGenerator<S>) -> GeneticAlgorithm<S>{
        GeneticAlgorithm{resource_type: PhantomData,
            max_iterations : max_iterations,
            recombination_generator : recombination_generator,
        }
    }
}

/// Implementation of the Metaheuristic trait
impl<'a, S> Metaheuristic<S> for GeneticAlgorithm<'a,S> where S: Clone{
    ///Implementation of the find-method
    fn find(&self, rsg : &RandomSolutionGenerator<S>, fitness_function : &FitnessFunction<S>)-> S{
        let mut current : S;
        current = rsg.generate_random();
        current
    }
}
