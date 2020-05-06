extern crate rand;

use itertools::Itertools;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::constants::*;

fn get_normal() -> f64 {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.sample(&mut rng)
}

pub struct State_Action {
    pub state: Vec<f64>,
    pub quality: Vec<f64>,
}

impl State_Action {
    pub fn new(state: Vec<f64>, num_actions: usize) -> State_Action {
        State_Action {
            state,
            //quality: (0..num_actions).map(|_| get_normal()).collect(),    
            quality: vec![0.0_f64; num_actions],    // Initializing at 0 gives objectively better results
        }
    }
}

pub struct QLearner {
    pub q: Vec<State_Action>,
    pub epsilon: f64,
    pub learning_rate: f64,
    pub discount_factor: f64,
    pub num_states: usize,
    pub len_states: usize,
    pub num_actions: usize,
}

impl QLearner {
    pub fn new(len_states: usize, num_actions: usize) -> QLearner {
        let states: Vec<Vec<usize>> = (0..len_states).map(|_| (0..2)).multi_cartesian_product().collect();
        let mut sa: Vec<State_Action> = Vec::new();
        for state in states {
            let a: Vec<f64> = state.iter().map(|x| *x as f64).collect();
            sa.push(State_Action::new(a, num_actions));
        }
        QLearner {
            num_states: sa.len(),
            q: sa,
            epsilon: EPSILON_GREEDY,
            learning_rate: LEARNING_RATE,
            discount_factor: DISCOUNT_FACTOR,
            len_states,
            num_actions,
        }
    }

    pub fn get_action(&mut self, state: &Vec<f64>) -> usize {
        // Following the epsilon-greedy policy
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() > 1.0_f64 - self.epsilon {
            rng.gen_range(0, self.num_actions)
        } else {
            let found = self.q.iter().find(|sa| sa.state == *state);
            match found {
                Some(sa) => get_index_max_float(&sa.quality).unwrap(),
                None => {
                    let state_copy: Vec<f64> = state.into_iter().map(|x| *x).collect();
                    let mut sa = State_Action::new(state_copy, self.num_actions);
                    let action = get_index_max_float(&sa.quality).unwrap();
                    self.q.push(sa);
                    action
                }
            }
        }
    }

    pub fn update_q(
        &mut self,
        state_initial: &Vec<f64>,
        action: usize,
        reward: f64,
        state_final: &Vec<f64>,
    ) -> Option<bool> {
        let index_initial = self.q.iter().position(|sa| sa.state == *state_initial);
        let index_final = self.q.iter().position(|sa| sa.state == *state_final);
        if index_initial == None || index_final == None {
            None
        } else {
            let ii = index_initial.unwrap();
            let fi = index_final.unwrap();
            self.q[ii].quality[action] = self.q[ii].quality[action]
                + self.learning_rate
                    * (reward + self.discount_factor * get_max_float(&self.q[fi].quality).unwrap()
                        - self.q[ii].quality[action]);
            Some(true)
        }
    }
}

fn get_index_max_float(input: &Vec<f64>) -> Option<usize> {
    input
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
}

fn get_max_float(input: &Vec<f64>) -> Option<&f64> {
    input
        .iter()
        .max_by(|&a, &b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_permutations() {
        let mut perms: Vec<Vec<usize>> = (0..3).map(|_| (0..2)).multi_cartesian_product().collect();
        println!("{:?}", perms);
        assert_eq!(perms.len(), 2_usize.pow(3));
    }

    #[test]
    fn test_get_index_max_float() {
        let mut q: Vec<f64> = (0..4).map(|_| get_normal()).collect();
        println!("{:?}", q);
        let i = get_index_max_float(&q);
        println!("{:?}", i);
        assert!(i.is_some());
    }

    #[test]
    fn test_get_max_float() {
        let mut q: Vec<f64> = (0..4).map(|_| get_normal()).collect();
        println!("{:?}", q);
        let q_max = get_max_float(&q);
        println!("{:?}", q_max);
        assert!(q_max.is_some());
    }

    #[test]
    fn test_qlearner_new() {
        let ql = QLearner::new(8, 4);
        assert_eq!(ql.q.len(), 2_usize.pow(8));
        assert_eq!(ql.q[0].state.len(), 8);
        assert_eq!(ql.q[0].quality.len(), 4);
        assert_eq!(ql.q[0].state, vec![0.0_f64; 8]);
    }

    #[test]
    fn test_qlearner_get_action() {
        let mut ql = QLearner::new(8, 4);
        let state = vec![0.0_f64; 8];
        let action = ql.get_action(&state);
        println!("{:?}", get_index_max_float(&ql.q[0].quality));
        assert_eq!(action, get_index_max_float(&ql.q[0].quality).unwrap());
    }
}
