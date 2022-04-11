#[macro_use] extern crate rocket;

#[cfg(test)]
mod test;

mod apikey;

/** 
* Routing
*/

// Check if our Api is working

#[get("/")]
fn check() -> &'static str {
    "App is Live!!"
}

// parse dynamic data
// Try http://127.0.0.1:8000/antrix

#[get("/<name>")]
fn name(name: String) -> String {
    format!("Hello {} , Welcome to my first rust-rocket App", name)
}


/** 
* Validation
*/

// use of FormParam
// https://api.rocket.rs/v0.4/rocket/request/trait.FromParam.html
// for Validation

// mixture of query-strings and FormParam 
// Try http://127.0.0.1:8000/greet?name=antrix
// Try http://127.0.0.1:8000/greet?name=antrix&salutation=Mr

#[get("/greet?<name>&<salutation>")]
fn custom_greeting(name: String, salutation: Option<String>) -> String {
    match salutation {
        Some(s) => format!("Welcome {} {}", s, name),
        None => format!("Welcome {}", name)
    }
}

/** 
* Request Guards
*/

// try http://127.0.0.1:8000/auth
// in Headers put x-api-key = secretkey

#[get("/auth")]
fn protected(key: apikey::ApiKey) -> String {
    format!("You have been granted access to this api Key and your key is : '{}'", key.0)
}



// This will generate (async) main function
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![
        check,
        name,
        custom_greeting,
        protected
    ])
}