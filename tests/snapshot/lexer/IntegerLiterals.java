// Integer literal stress test for Java lexer

class IntegerLiterals {

    void test() {
        // --- Decimal literals ---
        int d1 = 0;
        int d2 = 123;
        int d3 = 1_000_000;
        int d4 = 1__2__3;          // multiple underscores

        long d5 = 123L;
        long d6 = 123l;

        // --- Hexadecimal (base 16) ---
        int h1 = 0x0;
        int h2 = 0X1A;
        int h3 = 0xDEAD_BEEF;
        int h4 = 0xCAFE__BABE;    // multiple underscores

        long h5 = 0xFFL;
        long h6 = 0xaal

        // --- Binary (base 2) ---
        int b1 = 0b0;
        int b2 = 0B1010;
        int b3 = 0b1010_0101;
        int b4 = 0b10__10;

        long b5 = 0b1111L;
        long b6 = 0b0000l;

        // --- Octal (base 8) ---
        int o1 = 01;
        int o2 = 0777;
        int o3 = 0_7;             // underscore after leading 0 allowed
        int o4 = 0__77;

        long o5 = 0123L;

        // --- Boundary values ---
        int maxInt = 2147483647;
        long maxLong = 9223372036854775807L;

        // Special rule: allowed only with unary minus
        int minInt = -2147483648;
        long minLong = -9223372036854775808L;

        // Overflow without unary minus, should still be able to parse
        int e12 = 2147483648;           // too large for int
        long e13 = 9223372036854775808L; // too large for long

        // Unary minus separation (lexer should not merge!)
        int tricky1 = - 2147483648;   // '-' and literal must be separate tokens
        long tricky2 = - 9223372036854775808L;

        // --- Tokenization edge cases ---
        int mix1 = 0x1 + 0b10 + 07 + 9;
        int mix2 = 1_2_3 + 0xA_B + 0b1_0;

    }
}