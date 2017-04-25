/*
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */

 /// Trait for structs that generate random solutions
 /// and mutate them randomly.
 pub trait RandomSolutionGenerator<S : Clone>{
    /// Generates a random solution
    fn generate_random(&self) -> S;
    
    /// Mutates a given solution randomly
    fn mutate(&self, current : &S) -> S;
 }
