use anchor_lang::prelude::*;

use crate::constants::MAX_POINT_PER_QUEST;

#[account]
pub struct AppData {
    pub total_point_collected: u64,
}

impl AppData {
    pub fn on_task_initialized(&mut self, amount_initialized: u64) -> Result<()> {
        match self.total_point_collected.checked_add(amount_initialized) {
            Some(v) => {
                if self.total_task_collected >= MAX_TASK_PER_QUEST {
                    self.total_task_collected = 0;
                    msg!("Task successfully initialized.");
                } else {
                    self.total_point_collected = v;
                    msg!("Total point initialized: {}", v);
                }
            }
            None => {
                msg!("The ever Quest is completly initialized!");
            }
        };

        Ok(())
    }
}
