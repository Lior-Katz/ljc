@interface AnnotationInterfaces {
    int x() default 5;

    String y();

    int[] z() default {1, 2, };

    Class c();

    Other nested();

    Other[] nestedArray() default {@Other(3)};

    ;

    int CONST = 5;

    class InnerClass {
    }

    interface InnerInterface {
    }

    @interface Other {
        int value();
    }
}