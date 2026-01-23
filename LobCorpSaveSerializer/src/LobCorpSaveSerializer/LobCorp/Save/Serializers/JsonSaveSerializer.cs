using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using LobCorp.Exceptions;
using LobCorp.Save.Type;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Serializers
{
	public static class JsonSaveSerializer
	{
		private static readonly Dictionary<SaveType, string> defaultFileNameMap = new Dictionary<SaveType, string>()
		{
			{ SaveType.Options, "options.json" },
			{ SaveType.Etc, "etc.json" },
			{ SaveType.Master, "master.json" },
			{ SaveType.Global, "global.json" },
			{ SaveType.Unlimited, "unlimited.json" },
			{ SaveType.WhiteNight, "white_night.json" },
		};
		public static void Serialize(string path, SaveFile save)
		{
			using (var stream = File.Open(path, FileMode.CreateNew))
			using (var streamWriter = new StreamWriter(stream, new UTF8Encoding()))
			{
				var serializer = new JsonSerializer();
				serializer.Serialize(streamWriter, save.data);
			}
		}
		public static void Serialize(string path, SaveFile save, Formatting formatting)
		{
			using (var stream = File.Open(path, FileMode.CreateNew))
			using (var streamWriter = new StreamWriter(stream, new UTF8Encoding()))
			{
				var serializer = new JsonSerializer()
				{
					Formatting = formatting
				};
				serializer.Serialize(streamWriter, save.data);
			}
		}
		public static SaveFile Deserialize(string path)
		{
			JObject json;
			try
			{
				using (FileStream stream = File.Open(path, FileMode.Open))
				using (var streamReader = new StreamReader(stream, Encoding.UTF8))
				using (var jsonReader = new JsonTextReader(streamReader))
				{
					json = JObject.Load(jsonReader);
				}
			}
			catch (Exception e)
			{
				throw new BadSaveFile(string.Format("Unable to deserialize JSON file: {0}", path), e);
			}
			return new SaveFile(json);
		}
		public static string DefaultFileNameOf(SaveType type)
		{
			return defaultFileNameMap[type];
		}
	}
}
