use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct UserData {
    pub authority: Pubkey,
    pub name: String,
    pub level: u8,
    pub xp: u64,
    pub point: u64,
    pub quest: u64,
    pub last_login: i64,
    pub last_id: u16,
}

impl UserData {
    pub fn print(&mut self) -> Result<()> {
        msg!(
            "Authority: {} point: {} Quest: {}",
            self.authority,
            self.point,
            self.quest
        );
        Ok(())
    }

    pub fn update_quest(&mut self) -> Result<()> {
        // Get the current timestamp
        let current_timestamp = Clock::get()?.unix_timestamp;

        // Calculate the time passed since the last login
        let mut time_passed: i64 = current_timestamp - self.last_login;

        let mut time_spent = 0;

        while time_passed >= TIME_TO_REFILL_QUEST && self.quest < MAX_QUEST {
            self.quest += 1;
            time_passed -= TIME_TO_REFILL_QUEST;
            time_spent += TIME_TO_REFILL_QUEST;
        }

        if self.energy >= MAX_QUEST {
            self.last_login = current_timestamp;
        } else {
            self.last_login += time_spent;
        }

        Ok(())
    }

    pub fn chop_tree(&mut self, amount: u64) -> Result<()> {
        match self.point.checked_add(amount) {
            Some(v) => {
                self.point = v;
            }
            None => {
                msg!("Total point reached!");
            }
        };
        match self.quest.checked_sub(amount) {
            Some(v) => {
                self.quest = v;
            }
            None => {
                self.quest = 0;
            }
        };
        Ok(())
    }
}
