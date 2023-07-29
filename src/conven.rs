const RESET_COLOR: &str = "\x1B[0m";
const WHITE: &str = "\x1B[38;2;255;255;255m";
const YELLOW: &str = "\x1B[38;2;255;255;0m";
const GREEN: &str = "\x1B[38;2;0;255;0m";
const LIGHT_BLUE: &str = "\x1B[38;2;128;128;255m";

pub fn feed_item_str(
    title: Option<&str>,
    desc: Option<&str>,
    author: Option<&str>,
    link: Option<&str>,
) -> String {
    let title =
        format!("{WHITE}{}{RESET_COLOR}\n", title.unwrap_or("<Untitled>"));

    let desc = format!("  {YELLOW}Description:{RESET_COLOR}\n")
        + &format!("{}\n", desc.unwrap_or("None"));

    let author = format!("  {GREEN}Author:{RESET_COLOR}\n")
        + &format!("{}\n", author.unwrap_or("None"));

    let link = format!("  {LIGHT_BLUE}Link:{RESET_COLOR}\n")
        + &format!("{LIGHT_BLUE}{}{RESET_COLOR}\n", link.unwrap_or("None"));

    title + &desc + &author + &link
}
