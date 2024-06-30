use bank::Bank;

fn main() {
    let mut bank = Bank::new();
    let _ = bank.create_account("X".to_string());
    let _ = bank.create_account("Y".to_string());

    let _ = bank.increase_account("X".to_string(), 10);
    let _ = bank.transfer("X".to_string(), "Y".to_string(), 5);
    let _ = bank.decrease_account("Y".to_string(), 2);

    let _ =bank.get_account_balance("X".to_string());//5
    let _ =bank.get_account_balance("Y".to_string());//3

    println!("{:?}", bank.get_history()); // 5 operations

    let mut new_bank = Bank::new();

    // Restore operation from another bank. Don't return errors because bank history always correct.
    new_bank.restore(bank.get_history());
}