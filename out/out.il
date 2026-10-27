
.assembly extern System.Runtime {}
.assembly extern System.Console {}

.assembly MyProgram {}

.class public auto ansi CompiledClass
    extends [System.Runtime]System.Object
{


    .method public static int32 Main() cil managed
    {
        .entrypoint
        .maxstack 1

        call int32 CompiledClass::main()

        call void [System.Console]System.Console::WriteLine(int32)

        ldc.i4.0
        ret
    }


    
    .method public static int32 fib(int32 n)
    {
        .maxstack 20

        ldarg.0
        ldc.i4 0
        ceq
        brfalse JUMP_0
        ldc.i4 0
        ret
        JUMP_0:
        ldarg.0
        ldc.i4 1
        ceq
        brfalse JUMP_1
        ldc.i4 1
        ret
        JUMP_1:
        ldarg.0
        ldc.i4 1
        sub
        call int32 CompiledClass::fib(int32)
        ldarg.0
        ldc.i4 2
        sub
        call int32 CompiledClass::fib(int32)
        add
        ret
    }

    .method public static int32 main()
    {
        .maxstack 20

        ldc.i4 12
        call int32 CompiledClass::fib(int32)
        ret
    }

}