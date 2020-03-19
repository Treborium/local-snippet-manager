pub fn format_line(name: &str, command: &str, name_width: usize, command_width: usize) -> String {
    format!(
        "{f_name} {f_command}",
        f_name = limit_str(name, name_width),
        f_command = limit_str(command, command_width)
    )
}

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        // Check if the last character is a space
        if text[..length - 1].chars().rev().next().unwrap() == ' ' {
            format!("{}… ", &text[..length - 2])
        } else {
            format!("{}…", &text[..length - 1])
        }
    } else {
        format!("{:width$}", text, width = length)
    }
}
