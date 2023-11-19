
// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


enum TikType {
    Backstage(String),
    Vip(String),
    Standard
}

// alternatively you can use ticket as enum with types as vip and all, with different fields, and compare the fields later
struct Ticket{
    ticket_type:TikType,
    price:f32,
}
fn main() {
    let tickets = vec![
        Ticket{ticket_type: TikType::Backstage("Katie".to_owned()), price:10.99 },
        Ticket{ticket_type: TikType::Vip("Jack".to_owned()), price:19.99 },
        Ticket{ticket_type: TikType::Standard, price:19.99 },
    ];
    for ticket in tickets {
        match ticket.ticket_type{
            TikType::Backstage(name) => println!("{} Backstage ticket, price: {:.2}", name, ticket.price),
            TikType::Vip(name) => println!("{} Vip ticket, price: {:.2}", name, ticket.price),
            TikType::Standard => println!("Standard ticket, price: {:.2}", ticket.price),
        
        
        }
}

}


// alternatively

