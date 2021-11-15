using System;
using System.Runtime.InteropServices;

namespace BlazorMath;

[StructLayout(LayoutKind.Sequential)]
public struct ByteBuffer
{
    public long Length;
    public IntPtr Data;
}

public static class Native
{
    [DllImport("libmath")]
    public static extern int int_in_return(int x);

    [DllImport("libmath")]
    public static extern int int_in_out(int x, out int y);

    [DllImport("libmath")]
    public static extern int array_in(int in_len, IntPtr in_data);

    [DllImport("libmath")]
    public static extern void array_in_out(int in_len, IntPtr in_data, out int out_len, out IntPtr out_arr);

    [DllImport("libmath")]
    public static extern IntPtr array_in_return(int in_len, IntPtr in_data);

    [DllImport("libmath")]
    public static extern int buffer_in(ByteBuffer buffer);

    [DllImport("libmath")]
    [return: MarshalAs(UnmanagedType.LPStruct)]
    public static extern ByteBuffer buffer_in_return(ByteBuffer buffer);

    [DllImport("libmath")]
    public static extern void buffer_in_out(ByteBuffer buffer, out ByteBuffer out_buffer);

    public static ByteBuffer ToByteBuffer(byte[] array)
    {
        var pinnedArray = GCHandle.Alloc(array, GCHandleType.Pinned);
        var pointer = pinnedArray.AddrOfPinnedObject();

        return new ByteBuffer { Length = array.Length, Data = pointer };
    }

    public static byte[] FromByteBuffer(ByteBuffer buffer)
    {
        var array = new byte[buffer.Length];
        Marshal.Copy(buffer.Data, array, 0, (int)buffer.Length);

        return array;
    }
}