extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
struct CSVParser;

const CSV_STR: &str = "customer_id,date,price,quantity
00001,2019-12-15,12.50,6
00002,2020-01-01,12.25,25
00003,2020-02-20,11.50,100
";

fn main() {
    let p = CSVParser::parse(Rule::file, CSV_STR)
        .expect("failed to parse")
        .next().unwrap();

    let mut rec_count: u32 = 0;
    for record in p.into_inner() {
        match record.as_rule() {
            Rule::record => {
                rec_count += 1;
                for field in record.into_inner() {
                    println!("{}: {:?}", rec_count, field.as_str());
                }
            }
            Rule::EOI => {},
            _ => unreachable!(),
        }
    }
}
