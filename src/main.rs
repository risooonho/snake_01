mod brain;
mod constants;
mod game;
mod render;

extern crate rayon;

use rayon::prelude::*;

use crate::brain::{Population, NN};
use crate::constants::*;
use crate::game::Game;
use crate::render::Render;

fn main() {
    iterate_population(NUM_INDIVIDUALS, NUM_GAMES, NUM_GENERATIONS);
}

fn fitness_function(delta_t: i64, dist_before: i64, dist_after: i64, snake_eat: i64, snake_dead: i64) -> i64 {
    let mut fitness: i64 = 0;
    if dist_after < dist_before {
        fitness += 5;
    } else {
        fitness -= 1;
    }
    fitness += dist_after % 2;
    fitness += 200 * snake_eat;
    fitness
    //500 * score + time - 2 * food_distance
    //100 * score + score * 1000 / (time + 1)  + time - food_distance
}

fn iterate_population(num_nn: u32, num_games: u32, num_generations: u32) {
    let mut pop = Game::get_population(num_nn);
    for i in 0..num_generations - 1 {
        pop.fitness = population_play_parallel(&pop, num_games);
        let sorted_index = pop.get_sorted_index();
        println!("Gen: {}; Fitness: {}", i, pop.fitness[sorted_index[0]]);
        pop = pop.create_next_generation();
    }

    pop.fitness = population_play_parallel(&pop, num_games);
    let sorted_index = pop.get_sorted_index();
    println!("Final Fitness: {}", pop.fitness[sorted_index[0]]);
    let mut render = Render::new();
    render.run_nn(&pop.nn[sorted_index[0]]);
}

fn population_play_parallel(pop: &Population, num_games: u32) -> Vec<i64> {
    let fitness: Vec<i64> = pop
        .nn
        .par_iter()
        .map(|n| nn_play(n, num_games, fitness_function))
        .collect();
    fitness
}

fn nn_play(nn: &NN, num_games: u32, fitness_function: fn(i64, i64, i64, i64, i64) -> i64) -> i64 {
    let mut game = Game::new();
    let mut fitness: i64 = 0;
    for _ in 0..num_games {
        game.init();
        fitness += game.run_nn(nn, fitness_function);
    }
    fitness
}

fn get_snake_dist(game: &Game) -> i64 {
    let dist_x = (game.snake.body[0].position.x as i64 - game.food.position.x as i64).abs();
    let dist_y = (game.snake.body[0].position.y as i64 - game.food.position.y as i64).abs();
    dist_x + dist_y
}
