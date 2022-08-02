pragma circom 2.0.0;

template Test() {
    signal input in;
    signal output out;

    out <== in * 2;
}

template Semaphore(nInputs) {
    signal input in;
    signal output out;

    component mims[nInputs];

    mims[0] = Test();
    mims[0].in <== in;
    out <== mims[0].out;
}

component main = Semaphore(5);


/*
INPUT = { "a": "1234" }
*/