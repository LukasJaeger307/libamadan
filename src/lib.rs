extern crate rand;

mod fitnessfunction;
mod rndsolutiongenerator;
mod metaheuristic;
mod hillclimbing;
mod rrt;

#[cfg(test)]
mod tests {
    extern crate rand;
    use fitnessfunction::FitnessFunction;
    use rndsolutiongenerator::RandomSolutionGenerator;
    use metaheuristic::Metaheuristic;
    use hillclimbing::Hillclimbing;
    use rand::Rng;
    use rrt::RRT;
    
    struct SquareFitnessFunction;
    
     impl FitnessFunction<f64> for SquareFitnessFunction{
        fn get_fitness(&self, solution: &f64) -> f64{
            let mut result : f64 = *solution;
            result -= 1.0;
            result *= result;
            result = - result;
            result + 1.0
        }
    }
    
    #[test]
    fn test_fitness_function(){
        let fitness_function = SquareFitnessFunction{};
        assert!(fitness_function.get_fitness(&1.0) == 1.0);
        assert!(fitness_function.get_fitness(&0.0) == 0.0);
        assert!(fitness_function.get_fitness(&2.0) == 0.0);
    }
    
    struct SquareRandomSolutionGenerator;
    
    impl RandomSolutionGenerator<f64> for SquareRandomSolutionGenerator{
        fn generate_random(&self) -> f64{
            let mut rng = rand::thread_rng();
            let solution: f64 = rng.gen_range(-40.0, 40.0);
            solution
        }
        
        fn mutate(&self, current : &f64) -> f64{
            let result : f64;
            result = *current;
            let mut rng = rand::thread_rng();
            result + rng.gen_range(-1.0, 1.0)
        }
    }
    
    #[test]
    fn test_random_solution_generator() {
        let rsg = SquareRandomSolutionGenerator{};
        assert!(rsg.generate_random() != 0.0);
        let current : f64 = 0.0;
        assert!(rsg.mutate(&current) != 0.0);
    }
    
    #[test]
    fn test_hillclimbing(){
        let rsg = SquareRandomSolutionGenerator{};
        let fitness_function = SquareFitnessFunction{};
        let hillclimber : Hillclimbing<f64> = Hillclimbing::new(10000, 100);
        let optimum = hillclimber.find(&rsg, &fitness_function);
        println!("Optimum: {}", optimum);
        assert!(optimum > 0.0);
        assert!(optimum < 2.0);
    }
    
    #[test]
    fn test_rrt(){
        let rsg = SquareRandomSolutionGenerator{};
        let fitness_function = SquareFitnessFunction{};
        let rrt : RRT<f64> = RRT::new(10000, 100, 0.5);
        let optimum = rrt.find(&rsg, &fitness_function);
        println!("Optimum: {}", optimum);
        assert!(optimum > 0.0);
        assert!(optimum < 2.0);
    }
    
    #[test]
    fn it_works() {
    }
    
    
}
