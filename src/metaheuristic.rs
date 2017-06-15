/*
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */
 
use rndsolutiongenerator::RandomSolutionGenerator;
use fitnessfunction::FitnessFunction;
 
/// The Metaheuristic trait is used to generalize the interface of
/// the different metaheuristics.
pub trait Metaheuristic<S : Clone>{
    /// The actual metaheuristic optimization function. It finds a
    /// solution close to the optimimum, using a random solution generator 
    /// and a function to evaluate a solution's fitness.
    fn find(&self) -> S;
}
