/*
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */
 
//use metaheuristic::Metaheuristic;
//use rndsolutiongenerator::RandomSolutionGenerator;
//use fitnessfunction::FitnessFunction;

use std::f64;

pub struct TSP{
    num_nodes : u32,
    list_nodes : Vec<u32>,
    matrix : Vec<Vec<f64>>,
}

impl TSP{

    pub fn new (num_nodes : u32) -> TSP{
        let mut list_nodes : Vec<u32> = vec![];
        let mut matrix : Vec<Vec<f64>> = vec![];
        for x in 0..num_nodes{
            list_nodes.push(x);
            let mut matrix_element : Vec<f64> = vec![];
            for _ in 0..num_nodes{
                matrix_element.push(0.0);
            }
            matrix.push(matrix_element);
        }
        TSP{
            num_nodes : num_nodes,
            list_nodes : list_nodes,
            matrix : matrix,
        }
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
    
    pub fn get_node(&self, position : u32) -> u32{
        self.list_nodes[position as usize]
    }
    
    pub fn set_node (&mut self, position : u32, node: u32){
        if !((node >= self.num_nodes) | (position >= self.num_nodes)){
            self.list_nodes[position as usize] = node
        }
    }
}
