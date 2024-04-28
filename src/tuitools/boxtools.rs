



pub fn simplebox<'a, Opt>(_title:&str, text:String, opt:Opt) -> String 
where 
    Opt:Into<textwrap::Options<'a>>,
{
    let mut data = String::from("╭─").to_owned();
    let options:textwrap::Options<'a> = opt.into();
    /* We need to know what the width of the object is going to be... */
    
    /* Unoptimized for now... */
    /* Takes care of borders spaces and stuff */
    let top_lines_needed = (options.width + 4) - (_title.len() + 2);
    assert!(top_lines_needed > 3);
    
    data.push(' ');
    data.push_str(_title);
    data.push(' ');
    data.push_str("─".repeat(top_lines_needed - 3).as_str());
    data.push_str("╮\n");

    for x in textwrap::wrap(text.as_str(), options.width) {
        data.push_str("│ ");
        data.push_str(&x);
        if x.len() < options.width{
            data.push_str(" ".repeat(options.width - x.len()).as_str());
        }
        data.push_str(" │\n");
    }

    data.push('╰');
    data.push_str("─".repeat(options.width + 2).as_str());
    data.push('╯');
    data 
}




