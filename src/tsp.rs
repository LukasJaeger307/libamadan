/*
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */
 
//use metaheuristic::Metaheuristic;
//use rndsolutiongenerator::RandomSolutionGenerator;
use fitnessfunction::FitnessFunction;

use std::f64;

pub struct TSP{
    num_nodes : u32,
    list_nodes : Vec<u32>,
}

impl TSP{

    pub fn new (num_nodes : u32) -> TSP{
        let mut list_nodes : Vec<u32> = vec![];
        for x in 0..num_nodes{
            list_nodes.push(x);
        }
        TSP{
            num_nodes : num_nodes,
            list_nodes : list_nodes,
        }
    }
    
    pub fn get_num_nodes(&self) -> u32{
        self.num_nodes
    }
    
    pub fn get_node(&self, position : u32) -> u32{
        self.list_nodes[position as usize]
    }
    
    pub fn set_node (&mut self, position : u32, node: u32){
        if !((node >= self.num_nodes) | (position >= self.num_nodes)){
            self.list_nodes[position as usize] = node
        }
    }
}

impl Clone for TSP{
    fn clone(&self) -> Self{
        let mut list_nodes : Vec<u32> = vec![];
        for x in 0..self.num_nodes{
            list_nodes.push(self.list_nodes[x as usize]);
        }
        TSP{
            num_nodes : self.num_nodes,
            list_nodes : list_nodes,
        }
    }
}


pub struct TSPFitnessFunction{
    num_nodes : u32,
    matrix : Vec<Vec<f64>>,
}

impl TSPFitnessFunction{
    pub fn new(num_nodes : u32) -> TSPFitnessFunction{
        let mut matrix : Vec<Vec<f64>> = vec![];
        for _ in 0..num_nodes{
            let mut matrix_element : Vec<f64> = vec![];
            for _ in 0..num_nodes{
                matrix_element.push(0.0);
            }
            matrix.push(matrix_element);
        }
        TSPFitnessFunction{
            num_nodes : num_nodes,
            matrix : matrix,
        }
    }
    
    pub fn get_num_nodes(&self) -> u32{
        self.num_nodes
    }

    pub fn get_distance (&self, node1 : u32, node2 : u32) -> f64{
        if (node1 >= self.num_nodes) | (node2 >= self.num_nodes){
            f64::INFINITY
        } else{
            self.matrix[node1 as usize][node2 as usize]
        }
    }
    
    pub fn set_distance(&mut self, node1: u32, node2 : u32, distance : f64){
        if !((node1 >= self.num_nodes) | (node2 >= self.num_nodes) | (node1 == node2)){
            self.matrix[node1 as usize][node2 as usize] = distance;
            self.matrix[node2 as usize][node1 as usize] = distance;
        }
    }
}
    
impl FitnessFunction<TSP> for TSPFitnessFunction {
    fn get_fitness(&self, solution: &TSP) -> f64{
        if self.num_nodes == solution.num_nodes {
            let mut fitness : f64 = 0.0;
            for x in 0..(solution.get_num_nodes()) {
                let node1 : u32 = solution.get_node(x);
                let node2 : u32 = solution.get_node((x + 1) % solution.get_num_nodes());
                fitness += self.get_distance(node1, node2);
            }
            fitness
        } else {
            0.0
        }
    }
}

impl Clone for TSPFitnessFunction {
    fn clone(&self) -> Self{
        let mut matrix : Vec<Vec<f64>> = vec![];
        for x in 0..self.num_nodes {
            let mut matrix_element : Vec<f64> = vec![];
            for y in 0..self.num_nodes {
                matrix_element.push(self.matrix[x as usize][y as usize]);
            }
            matrix.push(matrix_element);
        }
        TSPFitnessFunction{
            num_nodes : self.num_nodes,
            matrix : matrix,
        }
    }
}


/*struct TSPRandomSolutionGenerator;
    
impl RandomSolutionGenerator<TSP> for TSPRandomSolutionGenerator{
    fn generate_random(&self) -> TSP{
        
    }
        
    fn mutate(&self, current : &TSP) -> TSP{
        
    }
}

struct TSPRecombinationGenerator;

impl RecombinationGenerator<TSP> for TSPRecombinationGenerator{
    fn recombine(&self, i1 : &TSP, i2 : &TSP) -> TSP{

    }
}*/
