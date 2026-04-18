record R0(int x, String y) {
    static final int CONST = 10;

    public R0() {
        this(1);
    }

    R0(int x) {
        this(x, "default");
    }

    int value() {
        return x;
    }

    static int f() {
        return 1;
    }

    class InnerClass {
    }

    interface InnerInterface {
    }

    public static record InnerRecord1() {
        public InnerRecord1() {
        }
    }

    record InnerRecord2(int... xs) {
    }
}