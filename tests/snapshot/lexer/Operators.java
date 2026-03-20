// Operator tokens stress test for Java lexer

class OperatorsTest {

    void test() {

        int a = 1, b = 2, c = 3;
        boolean t = true, f = false;

        // --- Assignment ---
        a = b;

        // --- Relational ---
        boolean r1 = a > b;
        boolean r2 = a < b;
        boolean r3 = a >= b;
        boolean r4 = a <= b;

        // --- Equality ---
        boolean e1 = a == b;
        boolean e2 = a != b;

        // --- Logical ---
        boolean l1 = t && f;
        boolean l2 = t || f;

        // --- Bitwise ---
        int bw1 = a & b;
        int bw2 = a | b;
        int bw3 = a ^ b;
        int bw4 = ~a;

        // --- Arithmetic ---
        int ar1 = a + b;
        int ar2 = a - b;
        int ar3 = a * b;
        int ar4 = a / b;
        int ar5 = a % b;

        // --- Unary ---
        int u1 = +a;
        int u2 = -a;
        int u3 = ++a;
        int u4 = --a;
        int u5 = a++;
        int u6 = a--;

        // --- Shift ---
        int s1 = a << 1;
        int s2 = a >> 1;
        int s3 = a >>> 1;

        // --- Compound assignment ---
        a += b;
        a -= b;
        a *= b;
        a /= b;
        a &= b;
        a |= b;
        a ^= b;
        a %= b;
        a <<= 1;
        a >>= 1;
        a >>>= 1;

        // --- Ternary ---
        int t1 = t ? a : b;

        // --- Lambda ---
        Runnable r = () -> {};

        // --- Glued operator cases (no whitespace) ---
        int g1 = a+b*c;
        boolean g2 = a>=b&&t||f;
        int g3 = a<<1>>2>>>3;
        a+=b-=c*=a/=b;

        // --- Critical longest-match cases ---
        int m1 = a>>>1;     // must be >>>, not >> + >
        int m2 = a>>>=1;    // must be >>>=, not >>> + =
        int m3 = a<<=1;     // <<=, not << + =
        int m4 = a==b;      // ==, not = =
        int m5 = a!=b;      // !=, not ! =
        boolean m6 = t&&f;  // &&, not & &
        boolean m7 = t||f;  // ||, not | |

        // --- Mixed adjacency ---
        int x = (a+b)*c;
        boolean y = (a<b)&&(b>c)||!t;

        // --- Colon vs ternary vs lambda ---
        int z = t? a:b;
    }
}