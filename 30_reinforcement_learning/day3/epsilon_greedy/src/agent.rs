pub trait EpsilonGreedyAgent{
    fn init(epsilon: f32) -> Self;
    fn policy(&mut self) -> f32;
    fn play(&mut self, env: cointoss::Application)

}

pub struct Agent {
    epsilon: f32,
    expect_vec : Vec<f32>,
}