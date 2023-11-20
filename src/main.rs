use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "Huff.pest"] // This should point to your huff.pest file
pub struct HuffParser;

fn main() {
    let unparsed_file = "// comment :D\n"; // Replace this with actual Huff code

    match HuffParser::parse(Rule::huff, &unparsed_file) {
        Ok(pairs) => {
            for pair in pairs {
                // Process the parsed pairs
                println!("Rule: {:?}, Span: {:?}", pair.as_rule(), pair.as_span());
            }
        }
        Err(error) => {
            println!("Error parsing file: {}", error);
        }
    }
}


/*fn main() {
    println!("Hello, world!");
}*/
