# Function Signature Calculator

This is for generating a 4 byte solidity function signature.

## How to use

```
git clone git@github.com:tommyrharper/function-sig-calculator.git
cd function-sig-calculator
cargo run "getAnotherValue()" // should output 0x25d805cf to terminal
cargo build
// now you can run it using the binary
<PATH_TO_THIS_REPO>/target/debug/function-sig-calc "getAnotherValue()"
```

I then added the following alias to zsh config:
```
alias sig='<PATH_TO_THIS_REPO>/target/debug/function-sig-calc'
```

Then I can do this in my terminal:
```
sig "someFunction(uint256)" // this will output 0x7062c094
```
