#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'a> {
    name: &'a str,
    relative_date: &'a str,
    subject: &'a str,
}

macro_rules! getter {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> &$type {
            &self.$name
        }
    };
}

impl<'a> Tag<'a> {
    getter!(name, &'a str);
    getter!(relative_date, &'a str);
    getter!(subject, &'a str);
}

/// input must be from the `git --no-pager tag -l --sort=-creatordate --format='%(refname:short)%1f%(creatordate:relative)%1f%(subject)%1e'` command
fn parse_git_tags<'a>(format_string: &'a str) -> Vec<Tag<'a>> {
    vec![]
}

#[cfg(test)]
mod test {}
