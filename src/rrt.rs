 
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

/// Record-to-Record-Travel is a Hillclimbing-variant that accepts "worse" 
/// solutions than the current one as long as they are not worse than the 
/// record solution minus a given allowed fitness deviation. This deviation
/// must be selected in a way that allows to leave local optima. The parameters
/// for the maximum number of iterations and failed iterations can be used to 
/// limit the algorithm's runtime.
pub struct RRT<'a, S : Clone + 'a>{
    resource_type : PhantomData<S>,
    max_iterations : u64,
    max_failed_iterations : u64,
    max_deviation : f64,
    random_solution_generator : &'a RandomSolutionGenerator<S>,
    fitness_function : &'a FitnessFunction<S>
}

/// The implementation of the constructor for Record-to-Record-Travel
impl <'a, S> RRT <'a, S> where S: Clone{
    /// Constructor for Record-to-Record-Travel. Requires the maximum number of
    /// overall iterations, the maximum number of failed iterations and the maximum
    /// allowed deviation from the record fitness as a parameter.
    pub fn new(random_solution_generator : &'a RandomSolutionGenerator<S>, fitness_function : &'a FitnessFunction<S>, max_iterations : u64, max_failed_iterations : u64, max_deviation : f64) -> RRT<'a, S>{
        RRT{resource_type: PhantomData,
            max_iterations : max_iterations,
            max_failed_iterations : max_failed_iterations,
            max_deviation : max_deviation,
            random_solution_generator : random_solution_generator,
            fitness_function : fitness_function,
        }
    }
}

/// Implementation of the Metaheuristic trait
impl<'a, S> Metaheuristic<S> for RRT<'a, S> where S: Clone{
    ///Implementation of the find-method
    fn find(&self) -> S{
        let mut current : S;
        let mut tmp : S;
        let mut record : S;
        let mut tmp_fitness : f64; 
        let mut record_fitness : f64;
        let mut iterations : u64 = 0;
        let mut failed_iterations : u64 = 0;
        current = self.random_solution_generator.generate_random();
        record = current.clone();
        record_fitness = self.fitness_function.get_fitness(&current);
        while (iterations < self.max_iterations) & (failed_iterations < self.max_failed_iterations){
            tmp = self.random_solution_generator.mutate(&current);
            tmp_fitness = self.fitness_function.get_fitness(&tmp);
            if tmp_fitness > record_fitness {
                record_fitness = tmp_fitness;
                record = tmp.clone();
            }
            if tmp_fitness > record_fitness - self.max_deviation{
                current = tmp;
                failed_iterations = 0;
            }
            else{
                failed_iterations +=1 ;
            }
            
            iterations += 1;
        }
        record
    }
}
