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
}

impl<S> Hillclimbing<S>{
    pub fn new() -> Hillclimbing<S>{
        Hillclimbing{resource_type: PhantomData}
    }
}

impl<S> Metaheuristic<S> for Hillclimbing<S>{
    fn find(&self, rsg : &RandomSolutionGenerator<S>, fitness_function : &FitnessFunction<S>) -> S{
        let mut current : S;
        let mut tmp : S;
        let mut current_fitness : f64;
        let mut tmp_fitness : f64;
        current = rsg.generate_random();
        current_fitness = fitness_function.get_fitness(&current);
        for x in 0..1000000 {
            tmp = rsg.mutate(&current);
            tmp_fitness = fitness_function.get_fitness(&tmp);
            if tmp_fitness > current_fitness{
                current = tmp;
                current_fitness = tmp_fitness;
            }
        }
        current
    }
}
