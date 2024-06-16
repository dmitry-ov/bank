use bank::Bank;

fn main()   {
  let mut bank = Bank::new();

  bank.create_account("X".to_string());
  bank.create_account("Y".to_string());

  println!("{:?}", bank);
}