/*
You need to return a string that looks like a diamond shape when printed on the screen, using asterisk (\*) characters.
Trailing spaces should be removed, and every line must be terminated with a newline character (\n).

Return null/nil/None/... if the input is an even number or negative, as it is not possible to print a diamond of even or negative size.

Examples
A size 3 diamond:
` *`
`***`
` *`
`...which would appear as a string of " *\n***\n *\n"`
*/

pub fn create_diamond(size: usize) -> String {
    let mut diamond = String::new();
    let mut whitespaces;

    // loop for the upper half of the diamond
    for x in 0..=size {
        if size == 1 {
            diamond.push_str(&"*".repeat(x));
            break;
        } else {
            if x % 2 != 0 {
                // draw whitespaces
                whitespaces = size - x;
                diamond.push_str(&" ".repeat(whitespaces / 2));

                // draw diamond
                diamond.push_str(&"*".repeat(x));

                // draw whitespaces
                diamond.push_str(&" ".repeat(whitespaces / 2));

                // jump line
                diamond.push('\n')
            }
        }
    }

    // loop for the other half of the diamond
    if size == 1 {
        diamond.push_str(&"*".repeat(size));
    } else {
        for y in (0..=size - 2).rev() {
            if y % 2 != 0 {
                // draw whitespaces
                whitespaces = size - y;

                // draw whitespaces
                diamond.push_str(&" ".repeat(whitespaces / 2));

                // draw diamond
                diamond.push_str(&"*".repeat(y));

                // draw whitespaces
                diamond.push_str(&" ".repeat(whitespaces / 2));

                // jump line
                diamond.push('\n')
            }
        }
    }

    diamond
}
