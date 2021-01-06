#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// start a simc from the simc string
#[post("/sim", data = "<simc_string>")]
fn sim(simc_string: String) -> &'static str {
    "sim_simc_string not implemented"
}

// gets the version of the current installed simc
#[get("/simc/version")]
fn simc_version() -> &'static str {
    "simc_version not implemented"
}

// upgrade the currently installed simc to the latest available version
#[get("/simc/upgrade")]
fn simc_upgrade() -> &'static str {
    "simc_upgrade not implemented"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![sim, simc_version, simc_upgrade])
        .launch();
}

#[cfg(test)]
mod tests;
