#[derive(Debug, PartialEq)]
enum Payment {
    Cash {
        received_cents: u32,
    },
    Card {
        last4: String,
        authorization_code: String,
    },
    BankTransfer {
        reference: String,
    },
}

fn describe(payment: &Payment) -> String {
    match payment {
        Payment::Cash { received_cents } => {
            format!("cash: {received_cents} cents")
        }
        Payment::Card {
            last4,
            authorization_code,
        } => format!("card {last4}, auth {authorization_code}"),
        Payment::BankTransfer { reference } => {
            format!("transfer {reference}")
        }
    }
}

fn main() {
    let payment = Payment::Card {
        last4: String::from("4242"),
        authorization_code: String::from("AUTH-7"),
    };

    assert_eq!(describe(&payment), "card 4242, auth AUTH-7");
}
