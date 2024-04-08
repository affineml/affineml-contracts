use crate::constants::*;
use crate::errors::BizError;
use anchor_lang::{
    prelude::*,
    solana_program::hash::{self},
};

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub bump: u8,
    // reward token mint
    pub mint: Pubkey,
    // total task
    pub task_count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct TaskInfo {
    pub bump: u8,
    pub creator: Pubkey,
    // flag: whether the task is revealed
    pub ended: bool,
    // commit info
    pub cur_commit_user: [Pubkey; MAX_COMMIT_USER_NUM],
    // total commit user num
    pub total_user: u8,
    // commit reward per user
    pub commit_reward: u64,
    // total round
    pub total_round: u8,
    // cur round
    pub cur_round: u8,
    // cur round
    pub cur_round_commit: u8,
}

impl TaskInfo {
    pub fn check_commitable(&self) -> Result<()> {
        //check task status
        require!(!self.ended, BizError::EndedError);
        Ok(())
    }

    // Check if the user's commit is correct.
    pub fn check_commit_result(&self, reveal_info: &[u8; 32], user_info: &UserInfo) -> bool {
        // check round
        if self.cur_round != user_info.last_round {
            return false;
        }
        let hash_result = hash::hashv(&[reveal_info, &user_info.user_pubkey.to_bytes()]);
        return hash_result.to_bytes() == user_info.commit_hash;
    }

    // // Check if the user's commit is correct.
    // pub fn check_commit_result(&self, reveal_info: &[u8; 32], user_info: &UserInfo) -> bool {
    //     let hash_result = hash::hashv(&[reveal_info, &user_info.user.to_bytes()]);
    //     return hash_result.to_bytes() == user_info.commit_hash;
    // }

    // Check if the user's commit is correct.
    // pub fn check_user_commit_result(&self, reveal_info: &[u8; 32], user: Pubkey) -> bool {
    //     for (_, user_info) in self.user_info.iter().enumerate() {
    //         if &user_info.user == &user {
    //             return self.check_commit_result(reveal_info, user_info);
    //         }
    //     }
    //     false
    // }
}

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
    // init flag
    pub init: bool,
    // user pubkey
    pub user_pubkey: Pubkey,
    // commit hash
    pub commit_hash: [u8; 32],
    // user's successful commit
    pub commit_count: u8,
    // user's last commit rount
    pub last_round: u8,
    // claim flag
    pub claimed: bool,
}
