use bank::Bank;

fn main() {
    let mut bank = Bank::default();
    let _ = bank.create_account("X".to_string());
    let _ = bank.create_account("Y".to_string());

    let _ = bank.increase_account("X".to_string(), 10);
    let _ = bank.transfer("X".to_string(), "Y".to_string(), 5);
    let _ = bank.decrease_account("Y".to_string(), 2);

    let _ = bank.get_account_balance("X".to_string()); //5
    let _ = bank.get_account_balance("Y".to_string()); //3

    let mut new_bank = Bank::default();
    new_bank.restore(bank.get_history());
    println!("{:?}", bank.get_history()); // 5 operations
    println!("{:?}", new_bank.get_history()); // 5 operations

    let x_account_balance = new_bank.get_account_balance("X".to_string()); //5
    let y_account_balance = new_bank.get_account_balance("Y".to_string()); //3
    println!(
        "Account balances X:{:?}, Y:{:?}",
        x_account_balance.unwrap(),
        y_account_balance.unwrap()
    ); // 5, 3
}
