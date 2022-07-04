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
        true
    }
}
