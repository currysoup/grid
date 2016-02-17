pub trait ProbingStrategy {
    fn probe_stride(&self) -> usize,
}

pub struct LinearProbingStrategy;

impl ProbingStrategy for LinearProbingStragey {
    fn probe_stride(&self) -> usize {
        1
    }
}
