// raw
// Program: 2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0

// opcodes decoded
// bst 4
// bxl 7
// cdv 5
// bxl 7
// bxc 6
// adv 3
// out 5
// jnz 0

// operands decoded
// bst %a
// bxl $7
// cdv %b
// bxl $7
// bxc
// adv $3
// out %b
// jnz 0

// decompiled
while (a != 0) {
    b = a % 8;
    b ^= 0b111;
    c = a / pow(2, b);
    b ^= 0b111;
    b ^= c;
    a = a / pow(2, 3);
    print(b);
}

// simplified
while (a != 0) {
    b = ~(a & 0b111);
    b ^= a >> b;
    a = a >> 3;
    print(b);
}

// inlined
while (a != 0) {
    b = ~(a & 0b111) ^ (a >> ~(a & 0b111));
    a = a >> 3;
    print(b);
}

// so term n is
b_n = ~((a >> (3 * n)) & 0b111) ^ ((a >> (3 * n)) >> ~((a >> (3 * n)) & 0b111));

// need:
// b_1 = 2
// b_2 = 4
// b_3 = 1
// b_4 = 7
// b_5 = 7
// b_6 = 5
// b_7 = 1
// b_8 = 7
// b_9 = 4
// b_10 = 6
// b_11 = 0
// b_12 = 3
// b_13 = 5
// b_14 = 5
// b_15 = 3
// b_16 = 0

// construct terms starting from the end
// recall:
while (a != 0) {
    b = ~(a & 0b111);
    b ^= a >> b;
    a = a >> 3;
    print(b);
}
// actually it might be easier to simulate this to find the proper b for each
// step... that takes 8 ^ 16 = 2 ^ 48 steps still so no actually
