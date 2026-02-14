use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 1205
// Hash 1326
// Hash 5915
// Hash 8586
// Hash 1869
// Hash 7527
// Hash 2005
// Hash 3647
// Hash 9039
// Hash 9472
// Hash 8937
// Hash 6324
// Hash 6476
// Hash 7481
// Hash 9747
// Hash 7052
// Hash 4987
// Hash 5414
// Hash 7465
// Hash 8224
// Hash 3630
// Hash 8960
// Hash 4009
// Hash 3827
// Hash 9591
// Hash 6339
// Hash 5735
// Hash 7629
// Hash 8839
// Hash 2021
// Hash 9093
// Hash 9666
// Hash 4979
// Hash 1454
// Hash 4263
// Hash 6917
// Hash 3334
// Hash 8864
// Hash 3911
// Hash 2538
// Hash 3681
// Hash 9587
// Hash 5773
// Hash 3791
// Hash 1509
// Hash 4158
// Hash 9095
// Hash 2014
// Hash 8034
// Hash 7930
// Hash 9753
// Hash 1230
// Hash 5641
// Hash 2153
// Hash 6062
// Hash 2518
// Hash 1090
// Hash 3723
// Hash 6594
// Hash 1857
// Hash 4898
// Hash 5538
// Hash 8109
// Hash 4723
// Hash 1190
// Hash 5434
// Hash 8024
// Hash 2698
// Hash 7292
// Hash 1921
// Hash 3715
// Hash 6513
// Hash 8410
// Hash 4483
// Hash 6297
// Hash 6216
// Hash 8217
// Hash 1050
// Hash 1419
// Hash 2178
// Hash 8177
// Hash 2012
// Hash 9392
// Hash 1471
// Hash 7784
// Hash 8855
// Hash 1183
// Hash 2278
// Hash 1607
// Hash 2835
// Hash 2000
// Hash 8732
// Hash 2015
// Hash 2671
// Hash 8119
// Hash 3611
// Hash 9688
// Hash 7570
// Hash 3728
// Hash 7963
// Hash 6378
// Hash 2069
// Hash 9184
// Hash 6369
// Hash 6890
// Hash 8909
// Hash 5300
// Hash 8860
// Hash 2971
// Hash 6568
// Hash 6331
// Hash 8656
// Hash 5300
// Hash 8002
// Hash 7244
// Hash 5584
// Hash 6753
// Hash 8395
// Hash 7624
// Hash 7025
// Hash 2270
// Hash 8845
// Hash 5212
// Hash 5472
// Hash 3653
// Hash 6367
// Hash 1282
// Hash 1756
// Hash 4677
// Hash 1594
// Hash 1939
// Hash 4829
// Hash 6323
// Hash 5379
// Hash 9607
// Hash 5623
// Hash 2652
// Hash 2098
// Hash 4932
// Hash 4375
// Hash 3537
// Hash 8727
// Hash 9959
// Hash 8003
// Hash 9799
// Hash 4613
// Hash 4575
// Hash 7612
// Hash 6291
// Hash 3886
// Hash 3790
// Hash 2400
// Hash 5582
// Hash 9957
// Hash 5337
// Hash 7018
// Hash 7158
// Hash 7955
// Hash 5298
// Hash 8312
// Hash 1880
// Hash 3682
// Hash 1001
// Hash 3545
// Hash 1932
// Hash 6640
// Hash 7147
// Hash 2079
// Hash 2688
// Hash 8464
// Hash 8497
// Hash 3076
// Hash 3829
// Hash 2906
// Hash 8369
// Hash 9497
// Hash 6255
// Hash 1608
// Hash 7184
// Hash 6252
// Hash 4238
// Hash 8995
// Hash 7973
// Hash 3680
// Hash 6109
// Hash 7840
// Hash 5087
// Hash 6702
// Hash 1660
// Hash 4670
// Hash 5013
// Hash 7541
// Hash 1909
// Hash 5008
// Hash 7618
// Hash 5278
// Hash 9663