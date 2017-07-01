// Returns a UTC DateTime one billion seconds after start.
pub fn is_leap_year(year: usize) -> bool {
    if year % 400 == 0 {
        return true
    }
    if year % 100 == 0 {
        return false
    }
    year % 4 == 0
}
