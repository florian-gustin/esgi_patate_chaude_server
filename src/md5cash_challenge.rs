use crate::challenge::Challenge;
use crate::challenge_message::{MD5HashCashInput, MD5HashCashOutput};

pub(crate) struct HashCash {
    input: MD5HashCashInput,
}

impl Challenge for HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    fn new(input: Self::Input) -> HashCash {
        HashCash {
            input
        }
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: Self::Output) -> bool {
        let hex = format!("{:016X}", answer.seed) + &self.input.message;
        let hash = format!("{:016X}", md5::compute(hex));

        if hash != answer.hashcode {
            return false;
        }

        let binary_seed = u128::from_str_radix(&hash.to_string(), 16).unwrap();
        if binary_seed.leading_zeros() != self.input.complexity {
            return false;
        }
        return true;
    }
}
