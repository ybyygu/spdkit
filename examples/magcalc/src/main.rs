// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::39fd755a-949c-492e-ab0c-c74826bf43f4][39fd755a-949c-492e-ab0c-c74826bf43f4]]
extern crate csv;
#[macro_use] extern crate quicli;
#[macro_use] extern crate serde_derive;

use std::io;
use std::collections::HashMap;
use quicli::prelude::*;

#[derive(Debug,Deserialize)]
struct Record {
    directory: String,
    energy: f64,
    seqs: String,
    net_mag: usize,
}

// read data records from an external csv file
fn read_data(filename: &str) -> Result<HashMap<String, f64>> {
    let mut rdr = csv::Reader::from_path(filename)?;

    let mut data = HashMap::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        data.insert(record.directory, record.energy);
    }

    Ok(data)
}

#[test]
fn test_read_data() {
    let filename = "tests/files/results.csv";
    let x = read_data(filename).expect("magresult");
    // println!("{:#?}", x);
}
// 39fd755a-949c-492e-ab0c-c74826bf43f4 ends here

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::65413bca-085a-4f26-8718-0241c853f9dd][65413bca-085a-4f26-8718-0241c853f9dd]]
extern crate genevo;
extern crate rand;
extern crate fixedbitset;
#[macro_use] extern crate lazy_static;

use genevo::prelude::*;
use genevo::operator::prelude::*;
use genevo::types::fmt::Display;
// use fixedbitset::FixedBitSet;
use genevo::population::{BinaryEncodedGenomeBuilder, ValueEncodedGenomeBuilder};

use std::iter::FromIterator;

type MagGenome = Vec<bool>;

#[derive(Clone,Debug,PartialEq)]
struct MagFitnessEvaluator;

// global data for energies
lazy_static! {
    static ref DATA: HashMap<String, f64> = {
        let filename = "tests/files/results.csv";
        read_data(filename).expect("magresult")
    };
}

fn calc_fitness(energy: f64) -> u32 {
    let temperature = 8000.;
    let eref = -205.40;
    let value = (energy - eref)*96.;
    let fitness = (-1.0*value / (temperature*0.0083145)).exp();

    (fitness * 1000.) as u32
}

#[test]
fn test_calc_fit() {
    let x = -205.30249;
    let x = calc_fitness(x);
    println!("{:#?}", x);
}

impl FitnessFunction<MagGenome, u32> for MagFitnessEvaluator {
    fn fitness_of(&self, individual: &MagGenome) -> u32 {
        let mut key = String::from("1");
        for i in 0..individual.len() {
            let x = match individual[i] {
                true => "1",
                false => "0"
            };
            key.push_str(x);
        }

        let energy = DATA[&key];
        let fitness = calc_fitness(energy);
        println!("{:#?}", fitness);

        fitness
    }

    fn average(&self, fitness_values: &[u32]) -> u32 {
        (fitness_values.iter().sum::<u32>() / fitness_values.len() as u32)
    }

    fn highest_possible_fitness(&self) -> u32 {
        1000
    }

    fn lowest_possible_fitness(&self) -> u32 {
        0
    }
}

fn create_population() -> Population<MagGenome> {
    build_population()
        .with_genome_builder(BinaryEncodedGenomeBuilder::new(11))
        .of_size(20)
        .uniform_at_random()
}

#[test]
fn test_genevo_magcalc() {
    let initial_population = create_population();

    println!("{:?}", initial_population);

    let algorithm = genetic_algorithm()
        .with_evaluation(MagFitnessEvaluator)
        .with_selection(RouletteWheelSelector::new(0.7, 2))
        .with_crossover(MultiPointCrossBreeder::new(3))
        .with_mutation(RandomValueMutator::new(0.1, false, true))
        .with_reinsertion(ElitistReinserter::new(MagFitnessEvaluator, false, 0.7))
        .with_initial_population(initial_population)
        .build();

    let mut magcalc_sim = simulate(algorithm)
        .until(GenerationLimit::new(50))
        .build();

    loop {
        let result = magcalc_sim.step();
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!("Step: generation: {}, average_fitness: {}, \
                          best fitness: {}, duration: {}, processing_time: {}",
                         step.iteration, evaluated_population.average_fitness(),
                         best_solution.solution.fitness,
                         step.duration.fmt(),
                         step.processing_time.fmt());
                // for row in best_solution.solution.genome.as_board() {
                //     println!("      {:?}", row);
                // }
            },
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!("Final result after {}: generation: {}, \
                          best solution with fitness {} found in generation {}, processing_time: {}",
                         duration.fmt(), step.iteration,
                         best_solution.solution.fitness,
                         best_solution.generation,
                         processing_time.fmt());
                // for row in best_solution.solution.genome.as_board() {
                //     println!("      {:?}", row);
                // }
                break;
            },
            Err(error) => {
                println!("{}", error.display());
                break;
            },
        }
    }
}
// 65413bca-085a-4f26-8718-0241c853f9dd ends here

// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::47a6cbcd-2168-440c-9c7c-c8a25018bc61][47a6cbcd-2168-440c-9c7c-c8a25018bc61]]
fn main() {
    //
}
// 47a6cbcd-2168-440c-9c7c-c8a25018bc61 ends here
