using System;
using System.IO;
using LobCorp.Save;

namespace LobCorp
{
	internal class Program
	{
		static void Main(string[] args)
		{
			FileStream fileStream = File.Open(args[0], FileMode.Open);
			LobCorpSave lobCorpSave = LobCorpSaveSerializer.Deserialize(fileStream);
			Console.WriteLine(lobCorpSave.ToJson());
		}
	}
}
