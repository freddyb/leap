#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[derive(PartialEq, Debug)]
enum ParserState {
    Text,
    ColorCode,
    Foreground1,
    Foreground2,
    Comma,
    Background1,
}
struct Parser {
    state: ParserState,
}

pub fn strip(tostrip: &str) -> String {
    let mut parser = Parser {
        state: ParserState::Text,
    };

    let result: String = tostrip
        .chars()
        .filter(move |cur| {
            println!("char {:?}, state: {:?}", cur, parser.state);
            // state machine -.-
            match parser.state {
                ParserState::Text if *cur == '\x03' => { parser.state = ParserState::ColorCode; false }
                ParserState::Text => !(*cur == '\x02' ||  *cur == '\x1F' || *cur =='\x16' || *cur == '\x0F'),
                ParserState::ColorCode if  (*cur).is_digit(10) => {
                    parser.state = ParserState::Foreground1;
                    false
                },
                ParserState::Foreground1 if (*cur).is_digit(6) => {
                    parser.state = ParserState::Foreground2;
                    false
                },
                ParserState::Foreground1 if *cur == ','  => {
                    parser.state = ParserState::Comma;
                    false
                },
                ParserState::Foreground2 if *cur == ',' => {
                    parser.state = ParserState::Comma;
                    false
                },
                ParserState::Comma if ((*cur).is_digit(10)) => {

                    parser.state = ParserState::Background1;
                    false
                },
                ParserState::Background1 if (*cur).is_digit(6) => {
                    parser.state = ParserState::Text;
                    false
                }
                _ => true
            }

        })
        .collect();

    result
}
