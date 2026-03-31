// Boolean literal operator-boundary stress test for Java lexer

class BooleanLiteralsOperatorsTest {

    void test() {
        // --- Valid boolean literals ---
        boolean t1 = true;
        boolean t2 = false;

        // --- Glued to unary operators ---
        boolean u1 = !true;
        boolean u2 = !false;

        // --- Glued to binary operators ---
        boolean b1 = true&&false;

        // --- Mixed spacing (should tokenize the same) ---
        boolean m1 = true&& false;

        // --- With parentheses ---
        boolean p1 = (true);

        // --- With other operators ---
        boolean o1 = true^false;     // XOR

        // --- Adjacent to punctuation ---
        if(true){}
        if(false){}
        if(true&&false){}

        // --- Used in expressions ---
        boolean e1 = true && false;
        boolean e3 = (false || true);

        // --- Identifiers that look similar (must NOT be literals) ---
        boolean trueValue = false;
        boolean falseValue = true;
        boolean TRUE = false;     // case-sensitive: not a literal
        boolean False = true;     // not a literal
        boolean c1 = truefalse;   // single identifier
        boolean c2 = falsetrue;   // single identifier
        boolean c3 = true_false;  // identifier
        boolean c4 = true1;       // identifier

        // --- Separation by operators only ---
        boolean s1 = true+false;  // '+' should break tokens
        boolean s2 = true- false;

        // Note: '+' and '-' are invalid with booleans semantically,
        // but lexer must still tokenize correctly.

    }
}