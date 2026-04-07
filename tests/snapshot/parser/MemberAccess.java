class MemberAccess {
    void member_access() {
        a.b;
        a.b.c.d;

        (a).b;
        (a.b).c;

        a.b = -c.d;
        x.y.z = a.b.c + d.e;
        a.b.c *= d.e;
    }

    void method_invocation() {
            System.out.println("Hello, world!");
            System.currentTimeMillis();
            Math.max(1, Math.abs(-1));
            "hello".toUpperCase().trim();
            "test".substring(0, 2).toLowerCase();
            int x = Math.abs(-5) + Math.max(1, 2);
            int y = "abc".length() * Math.min(2, 3);
    }

    void array_access() {
    }
}