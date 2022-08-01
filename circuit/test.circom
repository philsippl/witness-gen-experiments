pragma circom 2.0.0;

template Ark(t) {
    signal input in[t];
    signal output out[t];

    for (var i=0; i<t; i++) {
        out[i] <== in[i] * 2;
    }
}


template Multiplier() {
    signal input a;
    signal input b;
    signal output c[10];

    component ark[1];
    ark[0] = Ark(10);
    for (var j=0; j<10; j++) {
        ark[0].in[j] <== a;
    }

    for (var j=0; j<10; j++) {
        c[j] <== ark[0].out[j];
    }
}

component main = Multiplier();