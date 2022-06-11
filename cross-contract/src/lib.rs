use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::{env,near_bindgen};
use near_sdk::ext_contract;
use near_sdk::Gas;
use near_sdk::AccountId;

#[ext_contract(ext_puzzle)]
trait ExtPuzzle {
    fn set_solution(&mut self, solution: String);
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn guess_solution_callback(&mut self);
}

pub const CALLBACK_GAS: Gas = 5_000_000_000_000;
const CONTRACT_ID: &str = "dev-1652261684894-17500150718770";

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    solution :String
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn set_guess_solution_ext(&mut self,solution: String ){
       
        ext_puzzle::set_solution(
            solution,
            &CONTRACT_ID,
            0, //attach no yoctoNEAR to the method,
            CALLBACK_GAS
        )
        .then (
            ext_self::guess_solution_callback(
                &env::current_account_id(),
                0,
                CALLBACK_GAS
        ));
       
    }    

    pub fn guess_solution_callback(&self){
        let log_message = format!("Successfully assigned member to a team!");
        env::log(log_message.as_bytes());
    }
}




