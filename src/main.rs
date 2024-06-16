use bank::Bank;

fn main() {
    let mut bank = Bank::new();
    bank.create_account("X".to_string());
    bank.create_account("Y".to_string());

    bank.increase_account("X".to_string(), 10);
    bank.transfer("X".to_string(), "Y".to_string(), 5);
    bank.decrease_account("Y".to_string(), 2);

    bank.get_account_balance("X".to_string());//5
    bank.get_account_balance("Y".to_string());//3

    println!("{:?}", bank.get_history());
}