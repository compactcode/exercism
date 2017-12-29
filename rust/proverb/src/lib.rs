use std::iter::once;

pub fn build_proverb(list: Vec<&str>) -> String {
   match list.first() {
       Some(first) => list
           .windows(2)
           .map(format_pair)
           .chain(once(format_end(first)))
           .collect::<Vec<String>>()
           .join("\n"),

       None => String::new()
   }
}

fn format_pair(pair: &[&str]) -> String {
   format!("For want of a {} the {} was lost.", pair[0], pair[1])
}

fn format_end(last: &str) -> String {
   format!("And all for the want of a {}.", last)
}
