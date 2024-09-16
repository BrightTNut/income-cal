// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, result};

slint::include_modules!();

const TAXPER: f64 = 0.25;
const OWNERPER: f64 = 0.5;
const PROFITER: f64 = 0.1;
const OPEXPER: f64 = 0.15;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });


    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string|{
                let ui = ui_handle.unwrap();
                let num: f64 = string.trim().parse().unwrap();
                let tax: f64 = num * TAXPER;
                let owner: f64 = num * OWNERPER;
                let profit: f64 = num * PROFITER;
                let opex: f64 = num * OPEXPER;
                let result:String =format!("Taxes: {:.2} \n Owner: {:.2} \n Profits: {:.2} \n OpEx: {:.2}", tax,owner,profit,opex);
                ui.set_results(result.into());
            
        });

    ui.run()?;

    Ok(())
}
