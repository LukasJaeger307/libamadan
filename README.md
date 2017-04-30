# libamadan
A free metaheuristics library for Rust.
## What are metaheuristics?
A heuristic is a strategy to find solutions for a specific problem that are not optimal but close to it. Ideally those solutions are found in way less time than exact solutions. A metaheuristic is a type of heuristic that is applicable to more than one problem.
## Where can metaheuristics be used
There is a large number of problems where it is inherently difficult to find an exact solution but a close-to-optimum solution will suffice. The travelling salesman problem is one example for such a problem. Many more can be found in the field of hardware synthesis: Scheduling, place-and-route and many more problems are solved heuristically. If you have a difficult problem, where generating random solutions is easy and fast and their quality is easy to measure, then metaheuristics are probably a good choice for your problem.
## How is libamadan applied?
If you want to solve your problem using libamadan, make sure you implement the FitnessFunction- and the RandomSolutionGenerator-trait for you problem. The FitnessFunction is used to measure the quality of a solution. Make sure, good solutions have a higher fitness value than bad solutions. If the quality of your solution is measured in a way that good solutions have a lower fitness value than bad ones (for example the kilometres in a travelling salesman problem), you need to scale your fitness. The RandomSolutionGenerator generates random solutions (get out of town...) and makes small random changes to them. Make sure, the data type of your solution implements the Clone-trait. If you have done all that, you can import libamadan and use all provided metaheuristics to solve your problem.
## What kind of whacky license is that? Are you serious about it?
Well of course! I want to leverage the use of both metaheuristics and Rust and the best way to leverage the use of something is to remove all restrictions. Besides, writing the code of a metaheuristic is the easy part. Applying it to a specific problem is a different story. So go ahead, use my solution for the easy part the way you want to. No strings attached.
