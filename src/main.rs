use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "Huff.pest"] // This should point to your huff.pest file
pub struct HuffParser;

// return parsed file or error
fn parse(input: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    let pairs = HuffParser::parse(Rule::huff, input)?;
    Ok(pairs)
}

fn main() {
    let unparsed_file = "// comment :D\n"; // Replace this with actual Huff code
    let parsed_file = parse(unparsed_file).unwrap();
    for line in parsed_file {
        println!("Rule:    {:?}", line.as_rule());
        println!("Span:    {:?}", line.as_span());
        println!("Text:    {}", line.as_str());
        println!("Pairs:   {:?}", line.into_inner());
    }       
    
}


/*fn main() {
    println!("Hello, world!");
}*/

#[cfg(test)]
mod tests {
    use crate::parse;
    #[test]

    fn it_works_with_comments() {
        let unparsed_file = "// comment :D\n"; // Replace this with actual Huff code

        let results = parse(unparsed_file).unwrap();
        assert_eq!(results.as_str(), unparsed_file);
    }
}
