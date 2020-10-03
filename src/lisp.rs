pub mod majin {

    #[derive(Debug, PartialEq)]
    enum Element {
        Sexpr(Vec<Element>),
        Scalar(Scalar),
    }

    #[derive(Debug, PartialEq)]
    enum Scalar {
        Atom(String),
        Nat(u32),
    }

    pub(self) mod parser {
        use super::Element;
        use super::Scalar;

        use nom::{
            branch::alt,
            bytes::complete::{is_not, tag, take_while},
            character::complete::{alphanumeric1, digit1, multispace0, one_of},
            character::is_digit,
            combinator::{all_consuming, map, peek, rest},
            error::{Error, ErrorKind},
            multi::many0,
            sequence::{delimited, preceded, terminated},
            Err, IResult,
        };

        fn parse_nat(i: &str) -> IResult<&str, Scalar> {
            map(
                digit1,
                |c: &str| Scalar::Nat(c.parse::<u32>().unwrap())
            )(i)
        }

        fn parse_atom(i: &str) -> IResult<&str, Scalar> {
            map(
                alphanumeric1,
                |s: &str| Scalar::Atom(s.to_string())
            )(i)
        }

        fn parse_scalar(i: &str) -> IResult<&str, Element> {
            map(
                preceded(
                    multispace0,
                    alt((parse_nat, parse_atom)),
                ),
                |s| Element::Scalar(s)
            )(i)
        }

        fn parse_element(i: &str) -> IResult<&str, Element> {
            alt((parse_scalar, parse_sexpr))(i)
        }

        fn parse_sexpr(i: &str) -> IResult<&str, Element> {
            map(delimited(
                preceded(multispace0, tag("(")),
                many0(parse_element),
                preceded(multispace0, tag(")"))
            ), |v| Element::Sexpr(v) )(i)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_parse_nat() {
                assert_eq!(parse_nat("12 "), Ok((" ", Scalar::Nat(12))));
                // assert_eq!(parse_nat("12e"), Ok(("", Err(Err::Error(Error::new("e", ErrorKind::Eof))));
            }

            #[test]
            fn test_parse_atom() {
                assert_eq!(
                    parse_atom("atom"),
                    Ok(("", Scalar::Atom("atom".to_string())))
                );
            }

            #[test]
            fn test_parse_scalar() {
                assert_eq!(
                    parse_scalar("12"),
                    Ok(("", Element::Scalar(Scalar::Nat(12))))
                );

                assert_eq!(
                    parse_scalar("atom1 )"),
                    Ok((" )", Element::Scalar(Scalar::Atom(String::from("atom1")))))
                );

                assert_eq!(
                    parse_scalar("atom1 ("),
                    Ok((" (", Element::Scalar(Scalar::Atom(String::from("atom1")))))
                );

                assert_eq!(
                    terminated(parse_scalar, multispace0)("atom1 ("),
                    Ok(("(", Element::Scalar(Scalar::Atom(String::from("atom1")))))
                );

                assert_eq!(
                    preceded(tag("("), parse_scalar)("( atom1 )"),
                    Ok((" )", Element::Scalar(Scalar::Atom(String::from("atom1")))))
                );
            }

            #[test]
            fn test_parse_element() {
                assert_eq!(
                    parse_element("atom1"),
                    Ok(( "", Element::Scalar(Scalar::Atom(String::from("atom1")))) )
                );

                assert_eq!(
                    parse_element("atom1 )"),
                    Ok(( " )", Element::Scalar(Scalar::Atom(String::from("atom1"))) ))
                );

                assert_eq!(
                    parse_element("(atom1)"),
                    Ok(( "", Element::Sexpr(
                        vec![Element::Scalar(Scalar::Atom(String::from("atom1")))]
                    ) ))
                );

                assert_eq!(
                    parse_element("( atom1 atom2 )"),
                    Ok(( "", Element::Sexpr(
                        vec![
                            Element::Scalar(Scalar::Atom(String::from("atom1"))),
                            Element::Scalar(Scalar::Atom(String::from("atom2")))]
                    ) ))
                );

                assert_eq!(
                    parse_element("( atom1 (atom2) )"),
                    Ok(( "", Element::Sexpr(
                        vec![
                            Element::Scalar(Scalar::Atom(String::from("atom1"))),
                            Element::Sexpr(
                                vec![
                                    Element::Scalar(Scalar::Atom(String::from("atom2")))
                                ]
                            )
                        ]
                    ) ))
                );

                assert_eq!(
                    parse_element("( atom1 ((atom2) ( atom3 atom4 )) )"),
                    Ok(( "", Element::Sexpr(
                        vec![
                            Element::Scalar(Scalar::Atom(String::from("atom1"))),
                            Element::Sexpr(
                                vec![
                                    Element::Sexpr(
                                        vec![
                                            Element::Scalar(Scalar::Atom(String::from("atom2")))
                                        ]
                                    ),
                                    Element::Sexpr(
                                        vec![
                                            Element::Scalar(Scalar::Atom(String::from("atom3"))),
                                            Element::Scalar(Scalar::Atom(String::from("atom4")))
                                        ]
                                    )
                                ]
                            )
                        ],
                    ) ))
                );
            }
        }
    }
}
