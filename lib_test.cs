using System;
using System.Runtime.InteropServices;

class Scissor
{
    // ScissorDimension has 4 length
    // a : length of the element right up to the right
    // b : length of the element right up to the left
    // c : distance from a origin to cross point of a and b
    // d : distance from b origin to cross point of a and b
    [StructLayout(LayoutKind.Sequential)]
    public struct ScissorDimension
    {
        public double a;
        public double b;
        public double c;
        public double d;
    }
    [StructLayout(LayoutKind.Sequential)]
    public struct line
    {
        public double x1;
        public double y1;
        public double x2;
        public double y2;
    }
    [StructLayout(LayoutKind.Sequential)]
    private struct solve_scissor_return
    {
        public long error;
        public ulong num_lines;
        public IntPtr lines;
    }
    [DllImport("C:/Users/Nakanishi/Documents/linkage_sim/target/release/linkage_sim.dll")]
    private static extern IntPtr create_scissor_dimension_array(ulong size);
    [DllImport("C:/Users/Nakanishi/Documents/linkage_sim/target/release/linkage_sim.dll")]
    private static extern solve_scissor_return solve_from_scissor_dimension_array(IntPtr array, ulong len, double input_radius, double input_theta);

    public static ScissorDimension[] create_scissor(ulong size)
    {
        IntPtr ptr = create_scissor_dimension_array(size);
        ScissorDimension[] scissor_dimension = new ScissorDimension[size];
        for (ulong i = 0; i < size; i++)
        {
            scissor_dimension[i] = (ScissorDimension)Marshal.PtrToStructure(ptr, typeof(ScissorDimension));
            ptr += Marshal.SizeOf(typeof(ScissorDimension));
        }
        return scissor_dimension;
    }

    public struct solve_return
    {
        public long error;
        public line[] lines;
    }

    public static solve_return solve_from_scissor(ScissorDimension[] scissor_dimension, double input_radius, double input_theta)
    {
        Console.WriteLine("scissor_dimension: " + scissor_dimension.Length);
        IntPtr ptr = create_scissor_dimension_array((ulong)scissor_dimension.Length);
        IntPtr ptr_inc = ptr;
        for (ulong i = 0; (int)i < scissor_dimension.Length; i++)
        {
            Marshal.StructureToPtr(scissor_dimension[i], ptr_inc, false);
            ptr_inc += Marshal.SizeOf(typeof(ScissorDimension));
        }
        solve_scissor_return ret = solve_from_scissor_dimension_array(ptr, (ulong)scissor_dimension.Length, input_radius, input_theta);
        line[] lines = new line[ret.num_lines];
        ptr = ret.lines;
        for (ulong i = 0; i < ret.num_lines; i++)
        {
            lines[i] = (line)Marshal.PtrToStructure(ptr, typeof(line));
            ptr += Marshal.SizeOf(typeof(line));
        }
        return new solve_return { error = ret.error, lines = lines };
    }
}
class test
{
    // // ScissorDimension has 4 length
    // // a : length of the element right up to the right
    // // b : length of the element right up to the left
    // // c : distance from a origin to cross point of a and b
    // // d : distance from b origin to cross point of a and b
    // [StructLayout(LayoutKind.Sequential)]
    // private struct ScissorDimension
    // {
    //     public double a;
    //     public double b;
    //     public double c;
    //     public double d;
    // }
    // [StructLayout(LayoutKind.Sequential)]
    // private struct line
    // {
    //     public double x1;
    //     public double y1;
    //     public double x2;
    //     public double y2;
    // }
    // [StructLayout(LayoutKind.Sequential)]
    // public struct solve_scissor_return
    // {
    //     public long error;
    //     public ulong num_lines;
    //     public IntPtr lines;
    // }

    // [DllImport("C:/Users/Nakanishi/Documents/linkage_sim/target/release/linkage_sim.dll")]
    // public static extern int add(int a, int b);
    // [DllImport("C:/Users/Nakanishi/Documents/linkage_sim/target/release/linkage_sim.dll")]
    // public static extern IntPtr create_scissor_dimension_array(ulong size);
    // [DllImport("C:/Users/Nakanishi/Documents/linkage_sim/target/release/linkage_sim.dll")]
    // public static extern solve_scissor_return solve_from_scissor_dimension_array(IntPtr array, ulong len, double input_radius, double input_theta);

    // private static ScissorDimension[] create_scissor(ulong size)
    // {
    //     IntPtr ptr = create_scissor_dimension_array(size);
    //     ScissorDimension[] scissor_dimension = new ScissorDimension[size];
    //     for (ulong i = 0; i < size; i++)
    //     {
    //         scissor_dimension[i] = (ScissorDimension)Marshal.PtrToStructure(ptr, typeof(ScissorDimension));
    //         ptr += Marshal.SizeOf(typeof(ScissorDimension));
    //     }
    //     return scissor_dimension;
    // }

    // private struct solve_return
    // {
    //     public long error;
    //     public line[] lines;
    // }

    // private static solve_return solve_from_scissor(ScissorDimension[] scissor_dimension, double input_radius, double input_theta)
    // {
    //     Console.WriteLine("scissor_dimension: " + scissor_dimension.Length);
    //     IntPtr ptr = create_scissor_dimension_array((ulong)scissor_dimension.Length);
    //     IntPtr ptr_inc = ptr;
    //     for (ulong i = 0; (int)i < scissor_dimension.Length; i++)
    //     {
    //         Marshal.StructureToPtr(scissor_dimension[i], ptr_inc, false);
    //         ptr_inc += Marshal.SizeOf(typeof(ScissorDimension));
    //     }
    //     solve_scissor_return ret = solve_from_scissor_dimension_array(ptr, (ulong)scissor_dimension.Length, input_radius, input_theta);
    //     line[] lines = new line[ret.num_lines];
    //     ptr = ret.lines;
    //     for (ulong i = 0; i < ret.num_lines; i++)
    //     {
    //         lines[i] = (line)Marshal.PtrToStructure(ptr, typeof(line));
    //         ptr += Marshal.SizeOf(typeof(line));
    //     }
    //     return new solve_return { error = ret.error, lines = lines };
    // }

    public static void Main()
    {
        Scissor.ScissorDimension[] scissor_dimensions = Scissor.create_scissor(10);
        for (ulong i = 0; i < 10; i++)
        {
            Console.WriteLine("a: " + scissor_dimensions[i].a + ", b: " + scissor_dimensions[i].b + ", c: " + scissor_dimensions[i].c + ", d: " + scissor_dimensions[i].d);
        }
        Scissor.solve_return ret = Scissor.solve_from_scissor(scissor_dimensions, 0.8, 0);
        Console.WriteLine("error: " + ret.error);
    }
}
