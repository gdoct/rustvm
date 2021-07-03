using System.Collections.Generic;
using System.IO;

namespace RustGen
{
    class OpcodeParser
    {
        public IEnumerable<Opcode> Parse(string filename)
        {
            var lines = File.ReadAllLines(filename);
            foreach(var line in lines)
            {
                var dollar = line.IndexOf("$");
                if (dollar < 0) continue;
                var sub = line.Substring(0, dollar - 1).Trim().Split(' ');
                if (sub.Length < 3 || sub[1].StartsWith("*")) continue;
                var addressingmode = ParseAddressingmode(sub[2]);
                yield return new Opcode { Num = short.Parse(sub[0], System.Globalization.NumberStyles.HexNumber), Code = sub[1], AddressingMode = addressingmode };
            }
        }

        private string ParseAddressingmode(string shortmode)
        {
            switch(shortmode)
            {
                case "imm": return "Imm";
                case "abs": return "Abs";
                case "abx": return "AbsX";
                case "aby": return "AbsY";
                case "zp": return "Zp";
                case "zpx": return "ZpX";
                case "zpy": return "ZpY";
                case "rel": return "Rel";
                case "ind": return "Ind";
                case "izx": return "IndX";
                case "izy": return "IndY";
                default: return "";
            }
        }
    }
}
