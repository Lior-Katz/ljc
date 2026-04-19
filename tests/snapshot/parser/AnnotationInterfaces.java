@interface AnnotationInterfaces {
    int x();

    String y();

    int[] z();

    Class c();

    Other nested();

    Other[] nestedArray();

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