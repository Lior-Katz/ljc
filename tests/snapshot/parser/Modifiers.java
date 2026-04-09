public abstract class ModifierTest {
    public int a;
    protected static int b;
    private final int c = 1;

    public abstract void abstractMethod();

    protected static void staticMethod(final int x) {
        final int y = x + 1;
    }

    public static final class Inner {

        private int x;

        public final void method(final int param) {
            final int local = param;
        }
    }
}