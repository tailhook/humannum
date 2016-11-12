=========
Human Num
=========

:Status: beta
:Documentation: http://tailhook.github.io/humannum/

Human-friendly numeric parser.

Supported features:

* Supports for underscore in numbers `1_000` → 1000
* ``k``, ``M``, ``G`` -- decimal suffixes ``25M`` → 25000000
* ``ki``, ``Mi``, ``Gi`` -- binary suffixes ``8Ki`` → 8192
* ``0x``, ``0o``, ``0b`` -- prefixes numeral systems
  ``0xFF`` → 255, ``0o40`` → 32, ``0b11`` → 3
* Wrapper type that implements ``FromStr``


=======
License
=======

Licensed under either of

* Apache License, Version 2.0, (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

------------
Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
