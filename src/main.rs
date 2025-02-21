use std::env;
use std::io;

use dlx::config::*;
use dlx::DancingLinks;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args.as_slice()).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    if config.is_help() {
        todo!("help menu");
    }

    let mut item_buffer = String::new();

    let (primary_items, secondary_items) = loop {
        io::stdin()
            .read_line(&mut item_buffer)
            .expect("Failed to read line.");

        if item_buffer.trim().is_empty() {
            continue;
        }

        if !item_buffer.is_ascii() {
            panic!("Item names should belong to ASCII range.");
        }

        if item_buffer.matches('|').count() > 1 {
            panic!("Item type separator \'|\' can only appear once.");
        }

        let mut items = item_buffer.split('|');

        let primary: Vec<&str> = items.next().unwrap().split_whitespace().collect();

        let secondary: Vec<&str> = match items.next() {
            Some(s) => s.split_whitespace().collect(),
            None => Vec::new(),
        };

        if primary.is_empty() {
            panic!("Primary items are required.");
        }

        break (primary, secondary);
    };

    let mut dlx = DancingLinks::new(
        primary_items.as_slice(), secondary_items.as_slice()
    );

    let mut option_buffer = String::new();

    loop {
        let read_bytes = io::stdin()
            .read_line(&mut option_buffer)
            .expect("Failed to read line.");

        if read_bytes == 0 {
            break;
        }

        if option_buffer.trim().is_empty() {
            option_buffer.clear();
            continue;
        }

        dlx.add_option(&option_buffer);

        option_buffer.clear();
    }

    eprintln!(
        "Read {}+{}={} items and {} options.",
        dlx.get_primary(),
        dlx.get_secondary(),
        dlx.get_item_count(),
        dlx.get_option_count(),
    );

    let (solution_count, elapsed_time, visited_nodes) = dlx.dance(&config);

    if solution_count == 1 {
        println!(
            "Found {} solution in {:?} visiting {} nodes.",
            solution_count, elapsed_time, visited_nodes,
        );
    } else {
        println!(
            "Found {} solutions in {:.5?} visiting {} nodes.",
            solution_count, elapsed_time, visited_nodes,
        );
    }
}
