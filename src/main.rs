use alloy::primitives::{Address, U256};

fn main() {
    let mut contract = Contract::default();
     contract.approve_token(
            "C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // weth
            "9FC3da866e7DF3a1c57adE1a97c9f00a70f010c8", // some randos addy
            "3635C9ADC5DEA00000", // 1,000 tokens
        );

    println!("\n[Calldata]\n{}", contract.calldata);
}


struct Contract {
    calldata: String,
    calldata_offsets: Vec<usize>,
    source: String,
}

impl Contract {
    pub const PUSH1: &'static str = "60";
    pub const PUSH20: &'static str = "73";
    pub const GAS: &'static str = "5A";
    pub const CALL: &'static str = "F1";
    pub const POP: &'static str = "50";
    pub const CALLDATALOAD: &'static str = "35";
    pub const MSTORE: &'static str = "52";
    pub const SHR: &'static str = "1C";


    pub const APPROVE_SIG: &'static str = "095ea7b3";

    pub fn default() -> Self {
        Self {
            calldata: String::new(),
            calldata_offsets: vec![],
            source: String::new(),
        }
    }
        pub fn extend_calldata(&mut self, new_calldata: Vec<&str>) {
        println!("\n[Extending Calldata: {}]", new_calldata.len());
        println!("- [00] Old: {}", &self.calldata);

        for (i, item) in new_calldata.iter().enumerate() {

                // Add it onto the end of our existing calldata
            self.calldata.extend([*item]);

                    // Record the offset of where we just added so we can ref it later
            match self.calldata_offsets.is_empty() {
                true => {
                        // We don't init with anything since we wont have calldata (duh)
                        // So we add it here to initialise + our own calldata
                    self.calldata_offsets.push(0);
                    // Since we're dealing with strings it'll be double the
                    // len -- we want the amount of bytes instead so 1/2 it
                    self.calldata_offsets.push(self.calldata.len() / 2);
                }
                false => self.calldata_offsets.push(self.calldata.len() / 2),
            }

            // println!("self.calldata_offsets {:?}", &self.calldata_offsets);
            println!("- [{:02x}] New: {}", i + 1, &self.calldata);
        }
    }

    pub fn approve_token(&mut self, token: &str, to: &str, amount: &str) {

        let latest_offset = *self.calldata_offsets.last().unwrap_or(&0);
        let calldata_len: usize = (Self::APPROVE_SIG.len() + to.len() + amount.len()) / 2;
        self.extend_calldata(vec![Self::APPROVE_SIG, to, amount]);





    }





}

