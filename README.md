# zfx-sortition

Cryptograhic sortition library in Rust.

## Building

`cargo build`

## Tests

The unittests contain a non-deterministic testcase with random inputs. It is supposed to check if out of 1k runs the average output of the selection is around 10. This might result in rare failures. Deviance allowed is increased to around ~3% from the original 2%, as it seems Rust has a random generator that produces more deviance. This has been verified by saving the input of some of the high-deviance runs, and testing them against the Go implementation.

Run unit tests with the following command:

`cargo test`

Perform benchmark tests with the following command:

`cargo bench`
