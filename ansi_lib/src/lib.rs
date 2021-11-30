extern crate ansi_term;

#[cfg(test)]
mod tests {
    use ansi_term::{
        ANSIByteStrings, ANSIString, ANSIStrings,
        Colour::{self, Blue, Cyan, Fixed, Green, Red, Yellow, RGB},
        Style,
    };

    #[test]
    fn test_ansi_term() {
        println!(
            "this is {} in color, {} in color and {} in color, {} in underline",
            Red.paint("red"),
            Blue.paint("blue"),
            Green.bold().paint("green"),
            Yellow.underline().paint("yellow"),
        );

        println!(
            "{} and this is not",
            Style::new().bold().paint("This is Bold")
        );

        println!(
            "Yellow on blue : {}",
            Style::new().on(Blue).fg(Yellow).paint("yow!")
        );

        println!(
            "Also yellow on blue: {}",
            Cyan.on(Blue).fg(Yellow).paint("zow!")
        );

        println!("blink text is : {}", Blue.blink().paint("yaphets"));
        println!("reverse text is : {}", Blue.reverse().paint("yaphets"));
        println!(
            "style is : {}, extended colours: {}, fixed colours: {}",
            Style::new()
                .fg(Blue)
                .on(Colour::Purple)
                .strikethrough()
                .dimmed()
                .italic()
                .paint("special text"),
            RGB(70, 130, 120).paint("Steel blue"),
            Fixed(200).on(Fixed(124)).paint("Mustard in the ketchup")
        );

        let some_value = format!("{:b}", 42);
        let strings: &[ANSIString<'static>] =
            &[Red.paint("["), Red.bold().paint(some_value), Red.paint("]")];
        println!("Value: {}", ANSIStrings(strings));

        ANSIByteStrings(&[
            Green.paint("user data 1\n".as_bytes()),
            Green.bold().paint("user data 2\n".as_bytes()),
        ])
        .write_to(&mut std::io::stdout())
        .unwrap();
    }
}
