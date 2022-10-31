use std::env;
use std::io;
use the_new_blockchain::ledger::block;
use the_new_blockchain::ledger::blockchain::Blockchain;
use the_new_blockchain::wallet::transaction;
use the_new_blockchain::wallet::wallet;

mod cli;
use structopt::StructOpt;
// use cursive::{Cursive, CursiveExt};
// use cursive::traits::Nameable;  
// use cursive::views::*;
// use cursive::event::*;
// use cursive::Printer;

fn main() {
    // let mut bc_app = cursive::default();
    // show_menu(&mut bc_app, "Start");
    cli::CommandLineArgs::from_args();

    let mut bc = Blockchain::new();



    // let new_block = block::Block::new(bc.get_last_block(), vec![], vec![]); 
    // bc.blocks.push(new_block);
    
}

// fn show_menu(bc_app: &mut Cursive, msg: &str){
//     bc_app.add_layer(Dialog::around(
//         LinearLayout::vertical()
//         .child(TextView::new("Enter Public Key"))
//         .child(EditView::new().with_name("pub_key"))
//         .child(TextView::new("Enter Private Key"))
//         .child(EditView::new().with_name("priv_key"))
//     )

//     .button("Validate Keys", |s|{
//         let pub_key = s.call_on_name("pub_key", |view: &mut EditView| view.get_content()).unwrap();
//         let priv_key = s.call_on_name("priv_key", |view: &mut EditView| view.get_content()).unwrap();
//         let valid = wallet::Wallet::validate_private_key(&pub_key, &priv_key);
//     })
//     .button("Send", |s| {
//         let dialog = Dialog::text("Send")
//         .title("Create Transactions")
//         .content(
//             Dialog::around(
//                 LinearLayout::vertical()
//                 .child(TextView::new("Enter Public Key"))
//                 .child(EditView::new().with_name("pub_key"))
//                 .child(TextView::new("Enter Amount"))
//                 .child(EditView::new().with_name("amount"))
//             )
//         )
//         .dismiss_button("Cancel")
//         .button("Okay", |s| {
//             let r = s.call_on_name("recipient", |view: &mut EditView| view.get_content()).unwrap();
//             transaction::Transaction::new(&sender_public_key, &sender_private_key, &recipient_public_key, &amount);
//         });
//         s.add_layer(dialog); 
//     })
//     .button("Quit", |s| s.quit())
//     );  
//     bc_app.run();
// }
