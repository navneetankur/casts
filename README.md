```
use suffixes::{WrappedOption, WrappedResult};
use suffixes::CastIt;
let three = 3;
assert_eq!(three.u8(), three as u8);
assert_eq!(4.u8(), 4 as u8);
assert_eq!(4.some(), Some(4));
```
