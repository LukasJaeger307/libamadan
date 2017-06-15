 
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

/// The Great Deluge Algorithm is a Hillclimbing-variant that accepts "worse" 
/// solutions than the current one as long as they are not worse than a rising
/// "water level".  The initial water level and the rising rate must be chosen
/// carefully in order to leave local maxima. The parameters
/// for the maximum number of iterations and failed iterations can be used to 
/// limit the algorithm's runtime.
pub struct GDA<'a, S : Clone + 'a>{
    resource_type : PhantomData<S>,
    max_iterations : u64,
    max_failed_iterations : u64,
    initial_water_level: f64,
    rising_rate : f64,
    random_solution_generator : &'a RandomSolutionGenerator<S>,
    fitness_function : &'a FitnessFunction<S>
}

/// The implementation of the constructor for GDA
impl <'a, S> GDA <'a, S> where S: Clone{
    /// Constructor for GDA. Requires the maximum number of
    /// overall iterations, the maximum number of failed iterations,
    /// the initial water level and the rising rate of the water.
    pub fn new(random_solution_generator : &'a RandomSolutionGenerator<S>, fitness_function : &'a FitnessFunction<S>, max_iterations : u64, max_failed_iterations : u64, initial_water_level : f64, rising_rate : f64) -> GDA<'a, S>{
        GDA{resource_type: PhantomData,
            max_iterations : max_iterations,
            max_failed_iterations : max_failed_iterations,
            initial_water_level: initial_water_level,
            rising_rate : rising_rate,
            random_solution_generator : random_solution_generator,
            fitness_function : fitness_function,
        }
    }
}

/// Implementation of the Metaheuristic trait
impl<'a, S> Metaheuristic<S> for GDA<'a, S> where S: Clone{
    ///Implementation of the find-method
    fn find(&self) -> S{
        let mut current : S;
        let mut tmp : S;
        let mut record : S;
        let mut tmp_fitness : f64;
        let mut record_fitness : f64;
        let mut iterations : u64 = 0;
        let mut failed_iterations : u64 = 0;
        let mut water_level : f64 = self.initial_water_level;
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
            if tmp_fitness > water_level{
                current = tmp;
                failed_iterations = 0;
                water_level += self.rising_rate;
            }
            else{
                failed_iterations +=1 ;
            }
            iterations += 1;
        }
        record
    }
}
