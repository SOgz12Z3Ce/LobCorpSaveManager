using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.Serialization.Formatters.Binary;
using LobCorp.Exceptions;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save
{
	public class LobCorpSaveSerializer
	{
		private static readonly Dictionary<FileType, IDeserializer> deserializerMap = new Dictionary<FileType, IDeserializer>()
		{
			{ FileType.Binary, new BinaryDeserializer()},
			{ FileType.Json, new JsonDeserializer()},
		};
		public static void Serialize(FileStream stream, LobCorpSaveSegment saveSeg)
		{
			var binaryFormatter = new BinaryFormatter();
			binaryFormatter.Serialize(stream, saveSeg.data);
		}
		public static LobCorpSaveSegment Deserialize(FileStream stream, FileType type)
		{
			return deserializerMap[type].Deserialize(stream);
		}
		public enum FileType
		{
			Binary,
			Json,
		}
		private interface IDeserializer
		{
			LobCorpSaveSegment Deserialize(FileStream stream);
		}
		private class BinaryDeserializer : IDeserializer
		{
			public LobCorpSaveSegment Deserialize(FileStream stream)
			{
				var binaryFormatter = new BinaryFormatter();
				Dictionary<string, object> save;
				try
				{
					save = binaryFormatter.Deserialize(stream) as Dictionary<string, object>;
				}
				catch (Exception e)
				{
					throw new BadSaveFile(string.Format("Unable to deserialize binary file: {0}", stream.Name), e);
				}
				return new LobCorpSaveSegment(save);
			}
		}
		private class JsonDeserializer : IDeserializer
		{
			public LobCorpSaveSegment Deserialize(FileStream stream)
			{
				JObject json;
				try
				{
					using (var streamReader = new StreamReader(stream))
					{
						using (var jsonReader = new JsonTextReader(streamReader))
						{
							json = JToken.Load(jsonReader) as JObject;
						}
					}
				}
				catch (Exception e)
				{
					throw new BadSaveFile(string.Format("Unable to deserialize json file: {0}", stream.Name), e);
				}
				return new LobCorpSaveSegment(json);
			}
		}
	}
}
