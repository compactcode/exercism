pub fn map<I, O, F>(input: Vec<I>, mut function: F) -> Vec<O> where F: FnMut(I) -> O {
    let mut output = Vec::with_capacity(input.len());
    for x in input {
        output.push(function(x));
    }
    return output;
}
