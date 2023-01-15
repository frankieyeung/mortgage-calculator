use serde::Deserialize;
use actix_web::{web, App, HttpResponse, HttpServer};

#[derive(Deserialize)]
struct MortgageParameters { 
    loan_amount: f64,
    year: u64,
    rate: f64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/mortgage_calculator", web::post().to(post_mortgage_calculator))
    });

    println!("Serving on http://localhost:4000...");
    server
        .bind("127.0.0.1:4000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Mortgage Calculator</title>
                <form action="/mortgage_calculator" method="post">
                Loan Amount: <input type="text" name="loan_amount"/>
                Year: <input type="text" name="year"/>
                Interest Rate(%): <input type="text" name="rate"/>
                <button type="submit">Calculate Monthy Payment</button>
                </form>
            "#,
        )
}

fn post_mortgage_calculator(form: web::Form<MortgageParameters>) -> HttpResponse {
    if form.loan_amount == 0.0 || form.year == 0 || form.rate == 0.0{
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Input can not be 0");
    }

    let response =
        format!("The monthly payment of loan amount ${} with {} years and {}% interest rate is
                $ {:.2}\n",
                form.loan_amount, form.year, form.rate, amortization(form.loan_amount, form.year, form.rate));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn amortization(loan_amount: f64, year: u64, rate: f64) -> f64 {
    assert!(loan_amount != 0.0 && year != 0 && rate != 0.0); 
    let rate_per_month = rate / 100.0 / 12.0; 
    let total_month = year * 12;
    let pvif = (1.0 - (1.0 /(1.0 + rate_per_month)).powf(total_month as f64))/ rate_per_month; //calculate Present Value Interest Factor(PVIF)
    return loan_amount / pvif; //monthly payment
}