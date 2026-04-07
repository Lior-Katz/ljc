public class Expressions {
    void literal_expressions() {
        int n = 5;
        long l = 10L;
        boolean b = true;
        b = false;
        char c = 'A';
        String s = "abc";
        Object o = null;
    }

    void unary_operators() {
         int x = 0;
         x = x++;
         x = x--;
         x = ++x;
         x = --x;
         x = ~100;
         boolean b = !!false;
         x = +3;
         x = -3;
    }

    void binary_operators() {
        // =======================
        // Associativity tests
        // =======================

        // Multiplicative
        int x = 8 / 4 / 2;
        x = 8 * 4 * 2;
        x = 8 % 4 % 2;
        x = 8 * 4 / 2 % 3;

        // Additive
        x = 5 - 3 - 1;
        x = 5 + 3 + 1;
        x = 5 + 3 - 1;

        // Shift
        x = 16 << 2 << 1;
        x = 16 >> 2 >> 1;
        x = 16 >>> 2 >>> 1;
        x = 16 << 2 >> 1;

        // Relational
        boolean b = 5 < 10;
        b = 5 > 10;
        b = 5 <= 10;
        b = 5 >= 10;

        // Equality
        b = 5 == 5;
        b = 5 != 5;

        // Bitwise
        x = 5 & 3 & 1;
        x = 5 ^ 3 ^ 1;
        x = 5 | 3 | 1;

        // Logical
        b = true && false && true;
        b = true || false || true;


        // =======================
        // Adjacent precedence
        // =======================

        // Multiplicative vs Additive
        x = 2 + 3 * 4;
        x = 2 * 3 + 4;

        // Additive vs Shift
        x = 2 + 3 << 1;
        x = 2 << 3 + 1;

        // Shift vs Relational
        b = 8 << 1 < 20;
        b = 8 < 1 << 4;

        // Relational vs Equality
        b = 5 < 10 == true;

        // Equality vs Bitwise AND
        b = 5 == 5 & true;
        b = false & 5 == 1;

        // Bitwise AND vs XOR
        x = 5 & 3 ^ 1;
        x = 5 ^ 3 & 1;

        // XOR vs OR
        x = 5 ^ 3 | 1;
        x = 5 | 3 ^ 1;

        // Bitwise OR vs Logical AND
        b = true | false && true;

        // Logical AND vs OR
        b = true && false || true;
        b = true || false && true;


        // =======================
        // Multi-level chains
        // =======================

        x = 2 + 3 * 4 << 1;
        x = 8 << 2 + 1 * 3;
        b = 5 < 10 && 3 + 4 > 6;
        b = 5 == 5 || 3 * 2 < 4;


        // =======================
        // Unary interaction
        // =======================

        x = -5 * 3;
        x = 5 * -3;
        x = -5 + -3;
        x = 5 + -3 * 2;


        // =======================
        // Parentheses override
        // =======================

        x = (2 + 3) * 4;
        x = 2 + (3 * 4);
        x = (2 << 3) + 1;
        x = 2 << (3 + 1);
    }

    void conditional_expressions() {
        int x = true ? 1 : 2;
        int a = false ? 1 : 2;

        // should parse as: true ? 1 : (false ? 2 : 3)
        int b = true ? 1 : false ? 2 : 3;

        // should parse as: true ? (false ? 1 : 2) : 3
        int c = true ? false ? 1 : 2 : 3;

        // condition is a logical expression
        int d = 1 < 2 ? 3 : 4;
        int e = true && false ? 1 : 2;
        int f = true || false ? 1 : 2;

        // expect: true ? 2 : (3 + 1)
        int g = true ? 2 : 3 + 1;

        // expect: (a < b) ? c : d + e
        x = a < b ? c : d + e;

        // expect: (a < b) ? c + d : e
        x = a < b ? c + d : e;

        x = a + b > c ? d * e : f / g;

        x = true ? a + b * c : d << e;

        // right-heavy nesting
        // expect a ? b : (c ? d : (e ? f :g))
        boolean b1 = false, b2 = false, b3 = false;
        x = b1 ? b : b2 ? d : b3 ? f : g;

        // mixed nesting
        // expect a ? (b ? c : d) : (e ? f : g)
        x = b1 ? b2 ? c : d : b3 ? f : g;

        x = -1 > 0 ? -2 : -3;
        x = true ? -a : -b;
    }
}