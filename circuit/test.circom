pragma circom 2.0.0;

template Multiplier() {
    signal input a;
    signal input b;
    signal output c;

    c <== a*b + 1000000000000000000999999 + 1337 + 129 + 1;
}

component main = Multiplier();