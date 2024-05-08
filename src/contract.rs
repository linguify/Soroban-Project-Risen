use soroban_sdk::{Env, Symbol, Binary, vec, Address, IntoVal, Bytes, Ed25519Signature, AccountId};

struct Player {
    score: u32,
    rewards_claimed: u32,
}

pub struct P2ERewardsContract;

#[contractimpl]
impl P2ERewardsContract {
    pub fn new(e: Env, admin: Address, reward_token: Symbol) {
        e.data().set(Symbol::from_str("admin"), admin.into_val(&e));
        e.data().set(Symbol::from_str("reward_token"), reward_token);
    }

    pub fn report_score(&self, e: Env, player: Address, score: u32, signature: Ed25519Signature) -> Result<(), String> {
        
        let player_account_id = AccountId::from_address(player.clone())?;
        player_account_id.verify(&Bytes::from_vec(score.to_be_bytes().to_vec()), &signature)?;

        let players = Binary::new(e, Symbol::from_str("players"));
        players.set(&player, &Player { score, rewards_claimed: 0 });
        Ok(())
    }

    pub fn claim_rewards(&self, e: Env, player: Address) -> Result<u32, String> {
        let players = Binary::new(e, Symbol::from_str("players"));
        let reward_token = e.data().get(Symbol::from_str("reward_token"))
                                 .unwrap().try_into().unwrap(); 

        let mut player_data = players.get(&player)
                                  .ok_or("Player not found".to_string())?; 

        
        let calculated_reward = self.calculate_reward(&e, player_data.score); 


        player_data.rewards_claimed += calculated_reward;
        players.set(&player, &player_data); 

        Ok(calculated_reward)
    }

    pub fn deposit_rewards(&self, e: Env, amount: u32, from: AccountId, signature: Ed25519Signature) -> Result<(), String> {
        let admin = e.data().get(Symbol::from_str("admin")).unwrap().try_into().unwrap();
        if from != admin {
            return Err("Only the admin can deposit rewards".to_string());
        }

        from.verify(&Bytes::from_vec(amount.to_be_bytes().to_vec()), &signature)?;
        Ok(())
    }

    
    fn calculate_reward(&self, e: &Env, score: u32) -> u32 {
         
         if score >= 1000 {
            return 50; // 50 reward tokens
         } else {
            return 10; // 10 reward tokens
         } 
    }
}
