macro_rules! import_models {
    ($x:ident) => {
        mod $x;
        pub use self::$x::*;
    };
}

import_models!(package);
import_models!(worker);
import_models!(queue);
