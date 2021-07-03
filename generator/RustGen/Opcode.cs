namespace RustGen
{
    public class Opcode
    {
        public string Code { get; set; }

        public short Num { get; set; }

        public string AddressingMode { get; set; }

        public string ToHex() => Num.ToString("X2");

        public string ToNamedCode() => $"{Code.Substring(0, 1).ToUpperInvariant()}{Code.Substring(1, 2).ToLowerInvariant()}";

        public string ClassName => $"{ToNamedCode()}{AddressingMode}";

        public string AddressingModeNice
        {
            get
            {
                switch (AddressingMode)
                {
                    case "Imm": return "immediate";
                    case "Abs": return "absolute";
                    case "AbsX": return "absolute, indexed by X";
                    case "AbsY": return "absolute, indexed by Y";
                    case "Zp": return "zeropage";
                    case "ZpX": return "absolute, indexed by X";
                    case "ZpY": return "absolute, indexed by X";
                    case "Rel": return "relative";
                    case "Ind": return "indirect";
                    case "IndX": return "indirect, indexed by X";
                    case "IndY": return "indirect, indexed by Y";
                    default: return "";
                }
            }
        }
    }
}
