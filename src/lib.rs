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
            if parser.state == ParserState::Text {
                if *cur == '\x03' {
                    parser.state = ParserState::ColorCode;
                    return false; // rm color code
                } else {
                    return !(*cur == '\x02' || // bold
                    *cur == '\x1F' || // underline
                    *cur =='\x16' || // reverse
                    *cur == '\x0F');
                }
            } else if parser.state == ParserState::ColorCode && (*cur).is_digit(10) {
                parser.state = ParserState::Foreground1;
                return false;
            } else if parser.state == ParserState::Foreground1 && (*cur).is_digit(6) {
                parser.state = ParserState::Foreground2;
                return false;
            } else if parser.state == ParserState::Foreground1 && *cur == ',' {
                parser.state = ParserState::Comma;
                return false;
            } else if parser.state == ParserState::Foreground2 && *cur == ',' {
                parser.state = ParserState::Comma;
                return false;
            } else if parser.state == ParserState::Comma && (*cur).is_digit(10) {
                parser.state = ParserState::Background1;
                return false;
            } else if parser.state == ParserState::Background1 && (*cur).is_digit(6) {
                parser.state = ParserState::Text;
                return false;
            }
            return true;
        })
        .collect();

    result
}
