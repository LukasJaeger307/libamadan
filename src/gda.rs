 
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
pub struct GDA<S : Clone>{
    resource_type : PhantomData<S>,
    max_iterations : u64,
    max_failed_iterations : u64,
    initial_water_level: f64,
    rising_rate : f64,
}

/// The implementation of the constructor for GDA
impl <S> GDA <S> where S: Clone{
    /// Constructor for GDA. Requires the maximum number of
    /// overall iterations, the maximum number of failed iterations,
    /// the initial water level and the rising rate of the water.
    pub fn new(max_iterations : u64, max_failed_iterations : u64, initial_water_level : f64, rising_rate : f64) -> GDA<S>{
        GDA{resource_type: PhantomData,
            max_iterations : max_iterations,
            max_failed_iterations : max_failed_iterations,
            initial_water_level: initial_water_level,
            rising_rate : rising_rate,
        }
    }
}

/// Implementation of the Metaheuristic trait
impl<S> Metaheuristic<S> for GDA<S> where S: Clone{
    ///Implementation of the find-method
    fn find(&self, rsg : &RandomSolutionGenerator<S>, fitness_function : &FitnessFunction<S>) -> S{
        let mut current : S;
        let mut tmp : S;
        let mut record : S;
        let mut current_fitness : f64;
        let mut tmp_fitness : f64 = 0.0;
        let mut record_fitness : f64 = 0.0;
        let mut iterations : u64 = 0;
        let mut failed_iterations : u64 = 0;
        let mut water_level : f64 = self.initial_water_level;
        current = rsg.generate_random();
        current_fitness = fitness_function.get_fitness(&current);
        record = current.clone();
        record_fitness = current_fitness;
        while (iterations < self.max_iterations) & (failed_iterations < self.max_failed_iterations){
            tmp = rsg.mutate(&current);
            tmp_fitness = fitness_function.get_fitness(&tmp);
            if tmp_fitness > record_fitness {
                record_fitness = tmp_fitness;
                record = tmp.clone();
            }
            if tmp_fitness > water_level{
                current = tmp;
                current_fitness = tmp_fitness;
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
