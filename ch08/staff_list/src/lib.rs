use std::collections::{BTreeMap, BTreeSet};

pub fn usage() {
    let command_lines = [
        "usage | help",
        "add <staff> to <department>",
        "list <department>",
        "list",
    ];

    eprintln!("{}", command_lines.join("\n"));
}

pub fn add(
    staff_list: &mut BTreeMap<String, BTreeSet<String>>,
    command_args: &mut dyn Iterator<Item = &str>,
) {
    let staff = match command_args.next() {
        None => return usage(),
        Some(arg) => arg.to_string(),
    };

    if let Some("to") = command_args.next() {
    } else {
        return usage();
    };

    let department = match command_args.next() {
        None => return usage(),
        Some(arg) => arg.to_string(),
    };

    staff_list.entry(department).or_default().insert(staff);
}

pub fn list(
    staff_list: &BTreeMap<String, BTreeSet<String>>,
    command_args: &mut dyn Iterator<Item = &str>,
) {
    let department = match command_args.next() {
        None => return list_all(staff_list),
        Some(arg) => arg,
    };

    let staff = match staff_list.get(department) {
        None => return eprintln!("the department not exists"),
        Some(xs) => xs,
    };

    println!(
        "[{}]\n{}",
        department,
        staff.to_owned().into_iter().collect::<Vec<_>>().join("\n")
    );
}

pub fn list_all(staff_list: &BTreeMap<String, BTreeSet<String>>) {
    if staff_list.is_empty() {
        return println!("empty");
    }

    println!(
        "{}",
        staff_list
            .iter()
            .map(|(department, staff)| format!(
                "[{}]\n{}",
                department,
                staff.to_owned().into_iter().collect::<Vec<_>>().join("\n")
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
