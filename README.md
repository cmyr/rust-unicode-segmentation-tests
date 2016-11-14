### test cases around unicode word segmentation.

This crate auto-generates tests cases from the unicode word break sample tests from [http://www.unicode.org/Public/8.0.0/ucd/auxiliary/WordBreakTest.txt]. 

By default it will build the tests from from Unicode 8. The Unicode 9 tests can be opted into by passing `--features "unicode9"` to `cargo test`.
