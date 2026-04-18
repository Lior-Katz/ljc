class ClassMembers {
    private int a;
    protected static int b = 1, c;


    ClassMembers() {}
    public ClassMembers(int x) {
    }

    // epilogue only
    protected ClassMembers(int x, int y) {
        int z = x + y;
    }

    // Alternate constructor call only
    ClassMembers(int x) {
        this(1, 2);
    }

    // Alternate constructor call + epilogue
    ClassMembers(double d) {
        this(2);
        int x = 0;
    }

    // Full form: prologue + alternate call + epilogue
    ClassMembers(boolean flag) {
        int x = 10;
        this(1 + 1);
        x = 5;
    }

    ;; // should be discarded
    public ClassMembers foo() {}; // should not be confused with constructor
    protected static int bar(int x) {}
    abstract void referenceTypeParam(java.lang.String s);

    public static class InnerClass {
        int x;

        void innerMethod() {}
    }

    interface InnerInterface {
        int X = 10;

        void innerMethod();
    }
}
