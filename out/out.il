
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


    
    .method public static int32 func(int32 a, int32 b, int32 c, int32 d)
    {
        .maxstack 20

        ldarg.0
        ldarg.1
        ldarg.2
        add
        ldarg.3
        add
        add
        ret
    }

    .method public static int32 main()
    {
        .maxstack 20

        ldc.i4 1
        ldc.i4 1
        ldc.i4 1
        add
        ldc.i4 1
        ldc.i4 1
        call int32 CompiledClass::func(int32, int32, int32, int32)
        ldc.i4 1
        ldc.i4 1
        ldc.i4 1
        ldc.i4 1
        call int32 CompiledClass::func(int32, int32, int32, int32)
        add
        ret
    }

}