using System.IO;

namespace RustGen
{
    class Program
    {
        static void Main(string[] args)
        {
            if (!Directory.Exists("out")) Directory.CreateDirectory("out");

            var opcodes = new OpcodeParser().Parse(@"in\opcodes.txt");
            var generator = new OpcodeGenerator(@"out");
            generator.GenerateCodes(opcodes);
        }
    }
}
