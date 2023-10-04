pub enum IndexingPhase {
    PastEvents,
    CurrentEvents,
}
pub struct LoopController {
    bot_start_block: u64,
    comptroller_creation_block: u64,
    i: u64,
    step_size: u64,
    failure_chain: u64,
    phase: IndexingPhase,
}

// TODO: make sure I didn't miss any edge cases
impl LoopController {
    pub fn new(
        bot_start_block: u64,
        comptroller_creation_block: u64,
        step_size: u64,
    ) -> LoopController {
        LoopController {
            bot_start_block,
            comptroller_creation_block,
            i: comptroller_creation_block,
            step_size,
            failure_chain: 0,
            phase: IndexingPhase::PastEvents,
        }
    }

    pub fn query_failure(&mut self) {
        // self.i -= self.step_size;
        self.step_size /= 2;
        self.failure_chain += 1;
        // println!("run failed, decreasing step size to {}", self.step_size);
    }

    pub fn query_successful(&mut self) {
        match &self.phase {
            IndexingPhase::PastEvents => {
                // state transition condition
                if self.i == self.bot_start_block {
                    self.phase = IndexingPhase::CurrentEvents;
                    println!("done indexing past events");
                }

                if self.failure_chain > 0 {
                    self.step_size *= 2;
                    self.failure_chain -= 1;
                    // println!("got a success, increasing step size to {}", self.step_size);
                }

                if self.i + self.step_size > self.bot_start_block {
                    self.step_size = self.bot_start_block - self.i;
                }

                self.i += self.step_size;
            }
            IndexingPhase::CurrentEvents => {
                self.i += 1;
            }
        }
    }

    pub fn from_block(&self) -> u64 {
        self.i
    }

    pub fn to_block(&self) -> u64 {
        match &self.phase {
            IndexingPhase::PastEvents => self.i + self.step_size,
            IndexingPhase::CurrentEvents => self.i,
        }
    }

    pub fn get_phase(&self) -> &IndexingPhase {
        &self.phase
    }

    pub fn get_i(&self) -> u64 {
        self.i
    }

    pub fn print_progress(&self) {
        match &self.phase {
            IndexingPhase::PastEvents => {
                let progress_percent = ((self.i - self.comptroller_creation_block) as f64
                    / (self.bot_start_block - self.comptroller_creation_block) as f64)
                    * 100 as f64;
                println!("processing past events {}%", progress_percent);
            }
            IndexingPhase::CurrentEvents => {
                println!("processing new block {}", self.i);
            }
        }
    }
}
