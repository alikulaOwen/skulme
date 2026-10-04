//Rust implementation of the ReLU (rectified linear unit) activation function.
//The formula for ReLU is quite simple really: (if x>0 -> x, else -> 0)
//More information on the concepts of ReLU can be found here:
//https://en.wikipedia.org/wiki/Rectifier_(neural_networks)

//The function below takes a reference to a mutable <f32> Vector as an argument
//and returns the vector with 'ReLU' applied to all values.
//Of course, these functions can be changed by the developer so that the input vector isn't manipulated.
//This is simply an implemenation of the formula.

pub fn relu(array: &mut Vec<f32>) -> &mut Vec<f32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (relu)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement relu");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        let mut test: Vec<f32> = Vec::from([1.0, 0.5, -1.0, 0.0, 0.3]);
        assert_eq!(
            relu(&mut test),
            &mut Vec::<f32>::from([1.0, 0.5, 0.0, 0.0, 0.3])
        );
    }
}
