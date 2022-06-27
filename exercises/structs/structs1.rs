// structs1.rs
// Address all the TODOs to make the tests pass!

// I AM NOT DONE

#[derive(PartialEq)]
struct Color<'a> {
    name: &'a str,
    hex: &'a str,
}

struct ColorClassicStruct<'a> {
    red: Color<'a>,
    green: Color<'a>,
    blue: Color<'a>,
}

#[derive(Debug, PartialEq)]
struct ColorTupleStruct<'a>((&'a str, &'a str), (&'a str, &'a str), (&'a str, &'a str));

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: Color {
                name: "red",
                hex: "#FF0000",
            },
            green: Color {
                name: "green",
                hex: "#00FF00",
            },
            blue: Color {
                name: "blue",
                hex: "#0000FF",
            },
        }
        .green;

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(
            ("red", "#FF0000"),
            ("green", "#00FF00"),
            ("blue", "#0000FF"),
        )
        .1;

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
