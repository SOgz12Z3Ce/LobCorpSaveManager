using System;
using System.IO;
using LobCorp.Save;

namespace LobCorp
{
	internal class Program
	{
		static int Main(string[] args)
		{
			if (args.Length != 1)
			{
				Console.WriteLine("Usage is: lobcss file");
				return 1;
			}
			using (FileStream fileStream = File.Open(args[0], FileMode.Open))
			{
				LobCorpSave lobCorpSave = LobCorpSaveSerializer.Deserialize(fileStream);
				Console.WriteLine(lobCorpSave.ToJson(Newtonsoft.Json.Formatting.Indented));
			}
			return 0;
		}
	}
}
