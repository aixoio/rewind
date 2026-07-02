/// input must be from the `git --no-pager tag -l --sort=-creatordate --format='%(refname:short)%1f%(creatordate:relative)%1f%(subject)%1e'` command
fn parse_git_tags(format_string: &str) -> Vec<&str> {
    vec![]
}

#[cfg(test)]
mod test {}
