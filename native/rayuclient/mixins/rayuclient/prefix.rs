// Prefix the final displayed name line on each visible name tag.
fn prefix_name_tags(mut tags: Vec<NameTag>) -> Vec<NameTag> {
    for tag in &mut tags {
        if let Some((name, _lift)) = tag.lines.last_mut() {
            let original = std::mem::replace(name, Text::empty());
            let mut prefixed = Text::literal("★ ");
            prefixed.push(original);
            *name = prefixed;
        }
    }
    tags
}
