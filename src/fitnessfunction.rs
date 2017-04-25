/*
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */
 
/// This trait is used for all structs that evaluate the fitness of a 
/// solution. Fitness functions' implementations are problem-dependant.
pub trait FitnessFunction<S : Clone>{
    /// Returns the fitness of a given solution. The fitness is
    /// returned as a 64-bit float and better solutions must achieve
    /// a higher fitness. If you optimize for a minimum, design your
    /// fitness function accordingly.
    fn get_fitness(&self, solution:&S) -> f64;
}
