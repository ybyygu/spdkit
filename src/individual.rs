// [[file:~/Workspace/Programming/structure-predication/spdkit/spdkit.note::eba13813-84c4-4a1e-8d3a-3f7bbc5462b0][eba13813-84c4-4a1e-8d3a-3f7bbc5462b0]]
pub trait Individual {
    fn mutate(&mut self);
    fn fitness(&mut self) -> f64;
    fn reset(&mut self);
}
// eba13813-84c4-4a1e-8d3a-3f7bbc5462b0 ends here
