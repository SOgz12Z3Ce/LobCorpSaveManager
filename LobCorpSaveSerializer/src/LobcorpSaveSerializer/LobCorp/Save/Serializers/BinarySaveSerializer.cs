using System;
using System.CodeDom;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization.Formatters.Binary;
using LobCorp.Exceptions;
using LobCorp.Save.Type;

namespace LobCorp.Save.Serializers
{
	public static class BinarySaveSerializer
	{
		private static readonly Dictionary<SaveType, string> defaultFileNameMap = new Dictionary<SaveType, string>()
		{
			{ SaveType.Settings, "Lobotomy170808state.dat" },
			{ SaveType.Etc, "etc170808.dat" },
			{ SaveType.Master, "saveData170808.dat" },
			{ SaveType.Global, "saveGlobal170808.dat" },
			{ SaveType.WhiteNight, "100014.dat" },
		};
		public static void Serialize(string path, SaveFile save)
		{
			var binaryFormatter = new BinaryFormatter();
			using (var stream = File.Open(path, FileMode.CreateNew))
			{
				binaryFormatter.Serialize(stream, save.data);
			}
		}
		public static SaveFile Deserialize(string path)
		{
			var binaryFormatter = new BinaryFormatter();
			Dictionary<string, object> save;
			using (var stream = File.Open(path, FileMode.Open))
			{
				try
				{
					save = binaryFormatter.Deserialize(stream) as Dictionary<string, object>;
				}
				catch (Exception e)
				{
					throw new BadSaveFile(string.Format("Unable to deserialize binary file: {0}", path), e);
				}
			}
			return new SaveFile(save);
		}
		public static string DefaultFileNameOf(SaveType type)
		{
			return defaultFileNameMap[type];
		}
	}
}
