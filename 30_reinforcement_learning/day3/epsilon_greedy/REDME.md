# Epsilon-Greedy method

Using the `Application` trait and `ConinToss` struct.

`Application` trait has each actions of  coin-toss game  and `CoinToss` struct has each game parameter. Then, `CoinToss` impl is defined as coin-toss game actual action implimentation.

Application : `trait`
- init()
- action()
- reset()

CoinToss : `strust`
- head_probs
- max_episode_steps
- toss_count

CoinToss : `impl`
- step()
- get_count()
- show_parameter()