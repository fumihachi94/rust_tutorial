use rand::Rng;

pub trait Application {
    fn init(head_probs: Vec<f32>, max_episode_steps: u32, toss_count: u32) -> Self;
    fn action(&mut self, coin_num: u32) -> (f32, bool);
    fn reset(&mut self);
}

pub struct CoinToss {
    head_probs: Vec<f32>,
    max_episode_steps: u32,
    toss_count: u32,
}

impl Application for CoinToss {
    fn init(head_probs: Vec<f32>, max_episode_steps: u32, toss_count: u32) -> CoinToss {
        CoinToss {
            head_probs,
            max_episode_steps,
            toss_count,
        }
    }

    fn action(&mut self, coin_num: u32) -> (f32,bool) {
        return self.step(coin_num)
    }

    fn reset(&mut self){
        self.toss_count = 0;
    }
}

impl CoinToss {
    pub fn step(&mut self, coin_num: u32) -> (f32, bool) {
        let _max_count = self.max_episode_steps - 1;
        let mut _done = false;
        let mut _reward = 0.0;

        if self.toss_count > _max_count {
            panic!("Done. Please reset Application.");
        } else if self.toss_count == _max_count {
            _done = true;
        }

        if coin_num >= self.head_probs.len() as u32 {
            panic!("The No.{} coin doesn't exist", coin_num);
        } else {
            let _head_prob: f32 = self.head_probs[coin_num as usize];
            
            let mut _rng = rand::thread_rng();
            let _ramd_prob: f32 =  _rng.gen();

            if _ramd_prob < _head_prob {
                _reward = 1.0;
            }else{
                _reward = 0.0;
            }
            println!("reward : {} ({}, {})", _reward, _ramd_prob, _head_prob);
            self.toss_count += 1;
        }

        return(_reward, _done);
    }

    pub fn get_count(&self) -> u32 {
        self.toss_count
    }

    pub fn show_parameter(&self) {
        println!("--show parameters--");
        println!("head_probs : {:?}", self.head_probs);
        println!("max_episode_steps : {}", self.max_episode_steps);
        println!("toss_count : {}", self.toss_count);
        println!("-------------------");
    }
}