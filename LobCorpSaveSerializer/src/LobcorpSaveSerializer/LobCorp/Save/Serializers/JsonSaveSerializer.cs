using System;
using System.IO;
using System.Runtime.Remoting.Lifetime;
using System.Text;
using LobCorp.Exceptions;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Serializers
{
	public static class JsonSaveSerializer
	{
		public static void Serialize(string path, SaveFile save)
		{
			using (var stream = File.Open(path, FileMode.CreateNew))
			using (var streamWriter = new StreamWriter(stream, new UTF8Encoding()))
			{
				var serializer = new JsonSerializer();
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
	}
}
