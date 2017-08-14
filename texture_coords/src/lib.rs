extern crate serde;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CoordinateMap {
    pub map: HashMap<String, [f64; 4]>,
}
