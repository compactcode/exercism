pub fn build_proverb(list: Vec<&str>) -> String {
   let mut results: Vec<String> = list
       .windows(2)
       .map(|pair| format!("For want of a {} the {} was lost.", pair[0], pair[1]))
       .collect();

   match list.first() {
       Some(first) => results.push(format!("And all for the want of a {}.", first)),
       None        => ()
   }

   return results.join("\n");
}
