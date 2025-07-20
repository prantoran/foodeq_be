
```bash
cargo watch -q -c -w src/ -x run
```

```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
sample output:
```bash
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test

=== Response for GET http://localhost:3000/hello
=> Status         : 200 OK
=> Headers        :
   content-type: text/html; charset=utf-8
   content-length: 31
   date: Sun, 20 Jul 2025 07:03:24 GMT
=> Response Body  :
Hello <strong>World!!!</strong>
===

.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
```