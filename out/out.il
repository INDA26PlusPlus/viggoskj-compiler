
.assembly extern System.Runtime {}
.assembly extern System.Console {}

.assembly MyProgram {}

.class public auto ansi CompiledClass
    extends [System.Runtime]System.Object
{
    .method public static int32 Main() cil managed
    {
        .entrypoint
        .maxstack 4

        ldc.i4.3
        ldc.i4.5
        ldc.i4.5
        ldc.i4.5
        call int32 CompiledClass::func(int32, int32, int32, int32)

        call void [System.Console]System.Console::WriteLine(int32)

        ldc.i4.0
        ret
    }

    .method public static int32 Add(int32 a, int32 b) cil managed
    {
        .maxstack 2

        ldarg.0
        ldarg.1
        add
        ret
    }

    .method public static int32 func(int32 a, int32 b, int32 c, int32 d) cil managed
    {
        .maxstack 20

        ldc.i4.3
        ldarg.3
        add
        ldarg.2
        add
        ldarg.1
        ldarg.0
        add
        ldc.i4.3
        add
        add

        ret
    }
}
