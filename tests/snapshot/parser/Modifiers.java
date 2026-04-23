@Modifiers.Marker
public abstract class Modifiers {

    @Single(1)
    public int a;
    protected static int b;
    private final int c = 1;

    @Single(5 + 10)
    public Modifiers(int a) {
        try {
        } catch (@Marker final Error | @Single(1) RuntimeException e) {
        }
    }

    @Normal(x = 2, y = @InnerAnno(a = {1, 2, 3}))
    public abstract void abstractMethod();

    protected static void staticMethod(@Marker final int x) {
        final int y = x + 1;
    }

    sealed
    @Normal(x = 1, y = @InnerAnno(a = {1,}))
    public static class Inner permits InnerChild {

        private int x;

        @Single(3)
        public final void method(final int... param) {
            @Normal(x = 4, y = @InnerAnno(a = {,}))
            final int[] local = param;
        }
    }

    non-sealed static class InnerChild extends Inner {
    }

    static public @interface Marker {
    }

    @Marker
    @interface Single {
        int value();
    }

    @interface Normal {
        int x();

        @Marker abstract InnerAnno y();
    }

    @interface InnerAnno {
        public int[] a();
    }

    @Marker
    protected enum E {
        @Single(1) A
    }

    @Marker
    private static record R(@Marker int a, @Marker int... b) {
        @Single(1)
        R {
        }
    }

    @Marker
    protected interface I {
        @Single(5)
        static final int X = 10;

        @Marker
        default void foo() {
        }
    }
}
