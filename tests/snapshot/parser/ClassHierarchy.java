sealed class ClassHierarchy extends Base implements I1, I2 permits A, B {
}

class Base {
}

interface I1 {
}

interface I2 {
}

interface I3 extends I1, I2 {
}

final class A extends ClassHierarchy {
}

non-sealed class B extends ClassHierarchy {
}

class OnlyExtends extends Base {
}

class OnlyImplements implements I1, I2 {
}

sealed class OnlyPermits permits C {
}

final class C extends OnlyPermits {
}

class ExtendsImplements extends Base implements I1 {
}

record R() implements I1, I2 {
}

enum E implements I1, I2 {
}
