use crate::getter;

pub struct Stash {
    id: String,
    created: String,
    subject: String,
}

impl Stash {
    getter!(id, String);
    getter!(created, String);
    getter!(subject, String);
}

/// only work with ouput from `git --no-pager stash list --pretty='format:%gd%x1f%cr%x1f%s%x1e'`
fn parse_stashes(input: &str) -> Vec<Stash> {
    let mut stashes = Vec::new();

    for record in input.split('\x1e') {
        if record.is_empty() {
            continue;
        }
        let mut data = record.split('\x1f');

        let Some(id) = data.next() else {
            continue;
        };
        let id = id.trim().to_string();

        let Some(created) = data.next() else {
            continue;
        };
        let created = created.trim().to_string();

        let Some(subject) = data.next() else {
            continue;
        };
        let subject = subject.trim().to_string();

        let stash = Stash {
            id,
            created,
            subject,
        };

        stashes.push(stash);
    }

    stashes
}

#[cfg(test)]
mod tests {}
