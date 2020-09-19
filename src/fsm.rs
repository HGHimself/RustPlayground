struct FSM {
    inputs: Vec<usize>,
    states: Vec<usize>,
    initial_state: usize,
    transition_table: Vec<Vec<usize>>,
    final_states: Vec<usize>,
}

impl FSM {
    pub fn new(inputs: Vec<usize>,
        states: Vec<usize>,
        initial_state: usize,
        transition_table: Vec<Vec<usize>>,
        final_states: Vec<usize>
    ) -> Self {
        // for now we will do no checking to see if the params are valid
        FSM {
            inputs,
            states,
            initial_state,
            transition_table,
            final_states
        }
    }

    pub fn transition(&self, state: usize, input: usize) -> usize {
        self.transition_table[state][input]
    }

    pub fn final(&self, state: usize) -> bool {
        self.final_states.contains(&state)
    }

    pub fn initial(&self) -> usize {
        self.initial_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn fsm_constructs() {
        let inputs = vec![2,4,6,];
        let states = vec![1,3,5,];
        let initial_state = 1;
        let transition_table = vec![
            /*1*/vec![0,2,1],
            /*3*/vec![2,1,0],
            /*5*/vec![1,2,0],
        ];
        
        let fsm = Fsm::new()
        assert_eq!(2 + 2, 4);
    }
}
