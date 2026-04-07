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
    }

    void array_access() {
    }
}