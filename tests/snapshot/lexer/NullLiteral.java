class NullLiteralTest {

    void test() {

        // --- Valid null literal ---
        Object n1 = null;

        // --- Used in expressions ---
        Object n2 = (null);
        boolean b1 = (null == null);
        boolean b2 = (null != null);

        // --- With operators (no whitespace) ---
        boolean o1 = null==null;
        boolean o2 = null!=null;

        // --- Adjacent to punctuation ---
        Object p1 = (null);
        if (null == null) {}

        // --- Identifier lookalikes (must NOT be null literal) ---
        Object nullValue = null;
        Object Null = null;      // identifier (case-sensitive)
        Object NULL = null;      // identifier

        // --- Token boundary checks ---
        Object t1 = nullnull;    // single identifier
        Object t2 = null_1;      // identifier
        Object t3 = null1;       // identifier

        // --- Separation by operators ---
        Object s1 = null + null; // '+' irrelevant semantically, lexer must tokenize correctly

        String s = "null in string";

    }
}