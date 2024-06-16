use std::collections::{HashMap, HashSet};
use crate::BankError::AccountAlreadyExists;

type OperationId = usize;

#[derive(Debug)]
pub struct Bank {
    // Счета
    accounts: HashSet<String>,
    // Балансы
    balances: HashMap<String, i32>,
    // История счета
    account_operations_index: HashMap<String, Vec<OperationId>>,
    // История
    history: Vec<Operation>,
}

#[derive(Debug)]
pub enum Operation {
    CreateAccount(String),
    IncreaseAccount(String, i32),
    DecreaseAccount(String, i32),
    Transfer(String, String, i32),
}

#[derive(Debug)]
pub enum BankError {
    AccountAlreadyExists(String),
    IncorrectAmount(i32),
    InsufficientFunds(i32),
    TransferToMyself,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: HashSet::new(),
            balances: HashMap::new(),
            account_operations_index: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn get_account_balance(&self, account: String) -> Result<i32, BankError> {
        if !self.accounts.contains(&account) {
            return Err(AccountAlreadyExists(
                format!(
                    "Account {} does not exist", account)));
        }
        let balance = *self.balances.get(&account).unwrap();
        Ok(balance)
    }

    pub fn create_account(&mut self, account: String) -> Result<(), BankError> {
        if !self.accounts.contains(&account) {
            self.accounts.insert(account.clone());
            self.balances.insert(account.clone(), 0);
            self.history.push(Operation::CreateAccount(account));
            Ok(())
        } else {
            Err(AccountAlreadyExists(
                format!(
                    "Account {} already exists", account)))
        }
    }

    pub fn increase_account(&mut self, account: String, amount: i32) -> Result<(), BankError> {
        self.update_balance_account(account.clone(), amount)?;
        self.history.push(Operation::IncreaseAccount(account, amount));
        Ok(())
    }

    pub fn decrease_account(&mut self, account: String, amount: i32) -> Result<(), BankError> {
        self.update_balance_account(account.clone(), (-1) * amount)?;
        self.history.push(Operation::DecreaseAccount(account, amount));
        Ok(())
    }

    pub fn transfer(&mut self, from: String, to: String, amount: i32) -> Result<(), BankError> {
        if from == to {
            Err(BankError::TransferToMyself)
        } else {
            self.update_balance_account(from.clone(), (-1) * amount)?;
            self.update_balance_account(to.clone(), amount)?;
            self.history.push(Operation::Transfer(from, to, amount));
            Ok(())
        }
    }

    fn update_balance_account(&mut self, account: String, amount: i32) -> Result<(), BankError> {
        if !self.accounts.contains(&account) {
            return Err(AccountAlreadyExists(format!("Account {} does not exist", account)));
        }

        if amount == 0 {
            return Err(BankError::IncorrectAmount(amount));
        }

        let current_balance = *self.balances.get(&account).unwrap();
        let new_balance = current_balance + amount;

        if new_balance < 0 {
            return Err(BankError::InsufficientFunds(amount));
        }

        self.balances.insert(account.clone(), new_balance);
        Ok(())
    }

    pub fn get_history(&self) -> &Vec<Operation> {
        &self.history
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_account() {
        let b = Bank::new();
        assert_eq!(0, b.history.len());
    }

    #[test]
    fn create_account_twice() {
        let mut b = Bank::new();
        let _ = b.create_account("X".to_string());
        let x = b.create_account("X".to_string());
        assert!(x.is_err());
    }

    #[test]
    fn increase_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.increase_account("X".to_string(), 10);
        assert!(x.is_ok());
        let balance = bank.balances.get(&"X".to_string()).unwrap();
        assert_eq!(10, *balance);
    }

    #[test]
    fn increase_account_zero() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.increase_account("X".to_string(), 0);
        assert!(x.is_err());
    }

    #[test]
    fn increase_no_account() {
        let mut bank = Bank::new();
        let x = bank.increase_account("X".to_string(), 10);
        assert!(x.is_err());
    }

    #[test]
    fn get_account_balance() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.get_account_balance("X".to_string());
        assert!(x.is_ok());
        assert_eq!(0, x.unwrap());
    }

    #[test]
    fn get_no_account_balance() {
        let mut bank = Bank::new();
        let x = bank.get_account_balance("X".to_string());
        assert!(x.is_err());
    }

    #[test]
    fn decrease_from_no_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.decrease_account("Y".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn decrease_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.decrease_account("X".to_string(), 5);
        assert!(x.is_ok());
        let balance = bank.balances.get(&"X".to_string()).unwrap();
        assert_eq!(5, *balance);
    }

    #[test]
    fn decrease_no_account() {
        let mut bank = Bank::new();
        let x = bank.decrease_account("X".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn decrease_account_zero() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.decrease_account("X".to_string(), 0);
        assert!(x.is_err());
    }

    #[test]
    fn decrease_account_too_much() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.decrease_account("X".to_string(), 20);
        assert!(x.is_err());
    }

    #[test]
    fn transfer() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "Y".to_string(), 5);
        assert!(x.is_ok());
        let balance = bank.balances.get(&"X".to_string()).unwrap();
        assert_eq!(5, *balance);
        let balance = bank.balances.get(&"Y".to_string()).unwrap();
        assert_eq!(5, *balance);
    }

    #[test]
    fn transfer_zero() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "Y".to_string(), 0);
        assert!(x.is_err());
    }

    #[test]
    fn transfer_to_self() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "X".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn transfer_to_no_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "Y".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn get_history() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        // let _ = bank.increase_account("X".to_string(), 10);
        // let _ = bank.transfer("X".to_string(), "Y".to_string(), 5);
        let history = bank.get_history();
        assert_eq!(1, history.len());
        // assert_eq!(3, history.len());
        println!("{:?}", history)
    }
}

