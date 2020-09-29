pub mod majin {

    pub type Sexpr = Vec<Elements>;

    pub enum Elements {
        Op(Op),
        Nat(u32),
        Atom(String),
    }

    pub enum Op {
        Add,
        Sub,
        Mul,
        Div
    }

    pub(self) mod parser {
        use super::Op;
        use super::Elements;
        use super::Sexpr;
        use nom::{
            branch::alt,
            bytes::complete::{tag},
            Err,
            error::{Error, ErrorKind},
            character::complete::{multispace0, alphanumeric1, line_ending},
            bytes::complete::{take_while},
            sequence::{preceded, delimited, terminated},
            multi::{many0},
            IResult,
        };

        fn parse_operator(i: &str) -> IResult<&str, &str> {
            alt((
                tag("+"),
                tag("-"),
                tag("*"),
                tag("/"),
            ))(i)
        }

        fn parse_value(i: &str) -> IResult<&str, &str> {
            preceded(multispace0, alphanumeric1)(i)
        }

        fn parse_sexpr(i: &str) -> IResult<&str, Vec<&str>> {
            delimited(
                tag("("),
                many0(parse_value),
                preceded(multispace0, tag(")"))
            )(i)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_parse_operator() {
                assert_eq!( parse_operator("*"), Ok(("", "*")));
                assert_eq!( parse_operator("-"), Ok(("", "-")));
                assert_eq!( parse_operator("+"), Ok(("", "+")));
                assert_eq!( parse_operator("/"), Ok(("", "/")));
                assert_eq!( parse_operator(")"), Err(Err::Error(Error::new(")", ErrorKind::Tag))));
            }

            #[test]
            fn test_parse_value() {
                assert_eq!( parse_value("applesauce"), Ok(("", "applesauce")));
                assert_eq!( parse_value(" \napplesauce"), Ok(("", "applesauce")));
                assert_eq!( parse_value(" \n"), Err(Err::Error(Error::new("", ErrorKind::AlphaNumeric))));
            }

            #[test]
            fn test_parse_sexpr() {
                assert_eq!( parse_sexpr("()"), Ok(("", vec![])));
                assert_eq!( parse_sexpr("(applesauce)"), Ok(("", vec!["applesauce"])));
                assert_eq!( parse_sexpr("(applesauce bananas )"), Ok(("", vec!["applesauce", "bananas"])));
            }
        }
    }
}
