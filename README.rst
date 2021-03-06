
defmac
======

Please read the `API documentation here`__

__ https://docs.rs/defmac

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/defmac.svg
.. _build_status: https://travis-ci.org/bluss/defmac

.. |crates| image:: http://meritbadge.herokuapp.com/defmac
.. _crates: https://crates.io/crates/defmac

Recent Changes
--------------

- 0.2.1

  - Fix so that the macro can be used through its full path (like
    `defmac::defmac! { .. }`
  - Fix so that the macro can expand at module level

- 0.2.0

  - New implementation that is general and variadic: defmac now supports
    making macros with arbitrarily many parameters!
  - Requires Rust 1.20

- 0.1.3

  - Update docs with another example and a tip about syntactical variable
    capture.


License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


