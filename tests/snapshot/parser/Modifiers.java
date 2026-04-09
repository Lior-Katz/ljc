public abstract class ModifierTest {
    public int a;
    protected static int b;
    private final int c = 1;

    public abstract void abstractMethod();

    protected static void staticMethod(int x) {
    }

    public static final class Inner {

        private int x;

        public final void method(int param) {
            int local = param;
        }
    }
}