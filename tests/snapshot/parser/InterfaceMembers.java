interface InterfaceMembers {
    int A = 1, B = 2;
    public static final int C = 3;

    private static void m1() {
    }

    public abstract int m2(int x);

    default int m3(int x, int y) {
        return 0;
    }

    ;;

    class InnerClass {
        int x;

        void foo() {
        }
    }

    interface InnerInterface {
        int X = 10;

        void innerMethod();
    }
}