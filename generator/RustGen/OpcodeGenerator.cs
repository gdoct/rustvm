using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Linq;

namespace RustGen
{
    public class OpcodeGenerator
    {
        const string HEADER = @"use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };";

        public OpcodeGenerator(string folder)
        {
            Folder = folder;
            ClearFolder(folder);
        }

        private void ClearFolder(string folder)
        {
            DirectoryInfo di = new DirectoryInfo(folder);
            FileInfo[] files = di.GetFiles();
            foreach (FileInfo file in files)
            {
                file.Delete();
            }
        }

        public string Folder { get; }

        public void GenerateCodes(IEnumerable<Opcode> codes)
        {
            var switchcases = new StringBuilder();
            var mods = new StringBuilder();
            var uses = new StringBuilder();
            foreach (var code in codes)
            {
                GenerateFile(code);
            }


            var allcodes = codes.OrderBy(c => c.Num).ToList();
            var previous = string.Empty;
            foreach(var code in allcodes.OrderBy(c => c.Code))
            {
                if (string.Compare(code.Code, previous, StringComparison.OrdinalIgnoreCase) != 0)
                {
                    switchcases.AppendLine($@"
        // {code.Code}");
                    previous = code.Code;
                }
                switchcases.AppendLine($"        0x{code.ToHex()} => {{ Ok(Box::new({code.ClassName} {{}})) }}");
            }

            foreach (var code in allcodes.Select(c => c.Code).Distinct().OrderBy(s => s))
            {
                var lower = code.ToLowerInvariant();
                mods.AppendLine($"mod {lower};"); ;
                uses.AppendLine($"use crate::instructions::{lower}::*;");
            }

            using (var instr = File.CreateText(Path.Combine(Folder, "instructions.rs")))
            {
                instr.WriteLine(mods);
                instr.WriteLine(@"use std::io::{ Error, ErrorKind };
use crate::types::{ Byte };
use crate::traits::{ Instruction };");
                instr.WriteLine(uses);
                instr.WriteLine(@"pub fn parse_opcode(opcode: Byte) -> std::result::Result<Box<dyn Instruction + 'static>, Error> {
    match opcode {");
                instr.WriteLine(switchcases);
                instr.WriteLine(@"
        _ => { Err(Error::new(ErrorKind::Other, format!(""opcode { } not implemented"", opcode))) }
    }
}");
            }
        }

        private void GenerateFile(Opcode code)
        {
            var filename = Path.Combine(Folder, code.Code.ToLower() + ".rs");
            var exists = File.Exists(filename);
            using (var file = File.AppendText(filename))
            {
                if (!exists)
                {
                    file.WriteLine(HEADER);
                }
                var hex = code.ToHex();
                var opcode = $"{code.Code.Substring(0,1).ToUpperInvariant()}{code.Code.Substring(1, 2).ToLowerInvariant()}";
                var method = @$"
/// {code.ClassName}: {code.Code} {code.AddressingModeNice}
pub struct {code.ClassName} {{ }}
impl Instruction for {code.ClassName} {{
    fn opcode (&self) -> &'static str  {{ ""{code.Code}""}}
    fn hexcode(&self) -> Byte {{ 0x{hex} }}
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {{
        panic!(""opcode {code.Code} ({opcode}) not implemented!"");
        // Ok(())
    }}
}}";
                file.WriteLine(method);
            }
        }
    }
}
