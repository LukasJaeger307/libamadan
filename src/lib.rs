extern crate rand;

pub mod fitnessfunction;
pub mod rndsolutiongenerator;
pub mod metaheuristic;
pub mod hillclimbing;
pub mod rrt;
pub mod gda;
pub mod recombinationgenerator;
//pub mod genetic;
pub mod tsp;

#[cfg(test)]
mod tests {
    extern crate rand;
    use fitnessfunction::FitnessFunction;
    use rndsolutiongenerator::RandomSolutionGenerator;
    use metaheuristic::Metaheuristic;
    use hillclimbing::Hillclimbing;
    use recombinationgenerator::RecombinationGenerator;
    use rand::Rng;
    use rrt::RRT;
    use gda::GDA;
    //use genetic::GeneticAlgorithm;
    use tsp::TSP;
    use tsp::TSPFitnessFunction;
    
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

    struct FloatRecombinationGenerator;

    impl RecombinationGenerator<f64> for FloatRecombinationGenerator{
        fn recombine(&self, i1 : &f64, i2 : &f64) -> f64{
            let max : f64;
            let min : f64;
            if *i1 > *i2{
                max = *i1;
                min = *i2;
            }
            else{
                max = *i2;
                min = *i1;
            }
            let diff : f64 = (max - min) / 2.0;
            max - diff
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
        let hillclimber : Hillclimbing<f64> = Hillclimbing::new(&rsg, &fitness_function,10000, 100);
        let optimum = hillclimber.find();
        println!("Optimum: {}", optimum);
        assert!(optimum > 0.0);
        assert!(optimum < 2.0);
    }
    
    #[test]
    fn test_rrt(){
        let rsg = SquareRandomSolutionGenerator{};
        let fitness_function = SquareFitnessFunction{};
        let rrt : RRT<f64> = RRT::new(&rsg, &fitness_function, 10000, 100, 0.5);
        let optimum = rrt.find();
        println!("Optimum: {}", optimum);
        assert!(optimum > 0.0);
        assert!(optimum < 2.0);
    }
    
    #[test]
    fn test_gda(){
        let rsg = SquareRandomSolutionGenerator{};
        let fitness_function = SquareFitnessFunction{};
        let gda : GDA<f64> = GDA::new(&rsg, &fitness_function, 10000, 100, -4000.0, 0.5);
        let optimum = gda.find();
        println!("Optimum: {}", optimum);
        assert!(optimum > 0.0);
        assert!(optimum < 2.0);
    }

    /*#[test]
    fn test_genetic(){
        let rsg = SquareRandomSolutionGenerator{};
        let fitness_function = SquareFitnessFunction{};
        let rg = FloatRecombinationGenerator{};
        let genetic : GeneticAlgorithm <f64> = GeneticAlgorithm::new(&rsg, &fitness_function, &rg, 60);
        let optimum = genetic.find();        
        println!("Optimum: {}", optimum);
        assert!(optimum > 0.0);
        assert!(optimum < 2.0);
    }*/

    #[test]
    fn test_recombination_generator(){
        let mut f1 = 1.0;
        let mut f2 = 2.0;
        let rg = FloatRecombinationGenerator{};
        let mut recombined = rg.recombine(&f1,&f2);
        assert!(recombined > 1.499999);
        assert!(recombined < 1.500001);
        f1 = -1.0;
        f2 = -2.0;
        recombined = rg.recombine(&f1, &f2);
        assert!(recombined > -1.500001);
        assert!(recombined < -1.499999);
        f1 = 1.0;
        f2 = -2.0;
        recombined = rg.recombine(&f1, &f2);
        assert!(recombined > -0.500001);
        assert!(recombined < -0.499999);
    }
    
    #[test]
    fn test_tsp_new(){
        let tsp = TSP::new(5);
        for x in 0..5{
            assert!(tsp.get_node(x) == x);
        }
    }
    
    #[test]
    fn test_tsp_ff_new(){
        let tsp_ff = TSPFitnessFunction::new(5);
        assert!(tsp_ff.get_distance(0, 1).is_finite());
        assert!(tsp_ff.get_distance(0, 5).is_infinite());
        assert!(tsp_ff.get_distance(5, 0).is_infinite());
        assert!(tsp_ff.get_distance(0, 0) == 0.0);
    }
        
    #[test]
    fn test_tsp_get_n_set(){
        let mut tsp = TSP::new(5);
        
        tsp.set_node(1,2);
        tsp.set_node(2,1);
        assert!(tsp.get_node(0) == 0);
        assert!(tsp.get_node(1) == 2);
        assert!(tsp.get_node(2) == 1);
        assert!(tsp.get_node(3) == 3);
        assert!(tsp.get_node(4) == 4);
    }
    
    #[test]
    fn test_tsp_clone(){
        let mut tsp = TSP::new(3);
        tsp.set_node(1, 2);
        tsp.set_node(2, 1);
        let tsp2 = tsp.clone();
        assert!(tsp2.get_node(0) == 0);
        assert!(tsp2.get_node(1) == 2);
        assert!(tsp2.get_node(2) == 1);
    }
    
    #[test]
    fn test_tsp_ff_clone(){
        let mut tsp_ff = TSPFitnessFunction::new(3);
        tsp_ff.set_distance(1, 0, 1.0);
        tsp_ff.set_distance(1, 2, 2.0);
        tsp_ff.set_distance(0, 2, 3.0);
        
        let tsp_ff_2 = tsp_ff.clone();
        
        assert!(tsp_ff_2.get_distance(0, 0) == 0.0);
        assert!(tsp_ff_2.get_distance(0, 1) == 1.0);
        assert!(tsp_ff_2.get_distance(0, 2) == 3.0);
        assert!(tsp_ff_2.get_distance(1, 0) == 1.0);
        assert!(tsp_ff_2.get_distance(1, 1) == 0.0);
        assert!(tsp_ff_2.get_distance(1, 2) == 2.0);
        assert!(tsp_ff_2.get_distance(2, 0) == 3.0);
        assert!(tsp_ff_2.get_distance(2, 1) == 2.0);
        assert!(tsp_ff_2.get_distance(2, 2) == 0.0);
    }
    
    #[test]
    fn test_tsp_fitness_function(){
        let mut tsp = TSP::new(3);
        let mut fitness_function = TSPFitnessFunction::new(3);
        fitness_function.set_distance(1, 0, 1.0);
        fitness_function.set_distance(1, 2, 2.0);
        fitness_function.set_distance(0, 2, 3.0);
        tsp.set_node(1, 2);
        tsp.set_node(2, 1);
       
        assert!((fitness_function.get_fitness(&tsp) > 2.9) &
                (fitness_function.get_fitness(&tsp) < 3.1));
        let tsp2 = TSP::new(4);
        assert!(fitness_function.get_fitness(&tsp2) == 0.0);
    }
    
    #[test]
    fn it_works() {
    }
    
    
}
