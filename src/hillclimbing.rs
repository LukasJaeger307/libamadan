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
 
pub struct Hillclimbing<S>{
    resource_type: PhantomData<S>,
    max_iterations: u64,
    max_failed_iterations: u64
}

impl<S> Hillclimbing<S>{
    pub fn new(max_iterations : u64, max_failed_iterations : u64) -> Hillclimbing<S>{
        Hillclimbing{resource_type: PhantomData,
            max_iterations : max_iterations,
            max_failed_iterations : max_failed_iterations,
        }
    }
}

impl<S> Metaheuristic<S> for Hillclimbing<S>{
    fn find(&self, rsg : &RandomSolutionGenerator<S>, fitness_function : &FitnessFunction<S>) -> S{
        let mut current : S;
        let mut tmp : S;
        let mut current_fitness : f64;
        let mut tmp_fitness : f64;
        let mut iterations : u64 = 0;
        let mut failed_iterations : u64 = 0;
        current = rsg.generate_random();
        current_fitness = fitness_function.get_fitness(&current);
        while (iterations < self.max_iterations) & (failed_iterations < self.max_failed_iterations){
            tmp = rsg.mutate(&current);
            tmp_fitness = fitness_function.get_fitness(&tmp);
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
