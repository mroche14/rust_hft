// Implements delta encoding for columns that request it. 
// For simplicity, we assume f64 values. We store the delta of the new value from the previous value.

pub fn delta_encode_column(previous_val: f64, new_val: f64) -> f64 {
    // Delta encoding: store the difference from previous value
    new_val - previous_val
}
