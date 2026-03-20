@interface MyAnnotation {}

class SeparatorsTest {

    int[] arr = new int[10];

    void test() {

        // --- Parentheses ---
        ( );
        (()());

        // --- Braces ---
        { }
        {{}}

        // --- Brackets ---
        int[][] matrix = new int[3][4];
        arr[0] = 1;

        // --- Semicolon ---
        int a = 0;;

        // --- Comma ---
        int x = 1, y = 2, z = 3;

        // --- Dot ---
        String s = "abc".toString();

        // --- Ellipsis ---
        methodVarArgs(1, 2, 3);

        // --- Annotation (@) ---
        @MyAnnotation
        int annotatedField;

        // --- Method reference (::) ---
        Runnable r = this::test;

        // --- Mixed / adjacency cases ---
        int v = arr[0];
        methodCall(a,b,c);
        methodCall((a),(b));
        obj.method().field;
        obj::toString;

        // --- Chained separators ---
        (((a)));
        arr[0][1][2];
    }

    void methodCall(int a, int b, int c) {}

    void methodVarArgs(int... nums) {}
}